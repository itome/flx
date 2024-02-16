use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    android,
    daemon::flutter::FlutterDaemon,
    ios,
    redux::{action::Action, state::State},
};

use super::context::Context;

pub struct LoadFlavorsThunk {
    context: Arc<Context>,
}

impl LoadFlavorsThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadFlavorsThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        store
            .dispatch(Action::SetFlavors { flavors: vec![] })
            .await;

        let project_root = store
            .select(|state: &State| state.project_root.clone())
            .await;

        let selected_device = store
            .select(|state: &State| state.select_device_popup.selected_device.clone())
            .await;

        let Some(selected_device) = selected_device else {
            return;
        };

        // TODO: (takassh): use enum
        let mut flavors = vec![];
        if selected_device.platform_type == *"ios" || selected_device.platform_type == *"macos" {
            match ios::get_schemes(project_root.unwrap_or(".".to_string())) {
                Some(shemes) => flavors = shemes,
                None => flavors = vec!["Undefined".to_string()],
            };
        } else if selected_device.platform_type == *"android" {
            match android::get_schemes(project_root.unwrap_or(".".to_string())) {
                Some(shemes) => flavors = shemes,
                None => flavors = vec!["Undefined".to_string()],
            };
        } else {
            flavors = vec!["Undefined".to_string()];
        }

        store
            .dispatch(Action::SetFlavors { flavors: flavors })
            .await;
    }
}
