use std::time::{SystemTime, UNIX_EPOCH};

use redux_rs::Selector;

use crate::redux::{
    selector::availale_devices::AvailableDevicesSelector,
    state::{Focus, PopUp, SelectDevicePopupState, SessionLog},
};

use super::{
    action::Action,
    state::{SessionState, State, Tab},
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
            current_focus: match state.current_focus {
                Focus::Tab(Tab::Project) => Focus::Tab(Tab::Runners),
                Focus::Tab(Tab::Runners) => Focus::Tab(Tab::Devices),
                Focus::Tab(Tab::Devices) => Focus::Tab(Tab::Project),
                _ => state.current_focus,
            },
            ..state
        },
        Action::PreviousTab => State {
            current_focus: match state.current_focus {
                Focus::Tab(Tab::Project) => Focus::Tab(Tab::Devices),
                Focus::Tab(Tab::Runners) => Focus::Tab(Tab::Project),
                Focus::Tab(Tab::Devices) => Focus::Tab(Tab::Runners),
                _ => state.current_focus,
            },
            ..state
        },
        Action::RegisterSession {
            session_id,
            device_id,
        } => State {
            session_id: Some(session_id.clone()),
            sessions: [
                state.sessions,
                vec![SessionState {
                    id: session_id,
                    device_id,
                    ..SessionState::default()
                }],
            ]
            .concat(),
            ..state
        },
        Action::UnregisterSession { session_id } => State {
            session_id: None,
            sessions: state
                .sessions
                .into_iter()
                .filter(|s| s.id != session_id)
                .collect(),
            ..state
        },
        Action::NextSession => State {
            session_id: match state.session_id {
                Some(session_id) => {
                    let index = state
                        .sessions
                        .iter()
                        .position(|s| s.id == session_id)
                        .unwrap();
                    let next_index = (index + 1) % state.sessions.len();
                    Some(state.sessions[next_index].id.clone())
                }
                None => state.sessions.first().map(|s| s.id.clone()),
            },
            ..state
        },
        Action::PreviousSession => State {
            session_id: match state.session_id {
                Some(session_id) => {
                    let index = state
                        .sessions
                        .iter()
                        .position(|s| s.id == session_id)
                        .unwrap();
                    let next_index = (index + state.sessions.len() - 1) % state.sessions.len();
                    Some(state.sessions[next_index].id.clone())
                }
                None => state.sessions.first().map(|s| s.id.clone()),
            },
            ..state
        },
        Action::NextDeviceForRunning => State {
            select_device_popup: SelectDevicePopupState {
                selected_device_id: {
                    let devices = AvailableDevicesSelector.select(&state);
                    match state.select_device_popup.selected_device_id {
                        Some(selected_device_id) => {
                            if let Some(index) =
                                devices.iter().position(|d| d.id == selected_device_id)
                            {
                                let next_index = (index + 1) % devices.len();
                                devices.get(next_index).map(|d| d.id.clone())
                            } else {
                                if devices.is_empty() {
                                    None
                                } else {
                                    devices.get(0).map(|d| d.id.clone())
                                }
                            }
                        }
                        None => {
                            if devices.is_empty() {
                                None
                            } else {
                                devices.get(0).map(|d| d.id.clone())
                            }
                        }
                    }
                },
                ..state.select_device_popup
            },
            ..state
        },
        Action::PreviousDeviceForRunning => State {
            select_device_popup: SelectDevicePopupState {
                selected_device_id: {
                    let devices = AvailableDevicesSelector.select(&state);
                    match state.select_device_popup.selected_device_id {
                        Some(selected_device_id) => {
                            if let Some(index) =
                                devices.iter().position(|d| d.id == selected_device_id)
                            {
                                let next_index = (index + devices.len() - 1) % devices.len();
                                devices.get(next_index).map(|d| d.id.clone())
                            } else {
                                if devices.is_empty() {
                                    None
                                } else {
                                    devices.get(devices.len() - 1).map(|d| d.id.clone())
                                }
                            }
                        }
                        None => {
                            if devices.is_empty() {
                                None
                            } else {
                                devices.get(devices.len() - 1).map(|d| d.id.clone())
                            }
                        }
                    }
                },
                ..state.select_device_popup
            },
            ..state
        },
        Action::ShowSelectDevicePopUp => State {
            current_focus: Focus::PopUp(PopUp::SelectDevice),
            select_device_popup: SelectDevicePopupState {
                visible: true,
                selected_device_id: AvailableDevicesSelector
                    .select(&state)
                    .get(0)
                    .map(|d| d.id.clone()),
            },
            ..state
        },
        Action::HideSelectDevicePopUp => State {
            current_focus: Focus::Tab(Tab::Runners),
            select_device_popup: SelectDevicePopupState {
                visible: false,
                ..state.select_device_popup
            },
            ..state
        },
        Action::StartApp {
            session_id,
            device_id,
            app_id,
            mode,
        } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            device_id: Some(device_id.clone()),
                            app_id: Some(app_id.clone()),
                            mode: Some(mode.clone()),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SetAppStarted { session_id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState { started: true, ..s }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::StartHotReload { session_id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            hot_reloading: true,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::CompleteHotReload { session_id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            hot_reloading: false,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::StartHotRestart { session_id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            hot_restarting: true,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::CompleteHotRestart { session_id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            hot_restarting: false,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SetProjectRoot { project_root } => State {
            project_root,
            ..state
        },
        Action::SetSupportedPlatforms { platforms } => State {
            supported_platforms: platforms,
            ..state
        },
        Action::AppendProgressLog {
            session_id,
            id,
            finished,
            message,
        } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            logs: {
                                if s.logs.iter().any(|log| {
                                    if let SessionLog::Progress { id: log_id, .. } = log {
                                        id == *log_id
                                    } else {
                                        false
                                    }
                                }) {
                                    s.logs
                                        .into_iter()
                                        .map(|log| {
                                            if let SessionLog::Progress {
                                                id: log_id,
                                                start_at,
                                                end_at,
                                                message,
                                            } = log
                                            {
                                                if id == log_id {
                                                    SessionLog::Progress {
                                                        id: log_id,
                                                        start_at,
                                                        message,
                                                        end_at: if finished {
                                                            Some(
                                                                SystemTime::now()
                                                                    .duration_since(UNIX_EPOCH)
                                                                    .unwrap()
                                                                    .as_millis(),
                                                            )
                                                        } else {
                                                            end_at
                                                        },
                                                    }
                                                } else {
                                                    SessionLog::Progress {
                                                        id: log_id,
                                                        start_at,
                                                        message: message.clone(),
                                                        end_at,
                                                    }
                                                }
                                            } else {
                                                log
                                            }
                                        })
                                        .collect()
                                } else {
                                    [
                                        s.logs,
                                        vec![SessionLog::Progress {
                                            id: id.clone(),
                                            message: message.clone(),
                                            start_at: SystemTime::now()
                                                .duration_since(UNIX_EPOCH)
                                                .unwrap()
                                                .as_millis(),
                                            end_at: None,
                                        }],
                                    ]
                                    .concat()
                                }
                            },
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::AppendStdoutLog { session_id, line } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            logs: { [s.logs, vec![SessionLog::Stdout(line.clone())]].concat() },
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
    }
}
