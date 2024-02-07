use std::{process::Stdio, sync::Arc};

use color_eyre::{eyre::eyre, Result};
use serde::{de::DeserializeOwned, Deserialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::{broadcast, mpsc, Mutex},
    task::JoinHandle,
};

use super::io::{
    device::Device,
    emulator::Emulator,
    event::{
        ConnectedEventParams, FlutterDaemonEvent, LogEventParams, LogMessageEventParams,
        ShowMessageEventParams,
    },
    parse_event, parse_response,
    request::{
        CreateEmultorParams, DeviceForwardParams, DeviceUnforwardParams, FlutterDaemonRequest,
        GetSupportedPlatformsParams, LaunchEmulatorParams,
    },
    response::{
        DeviceDisableResponse, DeviceEnableResponse, DeviceForwardResponse,
        DeviceUnforwardResponse, EmulatorCreateResponse, EmulatorLaunchResponse,
        FlutterDaemonResponse, GetDevicesResponse, GetEmulatorsResponse,
        GetSupportedPlatformsResponse, ServeDevToolsResponse, ServeDevToolsResult,
        ShutdownResponse, VersionResponse,
    },
};

pub struct FlutterDaemon {
    tx: broadcast::Sender<String>,
    stdin: Arc<Mutex<ChildStdin>>,
    request_count: Arc<Mutex<u32>>,
    _process: tokio::process::Child,
}

impl FlutterDaemon {
    pub fn new() -> Result<Self> {
        let mut process = Command::new("flutter")
            .arg("daemon")
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = process
            .stdout
            .take()
            .ok_or(eyre!("Stdout is not available"))?;

        let (tx, _) = broadcast::channel::<String>(16);

        let _tx = tx.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = _tx.send(line);
            }
        });

        Ok(Self {
            stdin: Arc::new(Mutex::new(process.stdin.take().unwrap())),
            tx,
            _process: process,
            request_count: Arc::new(Mutex::new(0)),
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

    pub async fn get_supported_platforms(&self, project_root: String) -> Result<Vec<String>> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::GetSupportedPlatforms {
            id: request_id,
            params: GetSupportedPlatformsParams { project_root },
        };
        self.send_request(&request).await?;
        let result: GetSupportedPlatformsResponse = self.receive_response(request_id).await?;
        result
            .result
            .map(|result| result.platforms)
            .ok_or(eyre!("Could not get supported platforms"))
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::GetDevices { id: request_id };
        self.send_request(&request).await?;
        let result: GetDevicesResponse = self.receive_response(request_id).await?;
        result.result.ok_or(eyre!("Could not get devices"))
    }

    pub async fn enable_device(&self) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::DeviceEnable { id: request_id };
        self.send_request(&request).await?;
        let _: DeviceEnableResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn disable_device(&self) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::DeviceDisable { id: request_id };
        self.send_request(&request).await?;
        let _: DeviceDisableResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn forward_device(
        &self,
        device_id: String,
        port: u32,
        host_port: Option<u32>,
    ) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::DeviceForward {
            id: request_id,
            params: DeviceForwardParams {
                device_id,
                port,
                host_port,
            },
        };
        self.send_request(&request).await?;
        let _: DeviceForwardResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn unforward_device(
        &self,
        device_id: String,
        port: u32,
        host_port: u32,
    ) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::DeviceUnforward {
            id: request_id,
            params: DeviceUnforwardParams {
                device_id,
                port,
                host_port,
            },
        };
        self.send_request(&request).await?;
        let _: DeviceUnforwardResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn get_emulators(&self) -> Result<Vec<Emulator>> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::GetEmulators { id: request_id };
        self.send_request(&request).await?;
        let result: GetEmulatorsResponse = self.receive_response(request_id).await?;
        result.result.ok_or(eyre!("Could not get devices"))
    }

    pub async fn launch_emulator(&self, emulator_id: String, cold_boot: bool) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::LaunchEmulator {
            id: request_id,
            params: LaunchEmulatorParams {
                emulator_id,
                cold_boot,
            },
        };
        self.send_request(&request).await?;
        let _: EmulatorLaunchResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn create_emulator(&self, name: Option<String>) -> Result<()> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::CreateEmulator {
            id: request_id,
            params: CreateEmultorParams { name },
        };
        self.send_request(&request).await?;
        let _: EmulatorCreateResponse = self.receive_response(request_id).await?;
        Ok(())
    }

    pub async fn serve_devtools(&self) -> Result<ServeDevToolsResult> {
        let request_id = self.request_id().await;
        let request = FlutterDaemonRequest::ServeDevtools { id: request_id };
        self.send_request(&request).await?;
        let response: ServeDevToolsResponse = self.receive_response(request_id).await?;
        response.result.ok_or(eyre!("Could not get devtools info"))
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

    pub async fn receive_show_message(&self) -> Result<ShowMessageEventParams> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::ShowMessage { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_device_added(&self) -> Result<Device> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::DeviceAdded { params } = event {
                return Ok(params);
            }
        }
        Err(eyre!("Could not receive daemon event"))
    }

    pub async fn receive_device_removed(&self) -> Result<Device> {
        while let Ok(event) = self.receive_event().await {
            if let FlutterDaemonEvent::DeviceRemoved { params } = event {
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

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use color_eyre::Result;
    use tokio::time::sleep;

    use crate::daemon::{
        flutter::FlutterDaemon,
        io::event::{ConnectedEventParams, LogMessageEventParams, MessageLevel},
    };

    #[tokio::test]
    async fn daemon_start() {
        let daemon = FlutterDaemon::new().unwrap();
        for i in 0..3 {
            let version = daemon.version().await.unwrap();
            assert_eq!(version, "0.6.1".to_string());
        }
        assert!(daemon.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn receive_daemon_connected() {
        let daemon = FlutterDaemon::new().unwrap();
        let event = daemon.receive_daemon_connected().await.unwrap();
        assert_eq!(event.version, "0.6.1");
        assert!(daemon.shutdown().await.is_ok());
    }
}
