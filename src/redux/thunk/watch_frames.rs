use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{sync::Arc, time::Duration};

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use devtools::{
    io::{request::StreamId, types::EventKind},
    service::VmService,
};

use daemon::flutter::FlutterDaemon;

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
            if event.kind != EventKind::Extension {
                continue;
            }
            if event.extension_kind != Some("Flutter.Frame".to_string()) {
                continue;
            }
            let Some(data) = event.extension_data else {
                continue;
            };

            let get_u64_data = |key: &str| -> u64 {
                data.get(key)
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .as_u64()
                    .unwrap()
            };
            store
                .dispatch(Action::AppendFlutterFrame {
                    session_id: self.session_id.clone(),
                    build: Duration::from_micros(get_u64_data("build")),
                    elapsed: Duration::from_micros(get_u64_data("elapsed")),
                    number: get_u64_data("number"),
                    raster: Duration::from_micros(get_u64_data("raster")),
                    start_time: Duration::from_micros(get_u64_data("startTime")),
                    vsync_overhead: Duration::from_micros(get_u64_data("vsyncOverhead")),
                })
                .await;
        }
    }
}
