use std::collections::HashMap;
use std::time::Duration;

use daemon::io::{device::Device, event::AppMode};

use super::state::Mode;

#[derive(Debug)]
pub enum Action {
    SetMode {
        mode: Mode,
    },
    AddDevice {
        device: Device,
    },
    RemoveDevice {
        device: Device,
    },

    SetFlavors {
        flavors: HashMap<String, Vec<String>>,
    },

    NextTab,
    PreviousTab,

    RegisterSession {
        session_id: String,
        device_id: Option<String>,
        flavor: Option<String>,
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

    ShowSelectFlavorPopUp,
    HideSelectFlavorPopUp,

    NextFlavorForRunning,
    PreviousFlavorForRunning,

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

    AppendStderrLog {
        session_id: String,
        line: String,
    },

    AppendFlutterFrame {
        session_id: String,
        build: Duration,
        elapsed: Duration,
        number: u64,
        raster: Duration,
        start_time: Duration,
        vsync_overhead: Duration,
    },

    SetDisplayRefreshRate {
        session_id: String,
        rate: f32,
    },
}
