use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetSupportedPlatformsParams {
    #[serde(rename = "projectRoot")]
    pub project_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LaunchEmulatorParams {
    #[serde(rename = "emulatorId")]
    pub emulator_id: String,

    #[serde(rename = "coldBoot")]
    pub cold_boot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateEmultorParams {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RestartAppParams {
    #[serde(rename = "appId")]
    pub app_id: String,

    #[serde(rename = "fullRestart")]
    pub full_restart: bool,

    pub pause: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debounce: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DetachAppParams {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StopAppParams {
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallServiceExtensionParams {
    #[serde(rename = "appId")]
    pub app_id: String,

    #[serde(rename = "methodName")]
    pub method_name: String,

    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Map<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceForwardParams {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "port")]
    pub port: u32,

    #[serde(rename = "hostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceUnforwardParams {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "port")]
    pub port: u32,

    #[serde(rename = "hostPort")]
    pub host_port: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
pub enum FlutterDaemonRequest {
    #[serde(rename = "daemon.version")]
    Version { id: u32 },

    #[serde(rename = "daemon.shutdown")]
    Shutdown { id: u32 },

    #[serde(rename = "daemon.getSupportedPlatforms")]
    GetSupportedPlatforms {
        id: u32,
        params: GetSupportedPlatformsParams,
    },

    #[serde(rename = "device.getDevices")]
    GetDevices { id: u32 },

    #[serde(rename = "device.enable")]
    DeviceEnable { id: u32 },

    #[serde(rename = "device.disable")]
    DeviceDisable { id: u32 },

    #[serde(rename = "device.forward")]
    DeviceForward {
        id: u32,
        params: DeviceForwardParams,
    },

    #[serde(rename = "device.unforward")]
    DeviceUnforward {
        id: u32,
        params: DeviceUnforwardParams,
    },

    #[serde(rename = "emulator.getEmulators")]
    GetEmulators { id: u32 },

    #[serde(rename = "emulator.launch")]
    LaunchEmulator {
        id: u32,
        params: LaunchEmulatorParams,
    },

    #[serde(rename = "emulator.create")]
    CreateEmulator {
        id: u32,
        params: CreateEmultorParams,
    },

    #[serde(rename = "devtools.serve")]
    ServeDevtools { id: u32 },

    #[serde(rename = "app.restart")]
    RestartApp { id: u32, params: RestartAppParams },

    #[serde(rename = "app.detach")]
    DetachApp { id: u32, params: DetachAppParams },

    #[serde(rename = "app.stop")]
    StopApp { id: u32, params: StopAppParams },

    #[serde(rename = "app.callServiceExtension")]
    CallServiceExtension {
        id: u32,
        params: CallServiceExtensionParams,
    },
}

#[cfg(test)]
mod tests {
    use super::FlutterDaemonRequest;

    #[test]
    fn daemon_version() {
        let method = FlutterDaemonRequest::Version { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"daemon.version","id":1}"#);
    }

    #[test]
    fn daemon_shutdown() {
        let method = FlutterDaemonRequest::Shutdown { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"daemon.shutdown","id":1}"#);
    }

    #[test]
    fn daemon_get_supported_platforms() {
        let method = FlutterDaemonRequest::GetSupportedPlatforms {
            id: 1,
            params: super::GetSupportedPlatformsParams {
                project_root: String::from("/home/username/Projects/flutter"),
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"daemon.getSupportedPlatforms","id":1,"params":{"projectRoot":"/home/username/Projects/flutter"}}"#
        );
    }

    #[test]
    fn device_get_devices() {
        let method = FlutterDaemonRequest::GetDevices { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"device.getDevices","id":1}"#);
    }

    #[test]
    fn device_enable() {
        let method = FlutterDaemonRequest::DeviceEnable { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"device.enable","id":1}"#);
    }

    #[test]
    fn device_disable() {
        let method = FlutterDaemonRequest::DeviceDisable { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"device.disable","id":1}"#);
    }

    #[test]
    fn device_forward() {
        let method = FlutterDaemonRequest::DeviceForward {
            id: 1,
            params: super::DeviceForwardParams {
                device_id: String::from("emulator-5554"),
                port: 8080,
                host_port: None,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"device.forward","id":1,"params":{"deviceId":"emulator-5554","port":8080}}"#
        );

        let method = FlutterDaemonRequest::DeviceForward {
            id: 1,
            params: super::DeviceForwardParams {
                device_id: String::from("emulator-5554"),
                port: 8080,
                host_port: Some(8081),
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"device.forward","id":1,"params":{"deviceId":"emulator-5554","port":8080,"hostPort":8081}}"#
        );
    }

    #[test]
    fn device_unforward() {
        let method = FlutterDaemonRequest::DeviceUnforward {
            id: 1,
            params: super::DeviceUnforwardParams {
                device_id: String::from("emulator-5554"),
                port: 8080,
                host_port: 8081,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"device.unforward","id":1,"params":{"deviceId":"emulator-5554","port":8080,"hostPort":8081}}"#
        );
    }

    #[test]
    fn emulator_get_emulators() {
        let method = FlutterDaemonRequest::GetEmulators { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"emulator.getEmulators","id":1}"#);
    }

    #[test]
    fn launch_emulator() {
        let method = FlutterDaemonRequest::LaunchEmulator {
            id: 1,
            params: super::LaunchEmulatorParams {
                emulator_id: String::from("emulator-5554"),
                cold_boot: false,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"emulator.launch","id":1,"params":{"emulatorId":"emulator-5554","coldBoot":false}}"#
        );
    }

    #[test]
    fn create_emulator() {
        let method = FlutterDaemonRequest::CreateEmulator {
            id: 1,
            params: super::CreateEmultorParams { name: None },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"emulator.create","id":1,"params":{}}"#
        );

        let method = FlutterDaemonRequest::CreateEmulator {
            id: 1,
            params: super::CreateEmultorParams {
                name: Some(String::from("test")),
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"emulator.create","id":1,"params":{"name":"test"}}"#
        );
    }

    #[test]
    fn devtools_serve() {
        let method = FlutterDaemonRequest::ServeDevtools { id: 1 };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, r#"{"method":"devtools.serve","id":1}"#);
    }

    #[test]
    fn restart_app() {
        let method = FlutterDaemonRequest::RestartApp {
            id: 1,
            params: super::RestartAppParams {
                app_id: String::from("com.example.app"),
                full_restart: false,
                pause: false,
                reason: None,
                debounce: None,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"app.restart","id":1,"params":{"appId":"com.example.app","fullRestart":false,"pause":false}}"#
        );
    }

    #[test]
    fn detach_app() {
        let method = FlutterDaemonRequest::DetachApp {
            id: 1,
            params: super::DetachAppParams {
                app_id: String::from("com.example.app"),
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"app.detach","id":1,"params":{"appId":"com.example.app"}}"#
        );
    }

    #[test]
    fn stop_app() {
        let method = FlutterDaemonRequest::StopApp {
            id: 1,
            params: super::StopAppParams {
                app_id: String::from("com.example.app"),
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"app.stop","id":1,"params":{"appId":"com.example.app"}}"#
        );
    }

    #[test]
    fn call_service_extension() {
        let method = FlutterDaemonRequest::CallServiceExtension {
            id: 1,
            params: super::CallServiceExtensionParams {
                app_id: String::from("com.example.app"),
                method_name: String::from("ext.flutter.debugPaint"),
                params: None,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"app.callServiceExtension","id":1,"params":{"appId":"com.example.app","methodName":"ext.flutter.debugPaint"}}"#
        );

        let method = FlutterDaemonRequest::CallServiceExtension {
            id: 1,
            params: super::CallServiceExtensionParams {
                app_id: String::from("com.example.app"),
                method_name: String::from("ext.flutter.debugPaint"),
                params: Some(serde_json::from_str(r#"{"enabled":true}"#).unwrap()),
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"app.callServiceExtension","id":1,"params":{"appId":"com.example.app","methodName":"ext.flutter.debugPaint","params":{"enabled":true}}}"#
        );
    }
}
