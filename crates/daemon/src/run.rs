use std::{path::PathBuf, process::Stdio, sync::Arc};

use color_eyre::{eyre::eyre, Result};
use serde::de::DeserializeOwned;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{ChildStdin, Command},
    sync::{broadcast, Mutex},
};

use super::io::{
    event::{
        AppDebugPortEventParams, AppLogEventParams, AppProgressEventParams, AppStartEventParams,
        AppStartedEventParams, AppStopEventParams, ConnectedEventParams, FlutterDaemonEvent,
        LogEventParams, LogMessageEventParams,
    },
    parse_event, parse_response,
    request::{DetachAppParams, FlutterDaemonRequest, RestartAppParams, StopAppParams},
    response::{
        DetachAppResponse, FlutterDaemonResponse, RestartAppResponse, RestartAppResult,
        ShutdownResponse, StopAppResponse, VersionResponse,
    },
};

pub struct FlutterRun {
    app_id: Arc<Mutex<Option<String>>>,
    tx: broadcast::Sender<String>,
    error_tx: broadcast::Sender<String>,
    // FIXME(itome): This is a workaround to keep the receiver alive.
    // If the receiver is dropped, tx.send will return an SendError.
    // So we need to keep the receiver alive.
    _rx: broadcast::Receiver<String>,
    _error_rx: broadcast::Receiver<String>,

    stdin: Arc<Mutex<ChildStdin>>,
    request_count: Arc<Mutex<u32>>,
    _process: tokio::process::Child,
}

