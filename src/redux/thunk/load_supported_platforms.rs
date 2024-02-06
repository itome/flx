use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::{
    daemon::flutter::FlutterDaemon,
    redux::{action::Action, state::State},
};

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

        let Ok(supprted_platforms) = self
            .context
            .daemon
            .get_supported_platforms(project_root.unwrap_or(".".to_string()))
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
