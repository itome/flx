use redux_rs::Selector;

use crate::redux::state::{SessionLog, SessionState, State};

use daemon::io::device::Device;

pub struct CurrentSessionSelector;

impl Selector<State> for CurrentSessionSelector {
    type Result = Option<SessionState>;

    fn select(&self, state: &State) -> Self::Result {
        let session_id = state.session_id.clone()?;
        state
            .sessions
            .clone()
            .into_iter()
            .find(|session| session.id == session_id)
    }
}
