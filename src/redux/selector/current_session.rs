use crate::redux::state::{SessionLog, SessionState, State};

pub fn current_session_selector(state: &State) -> Option<&SessionState> {
    if let Some(session_id) = &state.session_id {
        state.sessions.iter().find(|s| &s.id == session_id)
    } else {
        None
    }
}

pub fn current_session_selector_cloned(state: &State) -> Option<SessionState> {
    current_session_selector(state).cloned()
}
