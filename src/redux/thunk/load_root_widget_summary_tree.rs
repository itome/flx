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
    state::State,
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

use super::{context::Context, load_details_subtree::LoadDetailsSubtreeThunk};

pub struct LoadRootWidgetWithSummaryTreeThunk {
    context: Arc<Context>,
    session_id: String,
}

impl LoadRootWidgetWithSummaryTreeThunk {
    pub fn new(context: Arc<Context>, session_id: String) -> Self {
        Self {
            context,
            session_id,
        }
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
        ids
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadRootWidgetWithSummaryTreeThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(session) = self.context.manager.session(self.session_id.clone()).await else {
            return;
        };
        let vm_service = &session.vm_service;

        let Ok(vm) = vm_service.get_vm().await else {
            return;
        };
        let Some(main_isolate) = vm.isolates.iter().find(|isolate| isolate.name == "main") else {
            return;
        };

        loop {
            if let Ok(is_widget_tree_ready) =
                vm_service.is_widget_tree_ready(&main_isolate.id).await
            {
                if is_widget_tree_ready.result {
                    break;
                }
            }
            sleep(Duration::from_millis(100)).await;
        }

        let response = match vm_service
            .get_root_widget_summary_tree_with_previews(
                &main_isolate.id,
                Some("get_root_widget_summary_tree_with_previews"),
            )
            .await
        {
            Ok(response) => response,
            Err(err) => {
                log::error!("Failed to get root widget summary tree: {:?}", err);
                return;
            }
        };

        if let Some(value_id) = response.result.value_id.clone() {
            let _store = store.clone();
            let context = self.context.clone();
            tokio::spawn(async move {
                LoadDetailsSubtreeThunk::new(context, value_id.clone())
                    .execute(_store)
                    .await;
            });
        }

        store
            .dispatch(Action::SetOpenWidgetValueId {
                session_id: self.session_id.clone(),
                ids: Self::all_ids(&response.result, 30),
            })
            .await;

        store
            .dispatch(Action::SetWidgetSummaryTree {
                session_id: self.session_id.clone(),
                tree: response.result,
            })
            .await;
    }
}
