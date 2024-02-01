use crate::daemon::io::{device::Device, event::AppMode};

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
    },
    UnregisterSession {
        session_id: String,
    },

    NextSession,
    PreviousSession,

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
}
