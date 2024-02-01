use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    daemon::flutter::FlutterDaemon,
    redux::{action::Action, state::State},
};

use super::context::Context;

pub struct HotRestartThunk {
    context: Arc<Context>,
}

impl HotRestartThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for HotRestartThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(session_id) = store.select(|state: &State| state.session_id.clone()).await else {
            return;
        };

        let session_manager = self.context.session_manager.read().await;
        let run = &session_manager.sessions.get(&session_id).unwrap().run;
        run.hot_restart().await.unwrap();

        while let Ok(params) = run.receive_app_progress().await {
            if params.progress_id == Some("hot.restart".to_string()) && !params.finished {
                store
                    .dispatch(Action::StartHotRestart {
                        session_id: session_id.clone(),
                    })
                    .await;
                break;
            }
        }

        while let Ok(params) = run.receive_app_progress().await {
            if params.progress_id == Some("hot.restart".to_string()) && params.finished {
                store
                    .dispatch(Action::CompleteHotRestart {
                        session_id: session_id.clone(),
                    })
                    .await;
                break;
            }
        }
    }
}
