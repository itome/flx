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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SelectDevicePopupState {
    pub visible: bool,
    pub selected_device_id: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct State {
    pub project_root: Option<String>,
    pub devices: Vec<Device>,

    pub sessions: Vec<SessionState>,
    pub session_id: Option<String>,

    pub supported_platforms: Vec<String>,

    pub selected_tab: Tab,

    pub select_device_popup: SelectDevicePopupState,
}
