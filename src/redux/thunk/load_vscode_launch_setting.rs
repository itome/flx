use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::{collections::HashMap, path::Path, sync::Arc};
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
        if !launch_json_path.exists() {
            return;
        }
        let Ok(json) = fs::read_to_string(launch_json_path).await else {
            return;
        };

        let Ok(configurations) = parse_launch_configuration(&json) else {
            return;
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

        store
            .dispatch(Action::SetLaunchConfigurations { configurations })
            .await;
    }
}
