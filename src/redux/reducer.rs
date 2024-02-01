use super::{
    action::Action,
    state::{State, Tab},
};

pub fn reducer(state: State, action: Action) -> State {
    match action {
        Action::AddDevice { device } => State {
            devices: [state.devices, vec![device]].concat(),
            ..state
        },
        Action::RemoveDevice { device } => State {
            devices: state.devices.into_iter().filter(|d| d != &device).collect(),
            ..state
        },
        Action::NextTab => State {
            selected_tab: match state.selected_tab {
                Tab::Project => Tab::Runners,
                Tab::Runners => Tab::Devices,
                Tab::Devices => Tab::Project,
            },
            ..state
        },
        Action::PreviousTab => State {
            selected_tab: match state.selected_tab {
                Tab::Project => Tab::Devices,
                Tab::Runners => Tab::Project,
                Tab::Devices => Tab::Runners,
            },
            ..state
        },
        Action::RegisterSession { session_id } => State {
            session_id: Some(session_id.clone()),
            sessions: [state.sessions, vec![session_id]].concat(),
            ..state
        },
        Action::UnregisterSession { session_id } => State {
            session_id: None,
            sessions: state
                .sessions
                .into_iter()
                .filter(|s| s != &session_id)
                .collect(),
            ..state
        },
        Action::NextSession => State {
            session_id: match state.session_id {
                Some(session_id) => {
                    let index = state
                        .sessions
                        .iter()
                        .position(|s| s == &session_id)
                        .unwrap();
                    let next_index = (index + 1) % state.sessions.len();
                    Some(state.sessions[next_index].clone())
                }
                None => None,
            },
            ..state
        },
        Action::PreviousSession => State {
            session_id: match state.session_id {
                Some(session_id) => {
                    let index = state
                        .sessions
                        .iter()
                        .position(|s| s == &session_id)
                        .unwrap();
                    let next_index = (index + state.sessions.len() - 1) % state.sessions.len();
                    Some(state.sessions[next_index].clone())
                }
                None => None,
            },
            ..state
        },
    }
}
