use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmptyParams {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
pub enum VmServiceRequest {
    #[serde(rename = "getVersion")]
    GetVersion {
        jsonrpc: String,
        id: u32,
        params: EmptyParams,
    },
}

#[cfg(test)]
mod tests {
    use crate::devtools::io::request::EmptyParams;

    use super::VmServiceRequest;

    #[test]
    fn get_version() {
        let method = VmServiceRequest::GetVersion {
            jsonrpc: "2.0".to_string(),
            id: 1,
            params: EmptyParams {},
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"getVersion","jsonrpc":"2.0","id":1,"params":{}}"#
        );
    }
}
