use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    daemon::flutter::FlutterDaemon,
    redux::{action::Action, state::State},
};

use super::context::Context;

pub struct WatchDevicesThunk {
    context: Arc<Context>,
}

impl WatchDevicesThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for WatchDevicesThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        self.context.daemon.enable_device().await.unwrap();

        loop {
            tokio::select! {
                Ok(device) = self.context.daemon.receive_device_added() => {
                    store.dispatch(Action::AddDevice { device })
                }
                Ok(device) = self.context.daemon.receive_device_removed() => {
                    store.dispatch(Action::RemoveDevice { device })
                }
            }
            .await;
        }
    }
}
