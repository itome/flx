use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    daemon::flutter::FlutterDaemon,
    redux::{action::Action, state::State},
};

use super::context::Context;

pub struct RunNewAppThunk {
    context: Arc<Context>,
}

impl RunNewAppThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
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

        let Ok(id) = self
            .context
            .session_manager
            .run_new_app(device_id.clone())
            .await
        else {
            return;
        };

        store
            .dispatch(Action::RegisterSession {
                session_id: id.clone(),
                device_id,
            })
            .await;

        let Ok(session) = self.context.session_manager.session(id.clone()).await else {
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
                Ok(line) = run.receive_stdout() => {
                    store
                        .dispatch(Action::AppendStdoutLog {
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
