use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct StopAppThunk {
    context: Arc<Context>,
}

impl StopAppThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for StopAppThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(session_id) = store.select(|state: &State| state.session_id.clone()).await else {
            return;
        };

        store
            .dispatch(Action::UnregisterSession {
                session_id: session_id.clone(),
            })
            .await;

        if let Some(session) = self.context.manager.session(session_id.clone()).await {
            if let Err(e) = session.run.stop().await {
                log::error!("Failed to stop session {}: {}", session_id, e);
            }
        };
    }
}
