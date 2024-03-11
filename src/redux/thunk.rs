use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use self::context::Context;

use super::{action::Action, state::State};

pub mod context;
pub mod hot_reload;
pub mod hot_restart;
pub mod load_supported_platforms;
pub mod run_new_app;
pub mod stop_app;
pub mod watch_devices;

pub enum ThunkAction {
    WatchDevices,
    LoadSupportedPlatforms,
    RunNewApp,
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
        ThunkAction::RunNewApp => Box::new(run_new_app::RunNewAppThunk::new(context)),
        ThunkAction::HotReload => Box::new(hot_reload::HotReloadThunk::new(context)),
        ThunkAction::HotRestart => Box::new(hot_restart::HotRestartThunk::new(context)),
        ThunkAction::StopApp => Box::new(stop_app::StopAppThunk::new(context)),
        ThunkAction::LoadSupportedPlatforms => Box::new(
            load_supported_platforms::LoadSupportedPlatformsThunk::new(context),
        ),
    }
}
