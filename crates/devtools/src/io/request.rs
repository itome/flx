use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmptyParams {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StreamId {
    VM,
    Isolate,
    Debug,
    Profiler,
    GC,
    Extension,
    Timeline,
    Logging,
    Service,
    HeapSnapshot,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamCancelParams {
    #[serde(rename = "streamId")]
    pub stream_id: StreamId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamListenParams {
    #[serde(rename = "streamId")]
    pub stream_id: StreamId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FlutterListViewsParams {
    #[serde(rename = "isolateId")]
    pub isolate_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FlutterGetDisplayRefreshRateParams {
    #[serde(rename = "isolateId")]
    pub isolate_id: String,
    #[serde(rename = "viewId")]
    pub view_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
pub enum VmServiceRequest {
    #[serde(rename = "getVersion")]
    GetVersion {
        jsonrpc: String,
        id: u32,
        params: EmptyParams,
    },

    #[serde(rename = "streamListen")]
    StreamListen {
        jsonrpc: String,
        id: u32,
        params: StreamListenParams,
    },

    #[serde(rename = "streamCancel")]
    StreamCancel {
        jsonrpc: String,
        id: u32,
        params: StreamCancelParams,
    },

    /* ----------------------------- */
    /* Extension Request for Flutter */
    /* ----------------------------- */
    #[serde(rename = "_flutter.listViews")]
    FlutterListViews {
        jsonrpc: String,
        id: u32,
        params: FlutterListViewsParams,
    },

    #[serde(rename = "_flutter.getDisplayRefreshRate")]
    FlutterGetDisplayRefreshRate {
        jsonrpc: String,
        id: u32,
        params: FlutterGetDisplayRefreshRateParams,
    },
}

#[cfg(test)]
mod tests {
    use crate::io::request::{EmptyParams, StreamId};

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

    #[test]
    fn stream_listen() {
        let method = VmServiceRequest::StreamListen {
            jsonrpc: "2.0".to_string(),
            id: 1,
            params: super::StreamListenParams {
                stream_id: StreamId::VM,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"streamListen","jsonrpc":"2.0","id":1,"params":{"streamId":"VM"}}"#
        );
    }

    #[test]
    fn stream_cancel() {
        let method = VmServiceRequest::StreamCancel {
            jsonrpc: "2.0".to_string(),
            id: 1,
            params: super::StreamCancelParams {
                stream_id: StreamId::Isolate,
            },
        };
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(
            serialized,
            r#"{"method":"streamCancel","jsonrpc":"2.0","id":1,"params":{"streamId":"Isolate"}}"#
        );
    }
}
