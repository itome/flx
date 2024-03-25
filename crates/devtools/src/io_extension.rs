use crate::{params, protocols::io_extension::*, service::VmService};
use color_eyre::Result;
use serde_json::Map;

impl IoExtensionProtocol for VmService {
    async fn get_version(&self) -> Result<Version> {
        self.call("ext.dart.io.getVersion", Map::new()).await
    }

    async fn socket_profiling_enabled(
        &self,
        isolate_id: String,
        enabled: bool,
    ) -> Result<SocketProfilingState> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.dart.io.socketProfilingEnabled", params)
            .await
    }

    async fn clear_socket_profile(&self, isolate_id: String) -> Result<Success> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.dart.io.clearSocketProfile", params).await
    }

    async fn get_socket_profile(&self, isolate_id: String) -> Result<SocketProfile> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.dart.io.getSocketProfile", params).await
    }

    async fn get_open_file_by_id(&self, isolate_id: String, id: i64) -> Result<OpenFile> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
        };
        self.call("ext.dart.io.getOpenFileById", params).await
    }

    async fn get_open_files(&self, isolate_id: String) -> Result<OpenFileList> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.dart.io.getOpenFiles", params).await
    }

    async fn get_spawned_process_by_id(
        &self,
        isolate_id: String,
        id: i64,
    ) -> Result<SpawnedProcess> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
        };
        self.call("ext.dart.io.getSpawnedProcessById", params).await
    }

    async fn get_spawned_processes(&self, isolate_id: String) -> Result<SpawnedProcessList> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.dart.io.getSpawnedProcesses", params).await
    }

    async fn http_enable_timeline_logging(
        &self,
        isolate_id: String,
        enabled: bool,
    ) -> Result<HttpTimelineLoggingState> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.dart.io.httpEnableTimelineLogging", params)
            .await
    }

    async fn get_http_profile(
        &self,
        isolate_id: String,
        updated_since: Option<i64>,
    ) -> Result<HttpProfile> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "updatedSince".to_owned() => updated_since.into(),
        };
        self.call("ext.dart.io.getHttpProfile", params).await
    }

    async fn get_http_profile_request(
        &self,
        isolate_id: String,
        id: String,
    ) -> Result<HttpProfileRequest> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
        };
        self.call("ext.dart.io.getHttpProfileRequest", params).await
    }

    async fn clear_http_profile(&self, isolate_id: String) -> Result<Success> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.dart.io.clearHttpProfile", params).await
    }
}
