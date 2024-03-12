use serde::Deserialize;
use serde::Serialize;

use super::device::Device;
use super::emulator::Emulator;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlutterDaemonResponse<R> {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetSupportedPlatformsResult {
    pub platforms: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServeDevToolsResult {
    host: Option<String>,
    port: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartAppResult {
    pub code: u32,
    pub message: String,
}

pub type VersionResponse = FlutterDaemonResponse<String>;

pub type ShutdownResponse = FlutterDaemonResponse<()>;

pub type GetSupportedPlatformsResponse = FlutterDaemonResponse<GetSupportedPlatformsResult>;

pub type GetDevicesResponse = FlutterDaemonResponse<Vec<Device>>;

pub type DeviceEnableResponse = FlutterDaemonResponse<()>;

pub type DeviceDisableResponse = FlutterDaemonResponse<()>;

pub type DeviceForwardResponse = FlutterDaemonResponse<()>;

pub type DeviceUnforwardResponse = FlutterDaemonResponse<()>;

pub type GetEmulatorsResponse = FlutterDaemonResponse<Vec<Emulator>>;

pub type EmulatorLaunchResponse = FlutterDaemonResponse<()>;

pub type EmulatorCreateResponse = FlutterDaemonResponse<()>;

pub type ServeDevToolsResponse = FlutterDaemonResponse<ServeDevToolsResult>;

pub type RestartAppResponse = FlutterDaemonResponse<RestartAppResult>;

pub type StopAppResponse = FlutterDaemonResponse<bool>;

pub type DetachAppResponse = FlutterDaemonResponse<bool>;

#[cfg(test)]
mod test {
    use crate::io::device::DeviceCapabilities;

    use super::*;

    #[test]
    fn verseion_response() {
        let json = r#"{"id": 1, "result": "0.6.1"}"#;
        let response: VersionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            VersionResponse {
                id: 1,
                result: Some("0.6.1".to_string()),
            }
        );
    }

    #[test]
    fn shutdown_response() {
        let json = r#"{"id": 1}"#;
        let response: ShutdownResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            ShutdownResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn get_supported_platforms_response() {
        let json =
            r#"{"id":1,"result":{"platforms":["linux","macos","windows","ios","android","web"]}}"#;
        let response: GetSupportedPlatformsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            GetSupportedPlatformsResponse {
                id: 1,
                result: Some(GetSupportedPlatformsResult {
                    platforms: vec![
                        "linux".to_string(),
                        "macos".to_string(),
                        "windows".to_string(),
                        "ios".to_string(),
                        "android".to_string(),
                        "web".to_string(),
                    ],
                }),
            }
        );
    }

    #[test]
    fn get_devices_response() {
        let json = r#"{"id":1,"result":[{"id":"linux","name":"Linux","platform":"linux","emulator":false,"category":"mobile","platformType":"desktop","ephemeral":false,"capabilities":{"hotReload":true,"hotRestart":true,"screenshot":true,"fastStart":true,"flutterExit":true,"hardwareRendering":true,"startPaused":false},"sdk":"Flutter (Channel stable, 2.0.3, on Linux, locale en_US.UTF-8)","emulatorId":"linux"}]}"#;
        let response: GetDevicesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            GetDevicesResponse {
                id: 1,
                result: Some(vec![Device {
                    id: "linux".to_string(),
                    name: "Linux".to_string(),
                    platform: "linux".to_string(),
                    emulator: false,
                    category: "mobile".to_string(),
                    platform_type: "desktop".to_string(),
                    ephemeral: false,
                    emulator_id: Some("linux".to_string()),
                    sdk: "Flutter (Channel stable, 2.0.3, on Linux, locale en_US.UTF-8)"
                        .to_string(),
                    capabilities: DeviceCapabilities {
                        hot_reload: true,
                        hot_restart: true,
                        screenshot: true,
                        fast_start: true,
                        flutter_exit: true,
                        hardware_rendering: true,
                        start_paused: false,
                    },
                }]),
            }
        );
    }

    #[test]
    fn device_enable_response() {
        let json = r#"{"id":1}"#;
        let response: DeviceEnableResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            DeviceEnableResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn device_disable_response() {
        let json = r#"{"id":1}"#;
        let response: DeviceDisableResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            DeviceDisableResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn device_forward_response() {
        let json = r#"{"id":1}"#;
        let response: DeviceForwardResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            DeviceForwardResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn device_unforward_response() {
        let json = r#"{"id":1}"#;
        let response: DeviceUnforwardResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            DeviceUnforwardResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn get_emulators_response() {
        let json = r#"{"id":1,"result":[{"id":"android","name":"Android SDK built for x86","category":"mobile","platformType":"android"}]}"#;
        let response: GetEmulatorsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            GetEmulatorsResponse {
                id: 1,
                result: Some(vec![Emulator {
                    id: "android".to_string(),
                    name: "Android SDK built for x86".to_string(),
                    category: "mobile".to_string(),
                    platform_type: "android".to_string(),
                }]),
            }
        );
    }

    #[test]
    fn emulator_launch_response() {
        let json = r#"{"id":1}"#;
        let response: EmulatorLaunchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            EmulatorLaunchResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn emulator_create_response() {
        let json = r#"{"id":1}"#;
        let response: EmulatorCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            EmulatorCreateResponse {
                id: 1,
                result: None
            }
        );
    }

    #[test]
    fn serve_dev_tools_response() {
        let json = r#"{"id":1,"result":{"host":"somehost","port":"1234"}}"#;
        let response: ServeDevToolsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            ServeDevToolsResponse {
                id: 1,
                result: Some(ServeDevToolsResult {
                    host: Some("somehost".to_string()),
                    port: Some("1234".to_string())
                })
            }
        )
    }

    #[test]
    fn restart_app_response() {
        let json = r#"{"id":1,"result":{"code":0,"message":"Success"}}"#;
        let response: RestartAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            RestartAppResponse {
                id: 1,
                result: Some(RestartAppResult {
                    code: 0,
                    message: "Success".to_string(),
                })
            }
        )
    }

    #[test]
    fn stop_app_response() {
        let json = r#"{"id":1,"result":true}"#;
        let response: StopAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            StopAppResponse {
                id: 1,
                result: Some(true)
            }
        )
    }

    #[test]
    fn detach_app_response() {
        let json = r#"{"id":1,"result":true}"#;
        let response: DetachAppResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            DetachAppResponse {
                id: 1,
                result: Some(true)
            }
        )
    }
}
