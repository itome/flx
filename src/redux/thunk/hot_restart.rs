use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use super::context::Context;
use daemon::flutter::FlutterDaemon;

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

        let Some(session) = self.context.manager.session(session_id.clone()).await else {
            return;
        };
        let run = &session.run;

        if let Err(err) = run.hot_restart().await {
            log::error!("Failed to hot restart: {}", err);
            return;
        }
    }
}