impl FlutterRun {
    pub fn new(
        project_root: PathBuf,
        device_id: Option<String>,
        program: Option<String>,
        flutter_mode: Option<String>,
        cwd: Option<String>,
        args: Option<Vec<String>>,
        use_fvm: bool,
    ) -> Result<Self> {
        let mut arguments = vec!["run".to_string(), "--machine".to_string()];
        if let Some(device_id) = device_id {
            arguments.push("-d".to_string());
            arguments.push(device_id);
        }
        if let Some(program) = program {
            arguments.push("-t".to_string());
            arguments.push(program);
        }
        if let Some(flutter_mode) = flutter_mode {
            arguments.push(format!("--{}", flutter_mode));
        }
        if let Some(args) = args {
            arguments.extend(args);
        }
        let mut command = if use_fvm {
            Command::new("fvm")
        } else {
            Command::new("flutter")
        };
        if use_fvm {
            command.arg("flutter");
        }
        let mut process = command
            .args(arguments)
            .kill_on_drop(true)
            .current_dir(project_root.join(cwd.unwrap_or_default()))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = process
            .stdout
            .take()
            .ok_or(eyre!("Stdout is not available"))?;
        let stderr = process
            .stderr
            .take()
            .ok_or(eyre!("Stderr is not available"))?;

        let app_id = Arc::new(Mutex::new(None::<String>));
        let (tx, _rx) = broadcast::channel::<String>(16);
        let (error_tx, _error_rx) = broadcast::channel::<String>(16);

        let _tx = tx.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = _tx.send(line);
            }
        });

        let _error_tx = error_tx.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = _error_tx.send(line);
            }
        });

        let mut rx = tx.subscribe();
        let _app_id = app_id.clone();
        tokio::spawn(async move {
            while let Ok(line) = rx.recv().await {
                if let Some(FlutterDaemonEvent::AppStart { params }) = parse_event(&line) {
                    let mut app_id = _app_id.lock().await;
                    *app_id = Some(params.app_id.clone());
                    break;
                }
            }
        });

        Ok(Self {
            app_id,
            stdin: Arc::new(Mutex::new(process.stdin.take().unwrap())),
            tx,
            error_tx,
            _rx,
            _error_rx,
            request_count: Arc::new(Mutex::new(0)),
            _process: process,
        })
    }

    pub async fn version(&self) -> Result<String> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::Version { id: request_id };
        self.send_request(&request).await?;
        let result: VersionResponse = self.receive_response(request_id).await?;
        result.result.ok_or(eyre!("Could not get daemon version"))
    }

    pub async fn shutdown(&self) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::Shutdown { id: request_id };
        self.send_request(&request).await?;
        let _: ShutdownResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn hot_reload(&self) -> Result<RestartAppResult> {
        self.restart(false).await
    }

    pub async fn hot_restart(&self) -> Result<RestartAppResult> {
        self.restart(true).await
    }

    pub async fn detach(&self) -> Result<()> {
        let request_id = self.request_id().await;
        let app_id = self
            .app_id
            .lock()
            .await
            .clone()
            .ok_or(eyre!("App id is not set"))?;
        let request = FlutterDaemonRequest::DetachApp {
            id: request_id,
            params: DetachAppParams { app_id },
        };
        self.send_request(&request).await?;
        let response: DetachAppResponse = self.receive_response(request_id).await?;
        match response.result {
            Some(true) => Ok(()),
            _ => Err(eyre!("Could not detach app")),
        }
    }

    pub async fn stop(&self) -> Result<()> {
        let request_id = self.request_id().await;
        let app_id = self
            .app_id
            .lock()
            .await
            .clone()
            .ok_or(eyre!("App id is not set"))?;
        let request = FlutterDaemonRequest::StopApp {
            id: request_id,
            params: StopAppParams { app_id },
        };
        self.send_request(&request).await?;
        let response: StopAppResponse = self.receive_response(request_id).await?;
        match response.result {
            Some(true) => Ok(()),
            _ => Err(eyre!("Could not stop app")),
        }
    }

    pub async fn receive_daemon_connected(&self) -> Result<ConnectedEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::Connected { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_log(&self) -> Result<LogEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::Log { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_log_message(&self) -> Result<LogMessageEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::LogMessage { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_app_start(&self) -> Result<AppStartEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::AppStart { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_app_started(&self) -> Result<AppStartedEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::AppStarted { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_app_debug_port(&self) -> Result<AppDebugPortEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::AppDebugPort { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_app_log(&self) -> Result<AppLogEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::AppLog { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_app_progress(&self) -> Result<AppProgressEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::AppProgress { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_app_stop(&self) -> Result<AppStopEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::AppStop { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_stdout(&self) -> Result<String> {
        let mut rx = self.tx.subscribe();
        while let Ok(line) = rx.recv().await {
            if !(line.starts_with("[{") && line.ends_with("}]")) {
                return Ok(line);
            }
        }
        Err(eyre!("Could not receive daemon response"))
    }

    pub async fn receive_stderr(&self) -> Result<String> {
        let mut rx = self.error_tx.subscribe();
        loop {
            if let Ok(line) = rx.recv().await {
                return Ok(line);
            }
        }
    }

    async fn restart(&self, full_restart: bool) -> Result<RestartAppResult> {
        let request_id = self.request_id().await;
        let app_id = self
            .app_id
            .lock()
            .await
            .clone()
            .ok_or(eyre!("App id is not set"))?;
        let request = FlutterDaemonRequest::RestartApp {
            id: request_id,
            params: RestartAppParams {
                app_id,
                full_restart,
                pause: false,
                reason: None,
                debounce: None,
            },
        };
        self.send_request(&request).await?;
        let result: RestartAppResponse = self.receive_response(request_id).await?;
        result.result.ok_or(eyre!("Could not restart app"))
    }

    async fn request_id(&self) -> u32 {
        let mut request_count = self.request_count.lock().await;
        *request_count += 1;
        *request_count
    }

    async fn send_request(&self, request: &FlutterDaemonRequest) -> Result<()> {
        let message = serde_json::to_string(request)?;
        let message = format!("[{}]\n", message);
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(message.as_bytes()).await?;
        Ok(())
    }

    async fn receive_response<T>(&self, request_id: u32) -> Result<FlutterDaemonResponse<T>>
    where
        T: DeserializeOwned,
    {
        let mut rx = self.tx.subscribe();
        while let Ok(line) = rx.recv().await {
            let response: Option<FlutterDaemonResponse<T>> = parse_response(&line, request_id);
            if let Some(res) = response {
                return Ok(res);
            }
        }
        Err(eyre!("Could not receive daemon response"))
    }

    async fn receive_event(&self) -> Result<FlutterDaemonEvent> {
        let mut rx = self.tx.subscribe();
        while let Ok(line) = rx.recv().await {
            if let Some(res) = parse_event(&line) {
                return Ok(res);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }
}
