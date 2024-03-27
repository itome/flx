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
            .select(|state: &State| {
                let selected_device = state.select_device_popup.selected_device.clone()?;
                Some(selected_device.id)
            })
            .await;

        let flavor = store
            .select(|state: &State| {
                let selected_flavor = state.select_flavor_popup.selected_flavor.clone()?;
                if selected_flavor == *"Undefined" {
                    return None;
                }
                Some(selected_flavor.clone())
            })
            .await;

        let Ok(id) = self
            .context
            .session_manager
            .run_new_app(device_id.clone(), flavor.clone(), self.use_fvm)
            .await
        else {
            return;
        };

        store
            .dispatch(Action::RegisterSession {
                session_id: id.clone(),
                device_id,
                flavor,
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
                        .dispatch(Action::UnregisterSession {
                            session_id: id.clone(),
                        })
                        .await;
                    break;
                }
            }
        }
    }
}
