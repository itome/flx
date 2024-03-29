use std::collections::HashMap;
use std::time::Duration;

use daemon::io::{device::Device, event::AppMode};

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
    SelectFlavor,
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
    Stderr(String),
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

#[derive(Default, Clone, PartialEq)]
pub struct SessionState {
    pub id: String,
    pub app_id: Option<String>,
    pub device_id: Option<String>,
    pub flavor: Option<String>,
    pub started: bool,
    pub mode: Option<AppMode>,
    pub hot_reloading: bool,
    pub hot_restarting: bool,
    pub logs: Vec<SessionLog>,
    pub frames: Vec<FlutterFrame>,
    pub display_refresh_rate: f32,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SelectDevicePopupState {
    pub visible: bool,
    pub selected_device: Option<Device>,
}

impl SelectDevicePopupState {
    pub fn selected_device_platform(&self) -> Option<String> {
        Some(self.selected_device.clone()?.platform)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct SelectFlavorPopupState {
    pub visible: bool,
    pub selected_flavor: Option<String>,
}

#[derive(Default, Clone, PartialEq)]
pub struct State {
    pub current_focus: Focus,

    pub project_root: Option<String>,
    pub devices: Vec<Device>,

    pub flavors: HashMap<String, Vec<String>>,

    pub sessions: Vec<SessionState>,
    pub session_id: Option<String>,

    pub supported_platforms: Vec<String>,

    pub select_device_popup: SelectDevicePopupState,
    pub select_flavor_popup: SelectFlavorPopupState,
}
