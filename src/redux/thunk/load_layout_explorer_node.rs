use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use tokio::time::sleep;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{
    action::Action,
    selector::current_session::{current_session_selector, current_session_selector_cloned},
    state::{SessionState, State},
};

use devtools::{
    protocols::{
        flutter_extension::{DiagnosticNode, FlutterExtensionProtocol},
        io_extension::IoExtensionProtocol,
        vm_service::{EventKind, StreamId, VmServiceProtocol},
    },
    vm_service::VmService,
};

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct LoadLayoutExplorerNodeThunk {
    context: Arc<Context>,
    value_id: String,
}

impl LoadLayoutExplorerNodeThunk {
    pub fn new(context: Arc<Context>, value_id: String) -> Self {
        Self { context, value_id }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadLayoutExplorerNodeThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(SessionState {
            id: session_id,
            selected_widget_object_group,
            ..
        }) = store.select(current_session_selector_cloned).await
        else {
            return;
        };
        let Some(session) = self.context.manager.session(session_id.clone()).await else {
            return;
        };
        let vm_service = &session.vm_service;

        let Ok(vm) = vm_service.get_vm().await else {
            return;
        };
        let Some(main_isolate) = vm.isolates.iter().find(|isolate| isolate.name == "main") else {
            return;
        };

        let response = match vm_service
            .get_layout_explorer_node(
                &main_isolate.id,
                Some(&self.value_id),
                &format!("explorer-{}-{}", session_id, self.value_id),
                Some(10000),
            )
            .await
        {
            Ok(response) => response,
            Err(err) => {
                log::error!("Failed to get root widget summary tree: {:?}", err);
                return;
            }
        };

        log::info!("{}", serde_json::to_string(&response).unwrap());
    }
}
