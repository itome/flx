use std::time::{SystemTime, UNIX_EPOCH};

use redux_rs::Selector;

use crate::redux::{
    selector::availale_devices::AvailableDevicesSelector,
    state::{Focus, PopUp, SelectDevicePopupState, SessionLog},
};

use super::{
    action::Action,
    state::{FlutterFrame, SelectFlavorPopupState, SessionState, State, Tab},
};

pub fn reducer(state: State, action: Action) -> State {
    match action {
        Action::AddDevice { device } => State {
            devices: [state.devices, vec![device.clone()]].concat(),
            select_device_popup: SelectDevicePopupState {
                selected_device: {
                    let is_supported = state.supported_platforms.contains(&device.platform_type)
                        && state
                            .sessions
                            .iter()
                            .all(|s| s.device_id != Some(device.id.clone()));

                    if state.select_device_popup.selected_device.is_none() && is_supported {
                        Some(device.to_owned())
                    } else {
                        state.select_device_popup.selected_device
                    }
                },
                ..state.select_device_popup
            },
            ..state
        },
        Action::RemoveDevice { device } => State {
            devices: state.devices.into_iter().filter(|d| d != &device).collect(),
            ..state
        },
        Action::SetFlavors { flavors } => State { flavors, ..state },
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
            flavor,
        } => State {
            session_id: Some(session_id.clone()),
            sessions: [
                state.sessions,
                vec![SessionState {
                    id: session_id,
                    device_id,
                    flavor,
                    display_refresh_rate: 60.0,
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
                selected_device: {
                    let devices = AvailableDevicesSelector.select(&state);
                    match state.select_device_popup.selected_device {
                        Some(selected_device) => {
                            if let Some(index) =
                                devices.iter().position(|d| d.id == selected_device.id)
                            {
                                let next_index = (index + 1) % devices.len();
                                devices.get(next_index).map(|d| d.to_owned())
                            } else if devices.is_empty() {
                                None
                            } else {
                                devices.first().map(|d| d.to_owned())
                            }
                        }
                        None => {
                            if devices.is_empty() {
                                None
                            } else {
                                devices.first().map(|d| d.to_owned())
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
                selected_device: {
                    let devices = AvailableDevicesSelector.select(&state);
                    match state.select_device_popup.selected_device {
                        Some(selected_device) => {
                            if let Some(index) =
                                devices.iter().position(|d| d.id == selected_device.id)
                            {
                                let next_index = (index + devices.len() - 1) % devices.len();
                                devices.get(next_index).map(|d| d.to_owned())
                            } else if devices.is_empty() {
                                None
                            } else {
                                devices.last().map(|d| d.to_owned())
                            }
                        }
                        None => {
                            if devices.is_empty() {
                                None
                            } else {
                                devices.last().map(|d| d.to_owned())
                            }
                        }
                    }
                },
                ..state.select_device_popup
            },
            ..state
        },
        Action::NextFlavorForRunning => State {
            select_flavor_popup: SelectFlavorPopupState {
                selected_flavor: {
                    let selected_device_platform = &state
                        .select_device_popup
                        .selected_device_platform()
                        .unwrap_or("".to_string());
                    let Some(flavors) = &state.flavors.get(selected_device_platform) else {
                        return state;
                    };

                    match state.select_flavor_popup.selected_flavor {
                        Some(selected_flavor) => {
                            if let Some(index) = flavors.iter().position(|f| f == &selected_flavor)
                            {
                                let next_index = (index + 1) % flavors.len();
                                flavors.get(next_index).map(|d| d.to_owned())
                            } else if flavors.is_empty() {
                                None
                            } else {
                                flavors.first().map(|d| d.to_owned())
                            }
                        }
                        None => {
                            if flavors.is_empty() {
                                None
                            } else {
                                flavors.first().map(|f| f.to_owned())
                            }
                        }
                    }
                },
                ..state.select_flavor_popup
            },
            ..state
        },
        Action::PreviousFlavorForRunning => State {
            select_flavor_popup: SelectFlavorPopupState {
                selected_flavor: {
                    let selected_device_platform = &state
                        .select_device_popup
                        .selected_device_platform()
                        .unwrap_or("".to_string());
                    let Some(flavors) = &state.flavors.get(selected_device_platform) else {
                        return state;
                    };

                    match state.select_flavor_popup.selected_flavor {
                        Some(selected_flavor) => {
                            if let Some(index) = flavors.iter().position(|f| f == &selected_flavor)
                            {
                                let next_index = (index + flavors.len() - 1) % flavors.len();
                                flavors.get(next_index).map(|d| d.to_owned())
                            } else if flavors.is_empty() {
                                None
                            } else {
                                flavors.last().map(|d| d.to_owned())
                            }
                        }
                        None => {
                            if flavors.is_empty() {
                                None
                            } else {
                                flavors.last().map(|d| d.to_owned())
                            }
                        }
                    }
                },
                ..state.select_flavor_popup
            },
            ..state
        },
        Action::ShowSelectDevicePopUp => State {
            current_focus: Focus::PopUp(PopUp::SelectDevice),
            select_device_popup: SelectDevicePopupState {
                visible: true,
                selected_device: AvailableDevicesSelector
                    .select(&state)
                    .first()
                    .map(|d| d.to_owned()),
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
        Action::ShowSelectFlavorPopUp => State {
            current_focus: Focus::PopUp(PopUp::SelectFlavor),
            select_flavor_popup: SelectFlavorPopupState {
                visible: true,
                selected_flavor: None,
            },
            ..state
        },
        Action::HideSelectFlavorPopUp => State {
            current_focus: Focus::Tab(Tab::Runners),
            select_flavor_popup: SelectFlavorPopupState {
                visible: false,
                ..state.select_flavor_popup
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
        Action::AppendFlutterFrame {
            session_id,
            build,
            elapsed,
            number,
            raster,
            start_time,
            vsync_overhead,
        } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            frames: {
                                [
                                    s.frames,
                                    vec![FlutterFrame {
                                        build,
                                        elapsed,
                                        number,
                                        raster,
                                        start_time,
                                        vsync_overhead,
                                    }],
                                ]
                                .concat()
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
        Action::SetDisplayRefreshRate { session_id, rate } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            display_refresh_rate: rate,
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
