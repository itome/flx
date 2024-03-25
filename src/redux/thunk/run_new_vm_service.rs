use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use daemon::flutter::FlutterDaemon;
use devtools::{
    protocols::vm_service::{StreamId, VmServiceProtocol},
    service::VmService,
};

use super::{context::Context, watch_frames::WatchFramesThunk};

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

        vm_service.connect(self.uri.clone()).await;

        let stream_ids = vec![
            StreamId::VM,
            StreamId::Isolate,
            StreamId::Debug,
            StreamId::Profiler,
            StreamId::GC,
            StreamId::Extension,
            StreamId::Timeline,
            StreamId::Logging,
            StreamId::Service,
            StreamId::HeapSnapshot,
        ];

        for stream_id in stream_ids {
            if let Err(e) = vm_service.stream_listen(stream_id.clone()).await {
                log::error!("Failed to cancel stream {:?}: {:?}", stream_id, e);
            }
        }

        let context = self.context.clone();
        let session_id = self.session_id.clone();
        tokio::spawn(async move {
            WatchFramesThunk::new(context, session_id)
                .execute(store)
                .await;
        });
    }
}
