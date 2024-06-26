use crate::redux::state::{SessionLog, State};

use super::current_session::current_session_selector;

pub fn current_session_logs_selector(state: &State) -> Option<&Vec<SessionLog>> {
    let session = current_session_selector(state)?;
    Some(&session.logs)
}
