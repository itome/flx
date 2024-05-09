use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{collections::HashMap, sync::Arc};

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use android;
use ios;

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct LoadSupportedPlatformsThunk {
    context: Arc<Context>,
}

impl LoadSupportedPlatformsThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadSupportedPlatformsThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let project_root = store
            .select(|state: &State| state.project_root.clone())
            .await;

        let Ok(project_root) = project_root.into_os_string().into_string() else {
            return;
        };

        let Ok(supprted_platforms) = self
            .context
            .daemon
            .get_supported_platforms(project_root.clone())
            .await
        else {
            return;
        };

        store
            .dispatch(Action::SetSupportedPlatforms {
                platforms: supprted_platforms,
            })
            .await;
    }
}
