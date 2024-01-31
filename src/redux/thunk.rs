use std::sync::Arc;

use redux_rs::{middlewares::thunk::Thunk, StoreApi};

use self::context::Context;

use super::{action::Action, state::State};

pub mod context;
pub mod watch_devices;

pub enum ThunkAction {
    WatchDevices,
}

pub fn thunk_impl<Api>(action: ThunkAction, context: Arc<Context>) -> impl Thunk<State, Action, Api>
where
    Api: StoreApi<State, Action> + Send + Sync + 'static,
{
    match action {
        ThunkAction::WatchDevices => watch_devices::WatchDevicesThunk::new(context),
    }
}
