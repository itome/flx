use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    daemon::flutter::FlutterDaemon,
    devtools::service::VmService,
    redux::{action::Action, state::State},
};

use super::context::Context;

pub struct RunNewVmServiceThunk {
    uri: String,
    session_id: String,
    context: Arc<Context>,
}

impl RunNewVmServiceThunk {
    pub fn new(context: Arc<Context>, session_id: String, uri: String) -> Self {
        Self {
            context,
            session_id,
            uri,
        }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for RunNewVmServiceThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Ok(session) = self
            .context
            .session_manager
            .session(self.session_id.clone())
            .await
        else {
            return;
        };
        let session = session.read().await;
        let vm_service = &session.as_ref().unwrap().vm_service;

        vm_service.start_websocket(self.uri.clone()).await;

        let version = vm_service.get_version().await;
        log::info!("Connected to VM Service version: {:?}", version);
    }
}
