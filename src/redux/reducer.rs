use crate::redux::state::SelectDevicePopupState;

use super::{
    action::Action,
    state::{SessionState, State, Tab},
};

pub fn reducer(state: State, action: Action) -> State {
    log::info!("Action: {:?}", action);
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
                        .position(|s| s.id == session_id)
                        .unwrap();
                    let next_index = (index + state.sessions.len() - 1) % state.sessions.len();
                    Some(state.sessions[next_index].id.clone())
                }
                None => None,
            },
            ..state
        },
        Action::NextDeviceForRunning => State {
            select_device_popup: SelectDevicePopupState {
                selected_device_id: {
                    let devices = state.devices.iter().filter(|d| {
                        state.supported_platforms.contains(&d.platform_type)
                            && state
                                .sessions
                                .iter()
                                .all(|s| s.device_id != Some(d.id.clone()))
                    });
                    match state.select_device_popup.selected_device_id {
                        Some(selected_device_id) => {
                            let devices = devices.collect::<Vec<_>>();
                            if let Some(index) =
                                devices.iter().position(|d| d.id == selected_device_id)
                            {
                                let next_index = (index + 1) % devices.len();
                                devices.get(next_index).map(|d| d.id.clone())
                            } else {
                                None
                            }
                        }
                        None => state.devices.first().map(|d| d.id.clone()),
                    }
                },
                ..state.select_device_popup
            },
            ..state
        },
        Action::PreviousDeviceForRunning => State {
            select_device_popup: SelectDevicePopupState {
                selected_device_id: {
                    let devices = state.devices.iter().filter(|d| {
                        state.supported_platforms.contains(&d.platform_type)
                            && state
                                .sessions
                                .iter()
                                .all(|s| s.device_id != Some(d.id.clone()))
                    });
                    match state.select_device_popup.selected_device_id {
                        Some(selected_device_id) => {
                            let devices = devices.collect::<Vec<_>>();
                            if let Some(index) =
                                devices.iter().position(|d| d.id == selected_device_id)
                            {
                                let next_index = (index + devices.len() - 1) % devices.len();
                                devices.get(next_index).map(|d| d.id.clone())
                            } else {
                                None
                            }
                        }
                        None => state.devices.first().map(|d| d.id.clone()),
                    }
                },
                ..state.select_device_popup
            },
            ..state
        },
        Action::ShowSelectDevicePopUp => State {
            select_device_popup: SelectDevicePopupState {
                visible: true,
                selected_device_id: state
                    .devices
                    .iter()
                    .filter(|d| {
                        state.supported_platforms.contains(&d.platform_type)
                            && state
                                .sessions
                                .iter()
                                .all(|s| s.device_id != Some(d.id.clone()))
                    })
                    .nth(0)
                    .map(|d| d.id.clone()),
            },
            ..state
        },
        Action::HideSelectDevicePopUp => State {
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
    }
}
