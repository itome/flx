use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::sync::Arc;

use redux_rs::{
    middlewares::thunk::{self, Thunk},
    StoreApi,
};

use crate::redux::{
    action::Action,
    selector::device_or_emulators::{self, device_or_emulators_selector, DeviceOrEmulator},
    state::State,
    thunk::{run_new_vm_service::RunNewVmServiceThunk, thunk_impl, ThunkAction},
};

use daemon::flutter::FlutterDaemon;

use super::context::Context;

pub struct LaunchEmulatorThunk {
    context: Arc<Context>,
}

impl LaunchEmulatorThunk {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<Api> Thunk<State, Action, Api> for LaunchEmulatorThunk
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    async fn execute(&self, store: Arc<Api>) {
        let Some(selected_emulator_id) = store
            .select(|state: &State| state.selected_device_or_emulator_id.clone())
            .await
        else {
            return;
        };

        let device_or_emulators = store.select(device_or_emulators_selector).await;
        let is_emulator =
            device_or_emulators
                .iter()
                .any(|device_or_emulator| match device_or_emulator {
                    DeviceOrEmulator::Emulator(emulator) => emulator.id == selected_emulator_id,
                    _ => false,
                });

        if !is_emulator {
            return;
        }

        if let Err(e) = self
            .context
            .daemon
            .launch_emulator(selected_emulator_id, false)
            .await
        {
            log::error!("Failed to launch emulator: {:?}", e);
        };
    }
}
