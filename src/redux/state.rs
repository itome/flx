use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use std::{collections::HashMap, time::SystemTime};

use daemon::io::emulator::Emulator;
use daemon::io::{device::Device, event::AppMode};
use devtools::protocols::flutter_extension::DiagnosticNode;
use devtools::protocols::io_extension::{HttpProfileRequest, HttpProfileRequestRef};

#[derive(Clone, PartialEq, Eq, Default)]
pub enum Home {
    Project,
    #[default]
    Runners,
    Devices,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum PopUp {
    #[default]
    SelectDevice,
    SelectLaunchConfiguration,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum DevTools {
    #[default]
    App,
    Performance,
    Inspector,
    WidgetDetails,
    Network,
    NetworkRequest,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Focus {
    Home(Home),
    DevTools(DevTools),
}

impl Default for Focus {
    fn default() -> Self {
        Focus::Home(Home::Runners)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SdkVersion {
    pub framework_version: String,
    pub channel: String,
    pub repository_url: String,
    pub framework_revision: String,
    pub framework_commit_date: String,
    pub engine_revision: String,
    pub dart_sdk_version: String,
    pub dev_tools_version: String,
    pub flutter_version: String,
    pub flutter_root: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LaunchConfiguration {
    pub name: String,
    pub program: Option<String>,
    pub args: Option<Vec<String>>,
    pub cwd: Option<String>,
    pub flutter_mode: Option<String>,
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
    pub configuration: Option<LaunchConfiguration>,
    pub started: bool,
    pub stopped: bool,
    pub mode: Option<AppMode>,
    pub hot_reloading: bool,
    pub hot_restarting: bool,
    pub logs: Vec<SessionLog>,
    pub frames: Vec<FlutterFrame>,
    pub requests: Vec<HttpProfileRequestRef>,
    pub full_requests: HashMap<String, HttpProfileRequest>,
    pub selected_log_index: Option<u64>,
    pub selected_frame_number: Option<u64>,
    pub selected_request_id: Option<String>,
    pub display_refresh_rate: f32,
    pub widget_summary_tree: Option<DiagnosticNode>,
    pub selected_widget_value_id: Option<String>,
    pub opened_widget_value_ids: HashSet<String>,
    pub selected_widget_object_group: Option<String>,
    pub selected_widget_details_tree: Option<DiagnosticNode>,
    pub opened_widget_details_value_ids: HashSet<String>,

    pub debug_paint_enabled: bool,
    pub slow_animations_enabled: bool,
    pub debug_paint_baselines_enabled: bool,
    pub repaint_rainbow_enabled: bool,
    pub invert_oversized_images_enabled: bool,
    pub show_performance_overlay_enabled: bool,
    pub show_widget_inspector_enabled: bool,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SelectDevicePopupState {
    pub selected_device_id: Option<String>,
}

#[derive(Default, Clone, PartialEq)]
pub struct SelectLaunchConfigurationPopupState {
    pub selected_index: Option<usize>,
}

#[derive(Default, Clone, PartialEq)]
pub struct State {
    pub focus: Focus,
    pub popup: Option<PopUp>,

    pub sdk_version: Option<SdkVersion>,

    pub project_root: PathBuf,
    pub devices: Vec<Device>,
    pub emulators: Vec<Emulator>,
    pub selected_device_or_emulator_id: Option<String>,
    pub launch_configurations: Vec<LaunchConfiguration>,
    pub supported_platforms: HashMap<PathBuf, Vec<String>>,

    pub sessions: Vec<SessionState>,
    pub session_id: Option<String>,

    pub select_device_popup: SelectDevicePopupState,
    pub select_launch_configuration_poopup: SelectLaunchConfigurationPopupState,
}

impl State {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            ..Default::default()
        }
    }
}
