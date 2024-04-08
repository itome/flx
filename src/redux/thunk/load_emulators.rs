use super::context::Context;
use crate::redux::{action::Action, state::State};
use async_trait::async_trait;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;
use redux_rs::{middlewares::thunk::Thunk, StoreApi};
use std::sync::Arc;

pub struct LoadEmulatorsThunk {
    context: Arc<Context>,
}

impl LoadEmulatorsThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadEmulatorsThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Ok(emulators) = self.context.daemon.get_emulators().await else {
            return;
        };

        store.dispatch(Action::SetEmultors { emulators }).await;
    }
}
