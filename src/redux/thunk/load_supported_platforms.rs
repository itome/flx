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

        let Ok(supprted_platforms) = self
            .context
            .daemon
            .get_supported_platforms(project_root.clone().unwrap_or(".".to_string()))
            .await
        else {
            return;
        };

        store
            .dispatch(Action::SetSupportedPlatforms {
                platforms: supprted_platforms,
            })
            .await;

        let mut flavors = HashMap::new();

        let supported_platforms = store
            .select(|state: &State| state.supported_platforms.clone())
            .await;

        for supported_platform in supported_platforms {
            match supported_platform.as_str() {
                "ios" => {
                    if let Ok(Some(schemes)) =
                        ios::get_schemes(project_root.clone().unwrap_or(".".to_string()))
                    {
                        flavors.insert("ios".to_string(), schemes);
                    }
                }
                "macos" => {
                    if let Ok(Some(schemes)) =
                        ios::get_schemes(project_root.clone().unwrap_or(".".to_string()))
                    {
                        flavors.insert("darwin".to_string(), schemes);
                    }
                }
                "android" => {
                    if let Ok(Some(schemes)) =
                        android::get_schemes(project_root.clone().unwrap_or(".".to_string()))
                    {
                        flavors.insert("android".to_string(), schemes);
                    }
                }
                _ => {}
            }
        }

        flavors.entry("ios".to_string()).or_insert(vec![]);
        flavors.entry("darwin".to_string()).or_insert(vec![]);
        flavors.entry("android".to_string()).or_insert(vec![]);

        store.dispatch(Action::SetFlavors { flavors }).await;
    }
}
