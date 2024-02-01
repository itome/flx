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
        let Ok(id) = self.context.session_manager.write().await.run_new_app() else {
            return;
        };

        store
            .dispatch(Action::RegisterSession {
                session_id: id.clone(),
            })
            .await;

        let session_manager = self.context.session_manager.read().await;
        let run = &session_manager.sessions.get(&id).unwrap().run;

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

        if let Ok(_) = run.receive_app_started().await {
            store
                .dispatch(Action::StartHotRestart {
                    session_id: id.clone(),
                })
                .await;
        }

        run.receive_app_stop().await.unwrap();

        store
            .dispatch(Action::UnregisterSession {
                session_id: id.clone(),
            })
            .await;
    }
}
