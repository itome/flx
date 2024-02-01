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
        log::info!("Hot restarting app");
        let Some(session_id) = store.select(|state: &State| state.session_id.clone()).await else {
            return;
        };
        log::info!("Session ID: {}", session_id);

        let session_manager = self.context.session_manager.read().await;
        log::info!("Session manager locked");
        session_manager
            .sessions
            .get(&session_id)
            .unwrap()
            .run
            .hot_restart()
            .await
            .unwrap();
        log::info!("Hot restart finished");
    }
}
