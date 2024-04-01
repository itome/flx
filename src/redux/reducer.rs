use std::time::{SystemTime, UNIX_EPOCH};

use redux_rs::Selector;

use crate::redux::{
    selector::availale_devices::AvailableDevicesSelector,
    state::{Focus, PopUp, SelectDevicePopupState, SessionLog},
};

use super::{
    action::Action,
    state::{DevTools, FlutterFrame, Home, SelectFlavorPopupState, SessionState, State},
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
            },
            ..state
        },
        Action::RemoveDevice { device } => State {
            devices: state.devices.into_iter().filter(|d| d != &device).collect(),
            ..state
        },
        Action::SetFlavors { flavors } => State { flavors, ..state },
        Action::NextHomeTab => State {
            focus: match state.focus {
                Focus::Home(Home::Project) => Focus::Home(Home::Runners),
                Focus::Home(Home::Runners) => Focus::Home(Home::Devices),
                Focus::Home(Home::Devices) => Focus::Home(Home::Project),
                _ => state.focus,
            },
            ..state
        },
        Action::PreviousHomeTab => State {
            focus: match state.focus {
                Focus::Home(Home::Project) => Focus::Home(Home::Devices),
                Focus::Home(Home::Runners) => Focus::Home(Home::Project),
                Focus::Home(Home::Devices) => Focus::Home(Home::Runners),
                _ => state.focus,
            },
            ..state
        },
        Action::NextDevToolsTab => State {
            focus: match state.focus {
                Focus::DevTools(DevTools::App) => Focus::DevTools(DevTools::Inspector),
                Focus::DevTools(DevTools::Inspector) => Focus::DevTools(DevTools::Performance),
                Focus::DevTools(DevTools::Performance) => Focus::DevTools(DevTools::Network),
                Focus::DevTools(DevTools::Network) => Focus::DevTools(DevTools::App),
                _ => state.focus,
            },
            ..state
        },
        Action::PreviousDevToolsTab => State {
            focus: match state.focus {
                Focus::DevTools(DevTools::App) => Focus::DevTools(DevTools::Network),
                Focus::DevTools(DevTools::Inspector) => Focus::DevTools(DevTools::App),
                Focus::DevTools(DevTools::Performance) => Focus::DevTools(DevTools::Inspector),
                Focus::DevTools(DevTools::Network) => Focus::DevTools(DevTools::Performance),
                _ => state.focus,
            },
            ..state
        },
        Action::EnterDevTools => {
            if state.session_id.is_none() {
                state
            } else {
                State {
                    focus: Focus::DevTools(DevTools::App),
                    ..state
                }
            }
        }
        Action::ExitDevTools => State {
            focus: Focus::Home(Home::Runners),
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
            },
            ..state
        },
        Action::ShowSelectDevicePopUp => State {
            popup: Some(PopUp::SelectDevice),
            select_device_popup: SelectDevicePopupState {
                selected_device: AvailableDevicesSelector
                    .select(&state)
                    .first()
                    .map(|d| d.to_owned()),
            },
            ..state
        },
        Action::HideSelectDevicePopUp => State {
            popup: None,
            ..state
        },
        Action::ShowSelectFlavorPopUp => State {
            popup: Some(PopUp::SelectDevice),
            select_flavor_popup: SelectFlavorPopupState {
                selected_flavor: None,
            },
            ..state
        },
        Action::HideSelectFlavorPopUp => State {
            focus: Focus::Home(Home::Runners),
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
                        let logs = if s.logs.iter().any(|log| {
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
                        };
                        SessionState {
                            // If the selected log is the last one in the previous requests or None,
                            // then select the last one in the new log.
                            // Otherwise, keep the selected log.
                            selected_log_index: {
                                if let Some(selected_log_index) = s.selected_log_index {
                                    if selected_log_index == logs.len() as u64 - 2 {
                                        Some(logs.len() as u64 - 1)
                                    } else {
                                        s.selected_log_index
                                    }
                                } else {
                                    Some(logs.len() as u64 - 1)
                                }
                            },
                            logs,
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
                        let logs = [s.logs, vec![SessionLog::Stdout(line.clone())]].concat();
                        SessionState {
                            // If the selected log is the last one in the previous requests or None,
                            // then select the last one in the new log.
                            // Otherwise, keep the selected log.
                            selected_log_index: {
                                if let Some(selected_log_index) = s.selected_log_index {
                                    if selected_log_index == logs.len() as u64 - 2 {
                                        Some(logs.len() as u64 - 1)
                                    } else {
                                        s.selected_log_index
                                    }
                                } else {
                                    Some(logs.len() as u64 - 1)
                                }
                            },
                            logs,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::AppendStderrLog { session_id, line } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        let logs = [s.logs, vec![SessionLog::Stderr(line.clone())]].concat();
                        SessionState {
                            // If the selected log is the last one in the previous requests or None,
                            // then select the last one in the new log.
                            // Otherwise, keep the selected log.
                            selected_log_index: {
                                if let Some(selected_log_index) = s.selected_log_index {
                                    if selected_log_index == logs.len() as u64 - 2 {
                                        Some(logs.len() as u64 - 1)
                                    } else {
                                        s.selected_log_index
                                    }
                                } else {
                                    Some(logs.len() as u64 - 1)
                                }
                            },
                            logs,
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
        Action::AppendHttpProfileRequest {
            session_id,
            requests,
        } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        // If new request is already in the previous requests, then update the previous request.
                        let mut requests = [
                            s.requests
                                .iter()
                                .filter(|r| requests.iter().all(|next_r| next_r.id != r.id))
                                .cloned()
                                .collect::<Vec<_>>(),
                            requests.clone(),
                        ]
                        .concat();
                        requests.sort_by(|a, b| a.start_time.cmp(&b.start_time));
                        SessionState {
                            requests: requests.clone(),
                            // If the selected request is the last one in the previous requests or None,
                            // then select the last one in the new requests.
                            // Otherwise, keep the selected request.
                            selected_request_id: {
                                match s.selected_request_id.clone() {
                                    Some(selected_request_id) => {
                                        if s.requests.last().map(|r| r.id.clone())
                                            == Some(selected_request_id.clone())
                                        {
                                            requests.last().map(|r| r.id.clone())
                                        } else {
                                            s.selected_request_id
                                        }
                                    }
                                    None => requests.last().map(|r| r.id.clone()),
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
        Action::NextLog => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if Some(s.id.clone()) == state.session_id {
                        SessionState {
                            selected_log_index: {
                                let logs = s.logs.clone();
                                if let Some(selected_log_index) = s.selected_log_index {
                                    if selected_log_index + 1 < logs.len() as u64 {
                                        Some(selected_log_index + 1)
                                    } else {
                                        Some(logs.len() as u64 - 1)
                                    }
                                } else {
                                    Some(0)
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
        Action::PreviousLog => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if Some(s.id.clone()) == state.session_id {
                        SessionState {
                            selected_log_index: {
                                let logs = s.logs.clone();
                                if let Some(selected_log_index) = s.selected_log_index {
                                    if selected_log_index > 0 {
                                        Some(selected_log_index - 1)
                                    } else {
                                        Some(0)
                                    }
                                } else {
                                    Some(0)
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
        Action::NextFrame => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if Some(s.id.clone()) == state.session_id {
                        SessionState {
                            selected_frame_number: {
                                let frames = s.frames.iter().map(|f| f.number).collect::<Vec<_>>();
                                if let Some(selected_frame_number) = s.selected_frame_number {
                                    if let Some(index) =
                                        frames.iter().position(|n| n == &selected_frame_number)
                                    {
                                        let next_index = if index + 1 < frames.len() {
                                            index + 1
                                        } else {
                                            frames.len() - 1
                                        };
                                        Some(frames[next_index])
                                    } else {
                                        frames.first().map(|n| n.to_owned())
                                    }
                                } else {
                                    frames.first().map(|n| n.to_owned())
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
        Action::PreviousFrame => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if Some(s.id.clone()) == state.session_id {
                        SessionState {
                            selected_frame_number: {
                                let frames = s.frames.iter().map(|f| f.number).collect::<Vec<_>>();
                                if let Some(selected_frame_number) = s.selected_frame_number {
                                    if let Some(index) =
                                        frames.iter().position(|n| n == &selected_frame_number)
                                    {
                                        let next_index = if index > 0 { index - 1 } else { 0 };
                                        Some(frames[next_index])
                                    } else {
                                        frames.last().map(|n| n.to_owned())
                                    }
                                } else {
                                    frames.last().map(|n| n.to_owned())
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
        Action::NextReqest => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if Some(s.id.clone()) == state.session_id {
                        SessionState {
                            selected_request_id: {
                                let requests =
                                    s.requests.iter().map(|r| r.id.clone()).collect::<Vec<_>>();
                                if let Some(selected_request_id) = s.selected_request_id.clone() {
                                    if let Some(index) =
                                        requests.iter().position(|id| id == &selected_request_id)
                                    {
                                        let next_index = if index + 1 < requests.len() {
                                            index + 1
                                        } else {
                                            requests.len() - 1
                                        };
                                        Some(requests[next_index].clone())
                                    } else {
                                        requests.first().map(|id| id.to_owned())
                                    }
                                } else {
                                    requests.first().map(|id| id.to_owned())
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
        Action::PreviousRequest => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if Some(s.id.clone()) == state.session_id {
                        SessionState {
                            selected_request_id: {
                                let requests =
                                    s.requests.iter().map(|r| r.id.clone()).collect::<Vec<_>>();
                                if let Some(selected_request_id) = s.selected_request_id.clone() {
                                    if let Some(index) =
                                        requests.iter().position(|id| id == &selected_request_id)
                                    {
                                        let next_index = if index > 0 { index - 1 } else { 0 };
                                        Some(requests[next_index].clone())
                                    } else {
                                        requests.last().map(|id| id.to_owned())
                                    }
                                } else {
                                    requests.last().map(|id| id.to_owned())
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
    }
}
