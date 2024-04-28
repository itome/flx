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
    selector::current_session::current_session_selector_cloned,
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

pub struct LoadDetailsSubtreeThunk {
    context: Arc<Context>,
    value_id: String,
}

impl LoadDetailsSubtreeThunk {
    pub fn new(context: Arc<Context>, value_id: String) -> Self {
        Self { context, value_id }
    }

    fn all_ids(node: &DiagnosticNode, depth: usize) -> HashSet<String> {
        let mut ids = HashSet::new();
        if depth == 0 {
            return ids;
        }
        ids.insert(node.value_id.clone().unwrap_or_default());
        if let Some(children) = node.children.as_ref() {
            for child in children {
                ids.extend(Self::all_ids(child, depth - 1));
            }
        }
        if let Some(properties) = node.properties.as_ref() {
            for property in properties {
                ids.extend(Self::all_ids(property, depth - 1));
            }
        }
        ids
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadDetailsSubtreeThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(SessionState {
            id: session_id,
            selected_widget_object_group: prev_object_group,
            ..
        }) = store.select(current_session_selector_cloned).await
        else {
            return;
        };

        let next_object_group = format!("subtree-{}-{}", session_id, self.value_id);

        store
            .dispatch(Action::SetSelectedWidgetObjectGroup {
                session_id: session_id.clone(),
                group: Some(next_object_group.clone()),
            })
            .await;

        store
            .dispatch(Action::SetSelectedWidgetDetailsTree {
                session_id: session_id.clone(),
                tree: None,
            })
            .await;

        let Ok(session) = self
            .context
            .session_manager
            .session(session_id.clone())
            .await
        else {
            return;
        };

        let session = session.read().await;
        let vm_service = &session.as_ref().unwrap().vm_service;

        let Ok(vm) = vm_service.get_vm().await else {
            return;
        };
        let Some(main_isolate) = vm.isolates.iter().find(|isolate| isolate.name == "main") else {
            return;
        };

        if let Err(e) = vm_service
            .dispose_group(&main_isolate.id, &next_object_group)
            .await
        {
            log::error!("Failed to dispose group: {:?}", e);
        };

        let response = match vm_service
            .get_details_subtree(
                &main_isolate.id,
                Some(&self.value_id),
                Some(2),
                &next_object_group,
            )
            .await
        {
            Ok(response) => response,
            Err(err) => {
                log::error!("Failed to get details subtree: {:?}", err);
                return;
            }
        };

        store
            .dispatch(Action::SetOpenWidgetDetailsValueId {
                session_id: session_id.clone(),
                ids: Self::all_ids(&response.result, 5),
            })
            .await;

        store
            .dispatch(Action::SetSelectedWidgetDetailsTree {
                session_id: session_id.clone(),
                tree: Some(response.result),
            })
            .await;
    }
}
