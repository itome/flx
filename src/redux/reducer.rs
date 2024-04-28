use std::time::{SystemTime, UNIX_EPOCH};

use redux_rs::Selector;

use crate::redux::state::{Focus, PopUp, SelectDevicePopupState, SessionLog};

use super::{
    action::Action,
    selector::{
        availale_devices::available_devices_selector,
        device_or_emulators::{self, device_or_emulators_selector, DeviceOrEmulator},
        selected_device::{self, selected_device_selector},
    },
    state::{
        DevTools, FlutterFrame, Home, SdkVersion, SelectLaunchConfigurationPopupState,
        SessionState, State,
    },
};

pub fn reducer(state: State, action: Action) -> State {
    match action {
        Action::SetSdkVersion {
            framework_version,
            channel,
            repository_url,
            framework_revision,
            framework_commit_date,
            engine_revision,
            dart_sdk_version,
            dev_tools_version,
            flutter_version,
            flutter_root,
        } => State {
            sdk_version: Some(SdkVersion {
                framework_version,
                channel,
                repository_url,
                framework_revision,
                framework_commit_date,
                engine_revision,
                dart_sdk_version,
                dev_tools_version,
                flutter_version,
                flutter_root,
            }),
            ..state
        },
        Action::AddDevice { device } => {
            let mut new_state = State {
                devices: [state.devices, vec![device.clone()]].concat(),
                selected_device_or_emulator_id: match state.selected_device_or_emulator_id {
                    Some(id) => Some(id),
                    None => Some(device.id.clone()),
                },
                ..state
            };
            let first_device = available_devices_selector(&new_state).next();

            if new_state.popup == Some(PopUp::SelectDevice)
                && new_state.select_device_popup.selected_device_id.is_none()
            {
                new_state = State {
                    select_device_popup: SelectDevicePopupState {
                        selected_device_id: first_device.map(|d| d.id.clone()),
                    },
                    ..new_state
                };
            }
            new_state
        }
        Action::RemoveDevice { device } => State {
            devices: state.devices.into_iter().filter(|d| d != &device).collect(),
            ..state
        },
        Action::SetEmultors { emulators } => State { emulators, ..state },
        Action::SetFlavors { flavors } => State { flavors, ..state },
        Action::SetLaunchConfigurations { configurations } => State {
            launch_configurations: configurations,
            ..state
        },
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
            configuration,
        } => State {
            session_id: Some(session_id.clone()),
            sessions: [
                state.sessions,
                vec![SessionState {
                    id: session_id,
                    device_id,
                    configuration,
                    display_refresh_rate: 60.0,
                    ..SessionState::default()
                }],
            ]
            .concat(),
            ..state
        },
        Action::UnregisterSession { session_id } => State {
            session_id: None,
            focus: {
                if Some(session_id.clone()) == state.session_id {
                    Focus::Home(Home::Runners)
                } else {
                    state.focus
                }
            },
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
                    if index + 1 < state.sessions.len() {
                        Some(state.sessions[index + 1].id.clone())
                    } else {
                        None
                    }
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
                    if index > 0 {
                        Some(state.sessions[index - 1].id.clone())
                    } else {
                        None
                    }
                }
                None => state.sessions.last().map(|s| s.id.clone()),
            },
            ..state
        },
        Action::NextDevice => State {
            selected_device_or_emulator_id: match state.selected_device_or_emulator_id {
                Some(ref selected_device_id) => {
                    let device_or_emulators = device_or_emulators_selector(&state);
                    let index = device_or_emulators.iter().position(|device_or_emulator| {
                        match device_or_emulator {
                            DeviceOrEmulator::Device(device) => &device.id == selected_device_id,
                            DeviceOrEmulator::Emulator(emulator) => {
                                &emulator.id == selected_device_id
                            }
                        }
                    });

                    let next_index = if let Some(index) = index {
                        (index + 1) % device_or_emulators.len()
                    } else {
                        0
                    };

                    match device_or_emulators.get(next_index) {
                        Some(DeviceOrEmulator::Device(device)) => Some(device.id.clone()),
                        Some(DeviceOrEmulator::Emulator(emulator)) => Some(emulator.id.clone()),
                        None => None,
                    }
                }
                None => match device_or_emulators_selector(&state).first() {
                    Some(DeviceOrEmulator::Device(device)) => Some(device.id.clone()),
                    Some(DeviceOrEmulator::Emulator(emulator)) => Some(emulator.id.clone()),
                    None => None,
                },
            },
            ..state
        },
        Action::PreviousDevice => State {
            selected_device_or_emulator_id: match state.selected_device_or_emulator_id {
                Some(ref selected_device_id) => {
                    let device_or_emulators = device_or_emulators_selector(&state);
                    let index = device_or_emulators.iter().position(|device_or_emulator| {
                        match device_or_emulator {
                            DeviceOrEmulator::Device(device) => &device.id == selected_device_id,
                            DeviceOrEmulator::Emulator(emulator) => {
                                &emulator.id == selected_device_id
                            }
                        }
                    });
                    let next_index = if let Some(index) = index {
                        (index + device_or_emulators.len() - 1) % device_or_emulators.len()
                    } else {
                        (device_or_emulators.len() - 1).max(0)
                    };
                    match device_or_emulators.get(next_index) {
                        Some(DeviceOrEmulator::Device(device)) => Some(device.id.clone()),
                        Some(DeviceOrEmulator::Emulator(emulator)) => Some(emulator.id.clone()),
                        None => None,
                    }
                }
                None => match device_or_emulators_selector(&state).last() {
                    Some(DeviceOrEmulator::Device(device)) => Some(device.id.clone()),
                    Some(DeviceOrEmulator::Emulator(emulator)) => Some(emulator.id.clone()),
                    None => None,
                },
            },
            ..state
        },
        Action::NextDeviceForRunning => State {
            select_device_popup: SelectDevicePopupState {
                selected_device_id: {
                    let devices = available_devices_selector(&state).collect::<Vec<_>>();
                    match &state.select_device_popup.selected_device_id {
                        Some(selected_device_id) => {
                            if let Some(index) =
                                devices.iter().position(|d| &d.id == selected_device_id)
                            {
                                let next_index = (index + 1) % devices.len();
                                devices.get(next_index).map(|d| d.id.clone())
                            } else if devices.is_empty() {
                                None
                            } else {
                                devices.first().map(|d| d.id.clone())
                            }
                        }
                        None => {
                            if devices.is_empty() {
                                None
                            } else {
                                devices.first().map(|d| d.id.clone())
                            }
                        }
                    }
                },
            },
            ..state
        },
        Action::PreviousDeviceForRunning => State {
            select_device_popup: SelectDevicePopupState {
                selected_device_id: {
                    let devices = available_devices_selector(&state).collect::<Vec<_>>();
                    match &state.select_device_popup.selected_device_id {
                        Some(selected_device_id) => {
                            if let Some(index) =
                                devices.iter().position(|d| &d.id == selected_device_id)
                            {
                                let next_index = (index + devices.len() - 1) % devices.len();
                                devices.get(next_index).map(|d| d.id.clone())
                            } else if devices.is_empty() {
                                None
                            } else {
                                devices.last().map(|d| d.id.clone())
                            }
                        }
                        None => {
                            if devices.is_empty() {
                                None
                            } else {
                                devices.last().map(|d| d.id.clone())
                            }
                        }
                    }
                },
            },
            ..state
        },
        Action::NextLaunchConfiguration => State {
            select_launch_configuration_poopup: SelectLaunchConfigurationPopupState {
                selected_index: {
                    match state.select_launch_configuration_poopup.selected_index {
                        Some(selected_index) => {
                            Some((selected_index + 1) % state.launch_configurations.len())
                        }
                        None => {
                            if state.launch_configurations.is_empty() {
                                None
                            } else {
                                Some(0)
                            }
                        }
                    }
                },
            },
            ..state
        },
        Action::PreviousLaunchConfiguration => State {
            select_launch_configuration_poopup: SelectLaunchConfigurationPopupState {
                selected_index: {
                    match state.select_launch_configuration_poopup.selected_index {
                        Some(selected_index) => {
                            if selected_index > 0 {
                                Some(selected_index - 1)
                            } else {
                                Some(0)
                            }
                        }
                        None => {
                            if state.launch_configurations.is_empty() {
                                None
                            } else {
                                Some(state.launch_configurations.len() - 1)
                            }
                        }
                    }
                },
            },
            ..state
        },
        Action::ShowSelectDevicePopUp => {
            let first_available_device = available_devices_selector(&state).next();
            State {
                popup: Some(PopUp::SelectDevice),
                select_device_popup: SelectDevicePopupState {
                    selected_device_id: first_available_device.map(|d| d.id.clone()),
                },
                ..state
            }
        }
        Action::HideSelectDevicePopUp => State {
            popup: None,
            ..state
        },
        Action::ShowSelectLaunchConfigurationPopup => State {
            popup: Some(PopUp::SelectLaunchConfiguration),
            select_launch_configuration_poopup: SelectLaunchConfigurationPopupState {
                selected_index: if state.launch_configurations.is_empty() {
                    None
                } else {
                    Some(0)
                },
            },
            ..state
        },
        Action::HideSelectLaunchConfigurationPopuup => State {
            popup: None,
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
                            selected_frame_number: {
                                if s.selected_frame_number == s.frames.last().map(|f| f.number) {
                                    Some(number)
                                } else {
                                    s.selected_frame_number
                                }
                            },
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
                                            && state.focus
                                                != Focus::DevTools(DevTools::NetworkRequest)
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
        Action::AppendHttpProfileFullRequest {
            session_id,
            request,
        } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|mut s| {
                    if s.id == session_id {
                        s.full_requests.insert(request.id.clone(), request.clone());
                        SessionState {
                            full_requests: s.full_requests,
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
        Action::EnterNetworkRequest => State {
            focus: Focus::DevTools(DevTools::NetworkRequest),
            ..state
        },
        Action::ExitNetworkRequest => State {
            focus: Focus::DevTools(DevTools::Network),
            ..state
        },
        Action::SetWidgetSummaryTree { session_id, tree } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            widget_summary_tree: Some(tree.clone()),
                            selected_widget_value_id: tree.value_id.clone(),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SelectWidgetValueId { session_id, id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            selected_widget_value_id: Some(id.clone()),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SetOpenWidgetValueId { session_id, ids } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            opened_widget_value_ids: ids.clone(),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::ToggleOpenWidgetValueId { session_id, id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        let mut opened_widget_value_ids = s.opened_widget_value_ids.clone();
                        if opened_widget_value_ids.contains(&id) {
                            opened_widget_value_ids.remove(&id);
                        } else {
                            opened_widget_value_ids.insert(id.clone());
                        }
                        SessionState {
                            opened_widget_value_ids,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SetSelectedWidgetDetailsTree { session_id, tree } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            selected_widget_details_tree: tree.clone(),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SetSelectedWidgetObjectGroup { session_id, group } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            selected_widget_object_group: group.clone(),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::SetOpenWidgetDetailsValueId { session_id, ids } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        SessionState {
                            opened_widget_details_value_ids: ids.clone(),
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::ToggleOpenWidgetDetailsValueId { session_id, id } => State {
            sessions: state
                .sessions
                .into_iter()
                .map(|s| {
                    if s.id == session_id {
                        let mut opened_widget_value_ids = s.opened_widget_details_value_ids.clone();
                        if opened_widget_value_ids.contains(&id) {
                            opened_widget_value_ids.remove(&id);
                        } else {
                            opened_widget_value_ids.insert(id.clone());
                        }
                        SessionState {
                            opened_widget_details_value_ids: opened_widget_value_ids,
                            ..s
                        }
                    } else {
                        s
                    }
                })
                .collect(),
            ..state
        },
        Action::EnterWidgetDetails => State {
            focus: Focus::DevTools(DevTools::WidgetDetails),
            ..state
        },
        Action::ExitWidgetDetails => State {
            focus: Focus::DevTools(DevTools::Inspector),
            ..state
        },
    }
}
