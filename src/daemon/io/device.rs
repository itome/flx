use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    #[serde(rename = "hotReload")]
    pub hot_reload: bool,
    #[serde(rename = "hotRestart")]
    pub hot_restart: bool,
    #[serde(rename = "screenshot")]
    pub screenshot: bool,
    #[serde(rename = "fastStart")]
    pub fast_start: bool,
    #[serde(rename = "flutterExit")]
    pub flutter_exit: bool,
    #[serde(rename = "hardwareRendering")]
    pub hardware_rendering: bool,
    #[serde(rename = "startPaused")]
    pub start_paused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub emulator: bool,
    pub category: String,
    #[serde(rename = "platformType")]
    pub platform_type: String,
    pub ephemeral: bool,
    #[serde(rename = "emulatorId")]
    pub emulator_id: Option<String>,
    pub sdk: String,
    pub capabilities: DeviceCapabilities,
}
