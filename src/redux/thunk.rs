use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use self::context::Context;

use super::{action::Action, state::State};

pub mod context;
pub mod hot_reload;
pub mod hot_restart;
pub mod launch_emulator;
pub mod load_details_subtree;
pub mod load_emulators;
pub mod load_full_request;
pub mod load_layout_explorer_node;
pub mod load_root_widget_summary_tree;
pub mod load_sdk_versions;
pub mod load_vscode_launch_setting;
pub mod run_new_app;
pub mod run_new_vm_service;
pub mod stop_app;
pub mod toggle_debug_paint;
pub mod toggle_debug_paint_baselines;
pub mod toggle_invert_oversized_images;
pub mod toggle_repaint_rainbow;
pub mod toggle_show_performance_overlay;
pub mod toggle_slow_animations;
pub mod watch_devices;
pub mod watch_frames;
pub mod watch_requests;

#[derive(Debug)]
pub enum ThunkAction {
    WatchDevices,
    LoadSdkVersions { use_fvm: bool },
    LoadEmulators,
    LoadFullRequest,
    LoadVSCodeLaunchSetting,
    LoadRootWidgetSummaryTree { session_id: String },
    LoadLayoutExplorerNode { value_id: String },
    LoadDetailsSubtree { value_id: String },
    RunNewApp { use_fvm: bool },
    LaunchEmulator,
    HotReload,
    HotRestart,
    StopApp,
    ToggleDebugPaint,
    ToggleDebugPaintBaselines,
    ToggleInvertOversizedImages,
    ToggleRepaintRainbow,
    ToggleShowPerformanceOverlay,
    ToggleSlowAnimations,
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
        ThunkAction::LoadSdkVersions { use_fvm } => Box::new(
            load_sdk_versions::LoadSdkVersionsThunk::new(context, use_fvm),
        ),
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
        ThunkAction::LoadLayoutExplorerNode { value_id } => Box::new(
            load_layout_explorer_node::LoadLayoutExplorerNodeThunk::new(context, value_id),
        ),
        ThunkAction::LoadDetailsSubtree { value_id } => Box::new(
            load_details_subtree::LoadDetailsSubtreeThunk::new(context, value_id),
        ),
        ThunkAction::ToggleDebugPaint => {
            Box::new(toggle_debug_paint::ToggleDebugPaintThunk::new(context))
        }
        ThunkAction::ToggleDebugPaintBaselines => {
            Box::new(toggle_debug_paint_baselines::ToggleDebugPaintBaselinesThunk::new(context))
        }
        ThunkAction::ToggleInvertOversizedImages => {
            Box::new(toggle_invert_oversized_images::ToggleInvertOversizedImagesThunk::new(context))
        }
        ThunkAction::ToggleRepaintRainbow => Box::new(
            toggle_repaint_rainbow::ToggleRepaintRainbowThunk::new(context),
        ),
        ThunkAction::ToggleShowPerformanceOverlay => Box::new(
            toggle_show_performance_overlay::ToggleShowPerformanceOverlayThunk::new(context),
        ),
        ThunkAction::ToggleSlowAnimations => Box::new(
            toggle_slow_animations::ToggleSlowAnimationsThunk::new(context),
        ),
    }
}
