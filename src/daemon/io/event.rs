use serde::Deserialize;
use serde::Serialize;

use super::device::{Device, DeviceCapabilities};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConnectedEventParams {
    pub version: String,
    pub pid: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LogEventParams {
    pub log: String,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageLevel {
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowMessageEventParams {
    pub level: MessageLevel,
    pub title: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LogMessageEventParams {
    pub level: MessageLevel,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppLaunchMode {
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "attach")]
    Attach,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppMode {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "jit_release")]
    JitRelease,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppStartEventParams {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub directory: String,
    #[serde(rename = "supportsRestart")]
    pub supports_restart: bool,
    #[serde(rename = "launchMode")]
    pub launch_mode: AppLaunchMode,
    pub mode: AppMode,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppDebugPortEventParams {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "wsUri")]
    pub ws_uri: String,
    #[serde(rename = "baseUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_uri: Option<String>,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppStartedEventParams {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppLogEventParams {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub log: String,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppProgressEventParams {
    pub id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "progressId")]
    pub progress_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub finished: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppStopEventParams {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppWebLaunchUrlEventParams {
    pub url: String,
    pub launched: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceAddedEventParams {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceRemovedEventParams {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "event")]
pub enum FlutterDaemonEvent {
    #[serde(rename = "daemon.connected")]
    Connected { params: ConnectedEventParams },

    #[serde(rename = "daemon.log")]
    Log { params: LogEventParams },

    #[serde(rename = "daemon.showMessage")]
    ShowMessage { params: ShowMessageEventParams },

    #[serde(rename = "daemon.logMessage")]
    LogMessage { params: LogMessageEventParams },

    #[serde(rename = "app.start")]
    AppStart { params: AppStartEventParams },

    #[serde(rename = "app.debugPort")]
    AppDebugPort { params: AppDebugPortEventParams },

    #[serde(rename = "app.started")]
    AppStarted { params: AppStartedEventParams },

    #[serde(rename = "app.log")]
    AppLog { params: AppLogEventParams },

    #[serde(rename = "app.progress")]
    AppProgress { params: AppProgressEventParams },

    #[serde(rename = "app.stop")]
    AppStop { params: AppStopEventParams },

    #[serde(rename = "app.webLaunchUrl")]
    AppWebLaunchUrl { params: AppWebLaunchUrlEventParams },

    #[serde(rename = "device.added")]
    DeviceAdded { params: Device },

    #[serde(rename = "device.removed")]
    DeviceRemoved { params: Device },
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deserialize_connected_event() {
        let json = r#"{"event":"daemon.connected","params":{"version":"2.0.0","pid": 1234}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::Connected {
                params: super::ConnectedEventParams {
                    version: "2.0.0".to_string(),
                    pid: 1234
                }
            }
        );
    }

    #[test]
    fn test_deserialize_log_event() {
        let json = r#"{"event":"daemon.log","params":{"log":"Hello, world!","error":false}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::Log {
                params: super::LogEventParams {
                    log: "Hello, world!".to_string(),
                    error: false
                }
            }
        );
    }

    #[test]
    fn test_deserialize_show_message_event() {
        let json = r#"{"event":"daemon.showMessage","params":{"level":"info","title":"Hello, world!","message":"Hello, world!"}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::ShowMessage {
                params: super::ShowMessageEventParams {
                    level: super::MessageLevel::Info,
                    title: "Hello, world!".to_string(),
                    message: "Hello, world!".to_string()
                }
            }
        );
    }

    #[test]
    fn test_deserialize_log_message_event() {
        let json =
            r#"{"event":"daemon.logMessage","params":{"level":"info","message":"Hello, world!"}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::LogMessage {
                params: super::LogMessageEventParams {
                    level: super::MessageLevel::Info,
                    message: "Hello, world!".to_string(),
                    stack_trace: None
                }
            }
        );
    }

    #[test]
    fn test_deserialize_app_start_event() {
        let json = r#"{"event":"app.start","params":{"appId":"com.example.app","deviceId":"1234","directory":"/path/to/app","supportsRestart":true,"launchMode":"run","mode":"debug"}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppStart {
                params: super::AppStartEventParams {
                    app_id: "com.example.app".to_string(),
                    device_id: "1234".to_string(),
                    directory: "/path/to/app".to_string(),
                    supports_restart: true,
                    launch_mode: super::AppLaunchMode::Run,
                    mode: super::AppMode::Debug
                }
            }
        );
    }

    #[test]
    fn test_deserialize_app_debug_port_event() {
        let json = r#"{"event":"app.debugPort","params":{"appId":"com.example.app","wsUri":"ws://127.0.0.1:55742/vRaYiJQQ4pU=/ws","baseUri":"/path/to/app","port":1234}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppDebugPort {
                params: super::AppDebugPortEventParams {
                    app_id: "com.example.app".to_string(),
                    ws_uri: "ws://127.0.0.1:55742/vRaYiJQQ4pU=/ws".to_string(),
                    base_uri: Some("/path/to/app".to_string()),
                    port: 1234
                }
            }
        )
    }

    #[test]
    fn test_deserialize_app_started_event() {
        let json = r#"{"event":"app.started","params":{"appId":"com.example.app"}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppStarted {
                params: super::AppStartedEventParams {
                    app_id: "com.example.app".to_string()
                }
            }
        );
    }

    #[test]
    fn test_deserialize_app_log_event() {
        let json = r#"{"event":"app.log","params":{"appId":"com.example.app","log":"Hello, world!","error":false}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppLog {
                params: super::AppLogEventParams {
                    app_id: "com.example.app".to_string(),
                    log: "Hello, world!".to_string(),
                    error: false
                }
            }
        );
    }

    #[test]
    fn test_deserialize_app_progress_event() {
        let json = r#"{"event":"app.progress","params":{"id":"1234","appId":"com.example.app","progressId":"1234","message":"Hello, world!","finished":false}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppProgress {
                params: super::AppProgressEventParams {
                    id: "1234".to_string(),
                    app_id: "com.example.app".to_string(),
                    progress_id: Some("1234".to_string()),
                    message: Some("Hello, world!".to_string()),
                    finished: false
                }
            }
        );
    }

    #[test]
    fn test_deserialize_app_stop_event() {
        let json = r#"{"event":"app.stop","params":{"appId":"com.example.app"}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppStop {
                params: super::AppStopEventParams {
                    app_id: "com.example.app".to_string()
                }
            }
        );
    }

    #[test]
    fn test_deserialize_app_web_launch_url_event() {
        let json = r#"{"event":"app.webLaunchUrl","params":{"url":"http://localhost:3000","launched":true}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::AppWebLaunchUrl {
                params: super::AppWebLaunchUrlEventParams {
                    url: "http://localhost:3000".to_string(),
                    launched: true
                }
            }
        );
    }

    #[test]
    fn test_deserialize_device_added_event() {
        let json = r#"{"event":"device.added","params":{"id":"macos","name":"macOS","platform":"darwin","emulator":false,"category":"desktop","platformType":"macos","ephemeral":false,"emulatorId":null,"sdk":"macOS 14.1.2 23B92 darwin-arm64","capabilities":{"hotReload":true,"hotRestart":true,"screenshot":false,"fastStart":false,"flutterExit":true,"hardwareRendering":true,"startPaused":true}}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::DeviceAdded {
                params: super::Device {
                    id: "macos".to_string(),
                    name: "macOS".to_string(),
                    platform: "darwin".to_string(),
                    emulator: false,
                    category: "desktop".to_string(),
                    platform_type: "macos".to_string(),
                    ephemeral: false,
                    emulator_id: None,
                    sdk: "macOS 14.1.2 23B92 darwin-arm64".to_string(),
                    capabilities: super::DeviceCapabilities {
                        hot_reload: true,
                        hot_restart: true,
                        screenshot: false,
                        fast_start: false,
                        flutter_exit: true,
                        hardware_rendering: true,
                        start_paused: true
                    }
                }
            }
        );
    }

    #[test]
    fn test_deserialize_device_removed_event() {
        let json = r#"{"event":"device.removed","params":{"id":"macos","name":"macOS","platform":"darwin","emulator":false,"category":"desktop","platformType":"macos","ephemeral":false,"emulatorId":null,"sdk":"macOS 14.1.2 23B92 darwin-arm64","capabilities":{"hotReload":true,"hotRestart":true,"screenshot":false,"fastStart":false,"flutterExit":true,"hardwareRendering":true,"startPaused":true}}}"#;
        let event: super::FlutterDaemonEvent = serde_json::from_str(json).unwrap();
        assert_eq!(
            event,
            super::FlutterDaemonEvent::DeviceRemoved {
                params: super::Device {
                    id: "macos".to_string(),
                    name: "macOS".to_string(),
                    platform: "darwin".to_string(),
                    emulator: false,
                    category: "desktop".to_string(),
                    platform_type: "macos".to_string(),
                    ephemeral: false,
                    emulator_id: None,
                    sdk: "macOS 14.1.2 23B92 darwin-arm64".to_string(),
                    capabilities: super::DeviceCapabilities {
                        hot_reload: true,
                        hot_restart: true,
                        screenshot: false,
                        fast_start: false,
                        flutter_exit: true,
                        hardware_rendering: true,
                        start_paused: true
                    }
                }
            }
        );
    }
}
