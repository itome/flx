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

pub struct ToggleDebugPaintBaselinesThunk {
    context: Arc<Context>,
}

impl ToggleDebugPaintBaselinesThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for ToggleDebugPaintBaselinesThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(SessionState {
            id: session_id,
            debug_paint_baselines_enabled,
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
            .debug_paint_baseline_enabled(&main_isolate.id, Some(!debug_paint_baselines_enabled))
            .await
        {
            store
                .dispatch(Action::SetDebugPaintBaselinesEnabled {
                    session_id,
                    enabled: togglable.enabled,
                })
                .await;
        }
    }
}
