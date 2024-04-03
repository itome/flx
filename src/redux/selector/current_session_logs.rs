use crate::redux::state::{SessionLog, State};

use super::current_session::current_session_selector;

pub fn current_session_logs_selector(state: &State) -> Option<&Vec<SessionLog>> {
    let Some(session) = current_session_selector(state) else {
        return None;
    };
    Some(&session.logs)
}
