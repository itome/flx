use redux_rs::Selector;

use crate::{
    daemon::io::device::Device,
    redux::state::{SessionLog, State},
};

pub struct CurrentSessionLogsSelector;

impl Selector<State> for CurrentSessionLogsSelector {
    type Result = Vec<SessionLog>;

    fn select(&self, state: &State) -> Self::Result {
        let Some(session_id) = state.session_id.clone() else {
            return vec![];
        };
        for session in state.sessions.clone() {
            if session.id == session_id {
                return session.logs.clone();
            }
        }
        vec![]
    }
}
