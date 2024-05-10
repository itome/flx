use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};
use tokio::fs;
use vscode::launch::parse_launch_configuration;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{
    action::Action,
    state::{LaunchConfiguration, State},
};

use vscode;

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct LoadVSCodeLaunchSettingThunk {
    context: Arc<Context>,
}

impl LoadVSCodeLaunchSettingThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }

    async fn load_configurations(&self, launch_json_path: &Path) -> Vec<LaunchConfiguration> {
        if !launch_json_path.exists() {
            return vec![];
        }
        let Ok(json) = fs::read_to_string(launch_json_path).await else {
            return vec![];
        };

        let Ok(configurations) = parse_launch_configuration(&json) else {
            return vec![];
        };

        let configurations = configurations
            .iter()
            .map(|config| LaunchConfiguration {
                name: config.name.clone(),
                program: config.program.clone(),
                args: config.args.clone(),
                cwd: config.cwd.clone(),
                flutter_mode: config.flutter_mode.clone(),
            })
            .collect::<Vec<_>>();

        configurations
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadVSCodeLaunchSettingThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let project_root = store
            .select(|state: &State| state.project_root.clone())
            .await;

        let launch_json_path = project_root.join(".vscode/launch.json");
        let configurations = self.load_configurations(&launch_json_path).await;

        let mut dirs = HashSet::new();
        dirs.insert(project_root.clone());

        for configuration in &configurations {
            if let Some(cwd) = &configuration.cwd {
                let cwd = project_root.clone().join(cwd);
                dirs.insert(cwd);
            }
        }

        let mut supported_platforms = HashMap::new();
        for dir in dirs {
            let Ok(dir_string) = dir.clone().into_os_string().into_string() else {
                continue;
            };

            if let Ok(supprted_platforms) = self
                .context
                .daemon
                .get_supported_platforms(dir_string.clone())
                .await
            {
                supported_platforms.insert(dir, supprted_platforms);
            };
        }

        store
            .dispatch(Action::SetLaunchConfigurations { configurations })
            .await;

        store
            .dispatch(Action::SetSupportedPlatforms {
                supported_platforms,
            })
            .await;
    }
}
