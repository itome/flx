use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct HotReloadThunk {
    context: Arc<Context>,
}

impl HotReloadThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for HotReloadThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(session_id) = store.select(|state: &State| state.session_id.clone()).await else {
            return;
        };

        let Ok(session) = self
            .context
            .session_manager
            .session(session_id.clone())
            .await
        else {
            return;
        };
        let session = session.read().await;
        let run = &session.as_ref().unwrap().run;

        run.hot_reload().await.unwrap();

        while let Ok(params) = run.receive_app_progress().await {
            if params.progress_id == Some("hot.reload".to_string()) && !params.finished {
                store
                    .dispatch(Action::StartHotReload {
                        session_id: session_id.clone(),
                    })
                    .await;
                break;
            }
        }

        while let Ok(params) = run.receive_app_progress().await {
            if params.progress_id == Some("hot.reload".to_string()) && params.finished {
                store
                    .dispatch(Action::CompleteHotReload {
                        session_id: session_id.clone(),
                    })
                    .await;
                break;
            }
        }
    }
}
