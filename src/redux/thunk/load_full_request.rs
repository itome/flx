use async_trait::async_trait;
use color_eyre::eyre::Result;
use serde_json::Value;
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

pub struct LoadFullRequestThunk {
    context: Arc<Context>,
}

impl LoadFullRequestThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LoadFullRequestThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(current_session) = store.select(current_session_selector_cloned).await else {
            return;
        };

        let Some(request) = current_session
            .requests
            .iter()
            .find(|r| Some(r.id.clone()) == current_session.selected_request_id)
        else {
            return;
        };

        // If the request is already loaded, return
        if current_session.full_requests.contains_key(&request.id) {
            return;
        }

        let Some(session) = self
            .context
            .manager
            .session(current_session.id.clone())
            .await
        else {
            return;
        };
        let vm_service = &session.vm_service;

        let Ok(mut full_request) = vm_service
            .get_http_profile_request(request.isolate_id.clone(), request.id.clone())
            .await
        else {
            return;
        };

        if let Some(response_body) = full_request.response_body.clone() {
            if let Ok(body_string) = String::from_utf8(response_body) {
                let json: serde_json::Result<Value> = serde_json::from_str(&body_string);
                if let Ok(json) = json {
                    if let Ok(json) = serde_json::to_string_pretty(&json) {
                        full_request.response_body = Some(json.as_bytes().to_vec());
                    }
                }
            }
        }

        store
            .dispatch(Action::AppendHttpProfileFullRequest {
                session_id: current_session.id.clone(),
                request: full_request,
            })
            .await;
    }
}
