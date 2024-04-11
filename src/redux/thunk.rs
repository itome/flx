use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use self::context::Context;

use super::{action::Action, state::State};

pub mod context;
pub mod hot_reload;
pub mod hot_restart;
pub mod launch_emulator;
pub mod load_emulators;
pub mod load_full_request;
pub mod load_root_widget_summary_tree;
pub mod load_supported_platforms;
pub mod load_vscode_launch_setting;
pub mod run_new_app;
pub mod run_new_vm_service;
pub mod stop_app;
pub mod watch_devices;
pub mod watch_frames;
pub mod watch_requests;

#[derive(Debug)]
pub enum ThunkAction {
    WatchDevices,
    LoadSupportedPlatforms,
    LoadEmulators,
    LoadFullRequest,
    LoadVSCodeLaunchSetting,
    LoadRootWidgetSummaryTree { session_id: String },
    RunNewApp { use_fvm: bool },
    LaunchEmulator,
    HotReload,
    HotRestart,
    StopApp,
}

pub fn thunk_impl<Api>(
    action: ThunkAction,
    context: Arc<Context>,
) -> Box<dyn Thunk<State, Action, Api> + Send + Sync>
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    match action {
        ThunkAction::WatchDevices => Box::new(watch_devices::WatchDevicesThunk::new(context)),
        ThunkAction::LoadEmulators => Box::new(load_emulators::LoadEmulatorsThunk::new(context)),
        ThunkAction::LoadVSCodeLaunchSetting => {
            Box::new(load_vscode_launch_setting::LoadVSCodeLaunchSettingThunk::new(context))
        }
        ThunkAction::LoadRootWidgetSummaryTree { session_id } => Box::new(
            load_root_widget_summary_tree::LoadRootWidgetWithSummaryTreeThunk::new(
                context, session_id,
            ),
        ),
        ThunkAction::RunNewApp { use_fvm } => {
            Box::new(run_new_app::RunNewAppThunk::new(context, use_fvm))
        }
        ThunkAction::LoadFullRequest => {
            Box::new(load_full_request::LoadFullRequestThunk::new(context))
        }
        ThunkAction::LaunchEmulator => Box::new(launch_emulator::LaunchEmulatorThunk::new(context)),
        ThunkAction::HotReload => Box::new(hot_reload::HotReloadThunk::new(context)),
        ThunkAction::HotRestart => Box::new(hot_restart::HotRestartThunk::new(context)),
        ThunkAction::StopApp => Box::new(stop_app::StopAppThunk::new(context)),
        ThunkAction::LoadSupportedPlatforms => Box::new(
            load_supported_platforms::LoadSupportedPlatformsThunk::new(context),
        ),
    }
}
