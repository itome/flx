use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::sleep;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{
    action::Action,
    selector::current_session::{current_session_selector, current_session_selector_cloned},
    state::State,
};

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
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadRootWidgetWithSummaryTreeThunk
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

        let Ok(vm) = vm_service.get_vm().await else {
            return;
        };
        let Some(main_isolate) = vm.isolates.iter().find(|isolate| isolate.name == "main") else {
            return;
        };

        let Ok(response) = vm_service
            .get_root_widget_summary_tree_with_previews(
                &main_isolate.id,
                Some("get_root_widget_summary_tree_with_previews"),
            )
            .await
        else {
            return;
        };

        store
            .dispatch(Action::SetWidgetSummaryTree {
                session_id: self.session_id.clone(),
                tree: response.result,
            })
            .await;
    }
}
