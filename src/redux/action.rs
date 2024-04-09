use std::collections::HashMap;
use std::time::Duration;

use daemon::io::{device::Device, emulator::Emulator, event::AppMode};
use devtools::protocols::{
    flutter_extension::DiagnosticNode,
    io_extension::{HttpProfileRequest, HttpProfileRequestRef},
};

#[derive(Debug)]
pub enum Action {
    AddDevice {
        device: Device,
    },
    RemoveDevice {
        device: Device,
    },
    SetEmultors {
        emulators: Vec<Emulator>,
    },

    SetFlavors {
        flavors: HashMap<String, Vec<String>>,
    },

    NextHomeTab,
    PreviousHomeTab,
    NextDevToolsTab,
    PreviousDevToolsTab,

    EnterDevTools,
    ExitDevTools,

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

    NextDevice,
    PreviousDevice,

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

    AppendHttpProfileRequest {
        session_id: String,
        requests: Vec<HttpProfileRequestRef>,
    },

    AppendHttpProfileFullRequest {
        session_id: String,
        request: HttpProfileRequest,
    },

    SetDisplayRefreshRate {
        session_id: String,
        rate: f32,
    },

    SetWidgetSummaryTree {
        session_id: String,
        tree: DiagnosticNode,
    },

    NextLog,
    PreviousLog,

    NextFrame,
    PreviousFrame,

    NextReqest,
    PreviousRequest,

    EnterNetworkRequest,
    ExitNetworkRequest,
}
