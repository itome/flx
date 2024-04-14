use async_trait::async_trait;
use color_eyre::eyre::Result;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::process::Command;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use crate::redux::{action::Action, state::State};

use android;
use ios;

use daemon::flutter::FlutterDaemon;

use super::context::Context;

#[derive(Deserialize, Debug)]
pub struct SdkVersionJson {
    #[serde(rename = "frameworkVersion")]
    pub framework_version: String,
    pub channel: String,
    #[serde(rename = "repositoryUrl")]
    pub repository_url: String,
    #[serde(rename = "frameworkRevision")]
    pub framework_revision: String,
    #[serde(rename = "frameworkCommitDate")]
    pub framework_commit_date: String,
    #[serde(rename = "engineRevision")]
    pub engine_revision: String,
    #[serde(rename = "dartSdkVersion")]
    pub dart_sdk_version: String,
    #[serde(rename = "devToolsVersion")]
    pub dev_tools_version: String,
    #[serde(rename = "flutterVersion")]
    pub flutter_version: String,
    #[serde(rename = "flutterRoot")]
    pub flutter_root: String,
}

pub struct LoadSdkVersionsThunk {
    context: Arc<Context>,
    use_fvm: bool,
}

impl LoadSdkVersionsThunk {
    pub fn new(context: Arc<Context>, use_fvm: bool) -> Self {
        Self { context, use_fvm }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadSdkVersionsThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Ok(output) = Command::new("flutter")
            .arg("--version")
            .arg("--machine")
            .output()
            .await
        else {
            return;
        };

        let output = String::from_utf8(output.stdout).unwrap();
        let sdk_version: SdkVersionJson = match serde_json::from_str(&output) {
            Ok(sdk_version) => sdk_version,
            Err(e) => {
                log::error!("Failed to parse SDK version: {}", e);
                return;
            }
        };

        store
            .dispatch(Action::SetSdkVersion {
                framework_version: sdk_version.framework_version,
                channel: sdk_version.channel,
                repository_url: sdk_version.repository_url,
                framework_revision: sdk_version.framework_revision,
                framework_commit_date: sdk_version.framework_commit_date,
                engine_revision: sdk_version.engine_revision,
                dart_sdk_version: sdk_version.dart_sdk_version,
                dev_tools_version: sdk_version.dev_tools_version,
                flutter_version: sdk_version.flutter_version,
                flutter_root: sdk_version.flutter_root,
            })
            .await;
    }
}
