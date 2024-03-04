use serde::Deserialize;

use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetVersionResult {
    r#type: String,
    major: u32,
    minor: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VmServiceResponse<R> {
    pub id: u32,
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,
}

pub type GetVersionResponse = VmServiceResponse<GetVersionResult>;

#[cfg(test)]
mod tests {
    use crate::devtools::io::response::GetVersionResponse;

    #[test]
    fn get_version() {
        let json = r#"{"jsonrpc":"2.0","id":1,"result":{"type":"daemon","major":2,"minor":0}}"#;
        let response: super::GetVersionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response,
            super::VmServiceResponse {
                id: 1,
                jsonrpc: "2.0".to_string(),
                result: Some(super::GetVersionResult {
                    r#type: "daemon".to_string(),
                    major: 2,
                    minor: 0,
                })
            }
        );
    }
}
