use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    daemon::flutter::FlutterDaemon,
    devtools::{
        io::{request::StreamId, types::EventKind},
        service::VmService,
    },
    redux::{action::Action, state::State},
};

use super::context::Context;

pub struct WatchFramesThunk {
    session_id: String,
    context: Arc<Context>,
}

impl WatchFramesThunk {
    pub fn new(context: Arc<Context>, session_id: String) -> Self {
        Self {
            context,
            session_id,
        }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for WatchFramesThunk
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

        loop {
            let Ok(event) = vm_service.receive_event(StreamId::Extension).await else {
                break;
            };
            if event.kind == EventKind::Extension
                && event.extension_kind == Some("Flutter.Frame".to_string())
            {
                log::info!("Received frame event: {:?}", event.extension_data);
            }
        }
    }
}
