use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{
    middlewares::thunk::{self, Thunk},
    StoreApi,
};

use crate::redux::{
    action::Action,
    state::State,
    thunk::{run_new_vm_service::RunNewVmServiceThunk, thunk_impl, ThunkAction},
};

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct RunNewAppThunk {
    context: Arc<Context>,
    use_fvm: bool,
}

impl RunNewAppThunk {
    pub fn new(context: Arc<Context>, use_fvm: bool) -> Self {
        Self { context, use_fvm }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for RunNewAppThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let device_id = store
            .select(|state: &State| state.select_device_popup.selected_device_id.clone())
            .await;

        let configuration = store
            .select(|state: &State| {
                let index = state.select_launch_configuration_poopup.selected_index?;
                if index >= state.launch_configurations.len() {
                    return None;
                }
                state.launch_configurations.get(index).cloned()
            })
            .await;

        let Ok(id) = self
            .context
            .session_manager
            .run_new_app(
                device_id.clone(),
                configuration.clone().and_then(|c| c.program.clone()),
                configuration.clone().and_then(|c| c.flutter_mode.clone()),
                configuration.clone().and_then(|c| c.cwd.clone()),
                configuration.clone().and_then(|c| c.args.clone()),
                self.use_fvm,
            )
            .await
        else {
            return;
        };

        store
            .dispatch(Action::RegisterSession {
                session_id: id.clone(),
                device_id,
                configuration,
            })
            .await;

        let Ok(session) = self
            .context
            .clone()
            .session_manager
            .session(id.clone())
            .await
        else {
            return;
        };
        let session = session.read().await;
        let run = &session.as_ref().unwrap().run;

        if let Ok(params) = run.receive_app_start().await {
            store
                .dispatch(Action::StartApp {
                    session_id: id.clone(),
                    device_id: params.device_id,
                    app_id: params.app_id,
                    mode: params.mode,
                })
                .await;
        }

        loop {
            tokio::select! {
                Ok(_) = run.receive_app_started() => {
                    store
                        .dispatch(Action::SetAppStarted {
                            session_id: id.clone(),
                        })
                        .await;
                },
                Ok(progress) = run.receive_app_progress() => {
                    store
                        .dispatch(Action::AppendProgressLog {
                            session_id: id.clone(),
                            id: progress.id,
                            finished: progress.finished,
                            message: progress.message,
                        })
                        .await;
                },
                Ok(params) = run.receive_app_debug_port() => {
                    let store = store.clone();
                    let id = id.clone();
                    let context = self.context.clone();
                    let uri = params.ws_uri.clone();
                    tokio::spawn(async move {
                        RunNewVmServiceThunk::new(context, id, uri).execute(store).await;
                    });
                },
                Ok(line) = run.receive_stdout() => {
                    store
                        .dispatch(Action::AppendStdoutLog {
                            session_id: id.clone(),
                            line,
                        })
                        .await;
                },
                Ok(line) = run.receive_stderr() => {
                    store
                        .dispatch(Action::AppendStderrLog {
                            session_id: id.clone(),
                            line,
                        })
                        .await;
                },
                _ = run.receive_app_stop() => {
                    store
                        .dispatch(Action::StopSession {
                            session_id: id.clone(),
                        })
                        .await;
                    if let Err(e) = self
                        .context
                        .session_manager
                        .remove_session(id.clone())
                        .await
                    {
                        log::error!("Failed to remove session: {:?}", e);
                    }
                    break;
                }
            }
        }
    }
}
