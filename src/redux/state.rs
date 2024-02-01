use crate::daemon::io::{device::Device, event::AppMode};

#[derive(Clone, PartialEq, Eq)]
pub enum Tab {
    Project,
    Runners,
    Devices,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Project
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SessionState {
    pub id: String,
    pub app_id: Option<String>,
    pub device_id: Option<String>,
    pub started: bool,
    pub mode: Option<AppMode>,
    pub hot_reloading: bool,
    pub hot_restarting: bool,
}

impl SessionState {
    pub fn new(id: String) -> Self {
        Self {
            id,
            device_id: None,
            app_id: None,
            mode: None,
            started: false,
            hot_reloading: false,
            hot_restarting: false,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct State {
    pub devices: Vec<Device>,
    pub selected_tab: Tab,

    pub sessions: Vec<SessionState>,
    pub session_id: Option<String>,
}
