use crate::daemon::io::{device::Device, event::AppMode};

#[derive(Debug)]
pub enum Action {
    AddDevice {
        device: Device,
    },
    RemoveDevice {
        device: Device,
    },

    NextTab,
    PreviousTab,

    RegisterSession {
        session_id: String,
        device_id: Option<String>,
    },
    UnregisterSession {
        session_id: String,
    },

    SetProjectRoot {
        project_root: Option<String>,
    },

    SetSupportedPlatforms {
        platforms: Vec<String>,
    },

    NextSession,
    PreviousSession,

    NextDeviceForRunning,
    PreviousDeviceForRunning,

    ShowSelectDevicePopUp,
    HideSelectDevicePopUp,

    StartApp {
        session_id: String,
        device_id: String,
        app_id: String,
        mode: AppMode,
    },

    SetAppStarted {
        session_id: String,
    },

    StartHotReload {
        session_id: String,
    },
    CompleteHotReload {
        session_id: String,
    },

    StartHotRestart {
        session_id: String,
    },

    CompleteHotRestart {
        session_id: String,
    },

    AppendProgressLog {
        session_id: String,
        id: String,
        finished: bool,
        message: Option<String>,
    },

    AppendStdoutLog {
        session_id: String,
        line: String,
    },
}
