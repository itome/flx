use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{sync::Arc, time::Duration};

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use devtools::{
    protocols::{
        flutter_extension::FlutterExtensionProtocol,
        vm_service::{EventKind, StreamId},
    },
    vm_service::VmService,
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
        let Some(session) = self.context.manager.session(self.session_id.clone()).await else {
            return;
        };
        let vm_service = &session.vm_service;

        let Ok(flutter_view_list) = vm_service.list_views().await else {
            return;
        };
        let Some(flutter_view) = flutter_view_list
            .views
            .iter()
            .find(|view| view.r#type == "FlutterView")
        else {
            return;
        };
        let Ok(display_refresh_rate) = vm_service.get_display_refresh_rate(&flutter_view.id).await
        else {
            return;
        };
        store
            .dispatch(Action::SetDisplayRefreshRate {
                session_id: self.session_id.clone(),
                rate: display_refresh_rate.fps,
            })
            .await;

        loop {
            let Ok(event) = vm_service.next_event(StreamId::Extension).await else {
                continue;
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
