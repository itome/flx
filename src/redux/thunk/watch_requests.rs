use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::sleep;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use devtools::{
    protocols::{
        flutter_extension::FlutterExtensionProtocol,
        io_extension::IoExtensionProtocol,
        vm_service::{EventKind, StreamId, VmServiceProtocol},
    },
    vm_service::VmService,
};

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct WatchRequestsThunk {
    session_id: String,
    context: Arc<Context>,
}

impl WatchRequestsThunk {
    pub fn new(context: Arc<Context>, session_id: String) -> Self {
        Self {
            context,
            session_id,
        }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for WatchRequestsThunk
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

        let mut latest_timestamp_for_isolate_id = HashMap::<String, i64>::new();
        let mut isolate_ids = vec![];

        loop {
            let Ok(vm) = vm_service.get_vm().await else {
                continue;
            };

            for isolate in vm.isolates {
                if isolate_ids.contains(&isolate.id) {
                    continue;
                }
                if let Ok(status) = vm_service
                    .http_enable_timeline_logging(isolate.id.clone(), true)
                    .await
                {
                    if status.enabled {
                        isolate_ids.push(isolate.id.clone());
                    }
                }
            }

            for isolate_id in isolate_ids.clone() {
                let Ok(request) = vm_service
                    .get_http_profile(
                        isolate_id.clone(),
                        latest_timestamp_for_isolate_id.get(&isolate_id).copied(),
                    )
                    .await
                else {
                    continue;
                };
                latest_timestamp_for_isolate_id.insert(isolate_id.clone(), request.timestamp);

                store
                    .dispatch(Action::AppendHttpProfileRequest {
                        session_id: self.session_id.clone(),
                        requests: request.requests,
                    })
                    .await;
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
}
