use std::time::Duration;

use crate::daemon::io::{device::Device, event::AppMode};

#[derive(Clone, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Project,
    Runners,
    Devices,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum PopUp {
    #[default]
    SelectDevice,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum DevTools {
    #[default]
    Logs,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Focus {
    Tab(Tab),
    PopUp(PopUp),
    DevTools(DevTools),
}

impl Default for Focus {
    fn default() -> Self {
        Focus::Tab(Tab::Runners)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum SessionLog {
    Progress {
        id: String,
        message: Option<String>,
        start_at: u128,
        end_at: Option<u128>,
    },
    Stdout(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FlutterFrame {
    pub build: Duration,
    pub elapsed: Duration,
    pub number: u64,
    pub raster: Duration,
    pub start_time: Duration,
    pub vsync_overhead: Duration,
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
    pub logs: Vec<SessionLog>,
    pub frames: Vec<FlutterFrame>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SelectDevicePopupState {
    pub visible: bool,
    pub selected_device_id: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct State {
    pub current_focus: Focus,

    pub project_root: Option<String>,
    pub devices: Vec<Device>,

    pub sessions: Vec<SessionState>,
    pub session_id: Option<String>,

    pub supported_platforms: Vec<String>,

    pub select_device_popup: SelectDevicePopupState,
}
