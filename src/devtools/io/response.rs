use super::types::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VmServiceResponse<R> {
    pub id: u32,
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,
}

pub type GetVersionResponse = VmServiceResponse<Version>;

pub type SuccessResponse = VmServiceResponse<Success>;

#[cfg(test)]
mod tests {
    use crate::devtools::io::response::GetVersionResponse;
    use crate::devtools::io::types::*;

    #[test]
    fn get_version() {
        let json = r#"{"jsonrpc":"2.0","id":1,"result":{"type":"Version","major":2,"minor":0}}"#;
        let response: super::GetVersionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            super::VmServiceResponse {
                id: 1,
                jsonrpc: "2.0".to_string(),
                result: Some(Version {
                    r#type: "Version".to_string(),
                    major: 2,
                    minor: 0,
                })
            }
        );
    }

    #[test]
    fn success() {
        let json = r#"{"jsonrpc":"2.0","id":1,"result":{"type":"Success"}}"#;
        let response: super::SuccessResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            super::VmServiceResponse {
                id: 1,
                jsonrpc: "2.0".to_string(),
                result: Some(Success {
                    r#type: "Success".to_string()
                })
            }
        );
    }
}
