use color_eyre::Result;
use futures::Future;
use serde::{Deserialize, Serialize};

pub trait IoExtensionProtocol {
    fn get_version(&self) -> impl Future<Output = Result<Version>> + Send;

    fn socket_profiling_enabled(
        &self,
        isolate_id: String,
        enabled: bool,
    ) -> impl Future<Output = Result<SocketProfilingState>> + Send;

    fn clear_socket_profile(
        &self,
        isolate_id: String,
    ) -> impl Future<Output = Result<Success>> + Send;

    fn get_socket_profile(
        &self,
        isolate_id: String,
    ) -> impl Future<Output = Result<SocketProfile>> + Send;

    fn get_open_file_by_id(
        &self,
        isolate_id: String,
        id: i64,
    ) -> impl Future<Output = Result<OpenFile>> + Send;

    fn get_open_files(
        &self,
        isolate_id: String,
    ) -> impl Future<Output = Result<OpenFileList>> + Send;

    fn get_spawned_process_by_id(
        &self,
        isolate_id: String,
        id: i64,
    ) -> impl Future<Output = Result<SpawnedProcess>> + Send;

    fn get_spawned_processes(
        &self,
        isolate_id: String,
    ) -> impl Future<Output = Result<SpawnedProcessList>> + Send;

    fn http_enable_timeline_logging(
        &self,
        isolate_id: String,
        enabled: bool,
    ) -> impl Future<Output = Result<HttpTimelineLoggingState>> + Send;

    fn get_http_profile(
        &self,
        isolate_id: String,
        updated_since: Option<i64>,
    ) -> impl Future<Output = Result<HttpProfile>> + Send;

    fn get_http_profile_request(
        &self,
        isolate_id: String,
        id: String,
    ) -> impl Future<Output = Result<HttpProfileRequest>> + Send;

    fn clear_http_profile(
        &self,
        isolate_id: String,
    ) -> impl Future<Output = Result<Success>> + Send;
}

// Public types from the Dart VM Service Extension Protocol
// https://github.com/dart-lang/sdk/blob/main/runtime/vm/service/service_extension.md
//

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OpenFileRef {
    pub r#type: String,
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OpenFile {
    pub r#type: String,
    pub id: i64,
    pub name: String,
    pub read_bytes: i64,
    pub write_bytes: i64,
    pub read_count: i64,
    pub write_count: i64,
    pub last_read_time: i64,
    pub last_write_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OpenFileList {
    pub r#type: String,
    pub files: Vec<OpenFileRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpTimelineLoggingState {
    pub r#type: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfile {
    pub r#type: String,
    pub timestamp: i64,
    pub requests: Vec<HttpProfileRequestRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfileRequestRef {
    pub r#type: String,
    pub id: String,
    pub isolate_id: String,
    pub method: String,
    pub uri: String,
    pub events: Vec<HttpProfileRequestEvent>,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub request: Option<HttpProfileRequestData>,
    pub response: Option<HttpProfileResponseData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfileRequest {
    pub r#type: String,
    pub id: String,
    pub isolate_id: String,
    pub method: String,
    pub uri: String,
    pub events: Vec<HttpProfileRequestEvent>,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub request: Option<HttpProfileRequestData>,
    pub response: Option<HttpProfileResponseData>,
    pub request_body: Option<Vec<i64>>,
    pub response_body: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfileRequestData {
    pub r#type: String,
    pub connection_info: Option<serde_json::Map<String, serde_json::Value>>,
    pub content_length: Option<i64>,
    pub cookies: Option<Vec<String>>,
    pub error: Option<String>,
    pub follow_redirects: Option<bool>,
    pub headers: Option<serde_json::Map<String, serde_json::Value>>,
    pub max_redirects: Option<i64>,
    pub persistent_connection: Option<bool>,
    pub proxy_details: Option<HttpProfileProxyData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfileResponseData {
    pub r#type: String,
    pub redirects: Vec<serde_json::Map<String, serde_json::Value>>,
    pub cookies: Option<Vec<String>>,
    pub connection_info: Option<serde_json::Map<String, serde_json::Value>>,
    pub headers: Option<serde_json::Map<String, serde_json::Value>>,
    pub compression_state: Option<String>,
    pub reason_phrase: Option<String>,
    pub is_redirect: Option<bool>,
    pub persistent_connection: Option<bool>,
    pub content_length: Option<i64>,
    pub status_code: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfileProxyData {
    pub r#type: String,
    pub host: Option<String>,
    pub username: Option<String>,
    pub is_direct: Option<bool>,
    pub port: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HttpProfileRequestEvent {
    pub r#type: String,
    pub event: String,
    pub timestamp: i64,
    pub arguments: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SocketProfilingState {
    pub r#type: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpawnedProcessRef {
    pub r#type: String,
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpawnedProcess {
    pub r#type: String,
    pub id: i64,
    pub name: String,
    pub pid: i64,
    pub started_at: i64,
    pub arguments: Vec<String>,
    pub working_directory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpawnedProcessList {
    pub r#type: String,
    pub processes: Vec<SpawnedProcessRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SocketProfile {
    pub r#type: String,
    pub sockets: Vec<SocketStatistic>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SocketStatistic {
    pub id: i64,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub last_read_time: Option<i64>,
    pub last_write_time: Option<i64>,
    pub address: String,
    pub port: i64,
    pub socket_type: String,
    pub read_bytes: i64,
    pub write_bytes: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Success {
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Version {
    pub r#type: String,
    pub major: i64,
    pub minor: i64,
}
