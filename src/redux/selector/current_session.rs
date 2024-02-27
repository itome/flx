use redux_rs::Selector;

use crate::{
    daemon::io::device::Device,
    redux::state::{SessionLog, SessionState, State},
};

pub struct CurrentSessionSelector;

impl Selector<State> for CurrentSessionSelector {
    type Result = Option<SessionState>;

    fn select(&self, state: &State) -> Self::Result {
        let Some(session_id) = state.session_id.clone() else {
            return None;
        };
        for session in state.sessions.clone() {
            if session.id == session_id {
                return Some(session);
            }
        }
        None
    }
}
