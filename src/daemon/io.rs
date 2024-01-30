use serde::Deserialize;

use self::{event::FlutterDaemonEvent, response::FlutterDaemonResponse};

pub mod device;
pub mod emulator;
pub mod event;
pub mod request;
pub mod response;

pub fn parse_event(s: &str) -> Option<FlutterDaemonEvent> {
    if !(s.starts_with("[{") && s.ends_with("}]")) {
        return None;
    }
    let s = s.trim_start_matches('[').trim_end_matches(']');
    serde_json::from_str::<FlutterDaemonEvent>(s).ok()
}

pub fn parse_response<'a, T>(s: &'a str, id: u32) -> Option<FlutterDaemonResponse<T>>
where
    T: Deserialize<'a>,
{
    if !(s.starts_with("[{") && s.ends_with("}]")) {
        return None;
    }
    let s = s.trim_start_matches('[').trim_end_matches(']');
    let response = serde_json::from_str::<FlutterDaemonResponse<T>>(s);
    if let Ok(response) = response {
        if response.id == id {
            return Some(response);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::daemon::io::{
        device::{Device, DeviceCapabilities},
        event::ConnectedEventParams,
        response::GetDevicesResponse,
    };

    use super::*;

    #[test]
    fn test_parse_event() {
        let s = r#"[{"event":"daemon.connected","params":{"version":"2.0.0","pid":1234}}]"#;
        let event = parse_event(s);
        assert_eq!(
            event,
            Some(FlutterDaemonEvent::Connected {
                params: ConnectedEventParams {
                    version: "2.0.0".to_string(),
                    pid: 1234,
                }
            })
        );

        let s = r#"{"id":123,"result":"1234"}"#;
        let event = parse_event(s);
        assert_eq!(event, None);

        let s = r#"some invalid message"#;
        let event = parse_event(s);
        assert_eq!(event, None,);
    }

    #[test]
    fn test_parse_response() {
        let s = r#"[{"id":123,"result":"1234"}]"#;
        let response = parse_response::<String>(s, 123);
        assert_eq!(
            response,
            Some(FlutterDaemonResponse {
                id: 123,
                result: Some("1234".to_string())
            })
        );

        let s = r#"[{"id":1,"result":[{"id":"linux","name":"Linux","platform":"linux","emulator":false,"category":"mobile","platformType":"desktop","ephemeral":false,"capabilities":{"hotReload":true,"hotRestart":true,"screenshot":true,"fastStart":true,"flutterExit":true,"hardwareRendering":true,"startPaused":false},"sdk":"Flutter (Channel stable, 2.0.3, on Linux, locale en_US.UTF-8)","emulatorId":"linux"}]}]"#;
        let response: Option<GetDevicesResponse> = parse_response(s, 1);
        assert_eq!(
            response,
            Some(GetDevicesResponse {
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
            })
        );

        let response: Option<GetDevicesResponse> = parse_response(s, 2);
        assert_eq!(response, None);

        let s = r#"some invalid message"#;
        let response = parse_response::<String>(s, 1);
        assert_eq!(response, None);
    }
}
