use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{sync::Arc, time::Duration};

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{
    action::Action,
    selector::current_session::current_session_selector_cloned,
    state::{SessionState, State},
};

use devtools::{
    protocols::{flutter_extension::FlutterExtensionProtocol, vm_service::VmServiceProtocol},
    vm_service::VmService,
};

use super::context::Context;

pub struct ToggleShowPerformanceOverlayThunk {
    context: Arc<Context>,
}

impl ToggleShowPerformanceOverlayThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for ToggleShowPerformanceOverlayThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(SessionState {
            id: session_id,
            show_performance_overlay_enabled,
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

        if let Ok(togglable) = vm_service
            .show_performance_overlay(&main_isolate.id, Some(!show_performance_overlay_enabled))
            .await
        {
            store
                .dispatch(Action::SetShowPerformanceOverlayEnabled {
                    session_id,
                    enabled: togglable.enabled,
                })
                .await;
        }
    }
}
