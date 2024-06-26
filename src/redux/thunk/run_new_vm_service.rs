use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use daemon::flutter::FlutterDaemon;
use devtools::{
    protocols::vm_service::{StreamId, VmServiceProtocol},
    vm_service::VmService,
};

use super::{
    context::Context, load_root_widget_summary_tree::LoadRootWidgetWithSummaryTreeThunk,
    watch_frames::WatchFramesThunk, watch_requests::WatchRequestsThunk,
};

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
        let Some(session) = self.context.manager.session(self.session_id.clone()).await else {
            return;
        };
        let vm_service = &session.vm_service;

        vm_service.connect(self.uri.clone()).await;

        let stream_ids = vec![StreamId::Extension];

        for stream_id in stream_ids {
            if let Err(e) = vm_service.stream_listen(stream_id.clone()).await {
                log::error!("Failed to listen stream {:?}: {:?}", stream_id, e);
            }
        }

        let _store = store.clone();
        let context = self.context.clone();
        let session_id = self.session_id.clone();
        tokio::spawn(async move {
            WatchFramesThunk::new(context, session_id)
                .execute(_store)
                .await;
        });

        let _store = store.clone();
        let context = self.context.clone();
        let session_id = self.session_id.clone();
        tokio::spawn(async move {
            WatchRequestsThunk::new(context, session_id)
                .execute(_store)
                .await;
        });

        let _store = store.clone();
        let context = self.context.clone();
        let session_id = self.session_id.clone();
        tokio::spawn(async move {
            LoadRootWidgetWithSummaryTreeThunk::new(context, session_id)
                .execute(_store)
                .await;
        });
    }
}
