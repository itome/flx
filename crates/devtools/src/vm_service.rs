use crate::params;
use crate::protocols::vm_service::*;
use color_eyre::{eyre::eyre, Result};
use futures::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

pub struct VmService {
    pub incoming_tx: broadcast::Sender<String>,
    _incoming_rx: broadcast::Receiver<String>,
    outgoing_tx: mpsc::Sender<String>,
    outgoing_rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl Default for VmService {
    fn default() -> Self {
        Self::new()
    }
}

impl VmService {
    pub fn new() -> Self {
        let (incoming_tx, _incoming_rx) = broadcast::channel::<String>(16);
        let (outgoing_tx, outgoing_rx) = mpsc::channel::<String>(16);

        Self {
            incoming_tx,
            _incoming_rx,
            outgoing_tx,
            outgoing_rx: Arc::new(Mutex::new(outgoing_rx)),
        }
    }

    pub async fn connect(&self, uri: String) {
        let _incoming_tx = self.incoming_tx.clone();
        let _outgoing_rx = self.outgoing_rx.clone();
        let Ok((stream, _)) = connect_async(uri).await else {
            return;
        };
        let (mut write, mut read) = stream.split();
        tokio::spawn(async move {
            let mut _outgoing_rx = _outgoing_rx.lock().await;
            loop {
                tokio::select! {
                    Some(Ok(Message::Text(next))) = read.next() => {
                        log::info!("[<====] {}", next);
                        _incoming_tx.send(next).unwrap();
                    },
                    Some(text) = _outgoing_rx.recv() => {
                        log::info!("[====>] {}", text);
                        if let Err(e) = write.send(Message::Text(text)).await {
                            log::error!("Error sending message: {:?}", e);
                        };
                    },
                };
            }
        });
    }

    pub async fn next_event(&self, stream_id: StreamId) -> Result<Event> {
        let mut rx = self.incoming_tx.subscribe();
        while let Ok(line) = rx.recv().await {
            let response = serde_json::from_str::<VmServiceEvent>(&line);
            if let Ok(res) = response {
                if res.method == "streamNotify" && res.params.stream_id == stream_id {
                    return Ok(res.params.event);
                }
            }
        }
        Err(eyre!("Could not receive vm service event"))
    }

    pub(crate) async fn call<T>(
        &self,
        method: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let request_id = Uuid::new_v4().to_string();
        let request = VmServiceRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id.clone(),
            method: method.to_string(),
            params,
        };
        self.send_request(&request).await?;
        let result: VmServiceResponse<T> = self.receive_response(&request_id).await?;
        Ok(result.result)
    }

    async fn send_request(&self, request: &VmServiceRequest) -> Result<()> {
        let message = serde_json::to_string(request)?;
        self.outgoing_tx.send(message).await?;
        Ok(())
    }

    async fn receive_response<T>(&self, request_id: &str) -> Result<VmServiceResponse<T>>
    where
        T: DeserializeOwned,
    {
        let mut rx = self.incoming_tx.subscribe();
        while let Ok(line) = rx.recv().await {
            let mut deserializer = serde_json::Deserializer::from_str(&line);
            deserializer.disable_recursion_limit();
            let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
            let Ok(response) = VmServiceResponse::<T>::deserialize(deserializer) else {
                continue;
            };
            if response.id == request_id {
                return Ok(response);
            }
        }
        Err(eyre!("Could not receive vm service response"))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct VmServiceRequest {
    jsonrpc: String,
    id: String,
    method: String,
    params: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VmServiceResponse<R> {
    pub id: String,
    pub jsonrpc: String,
    pub result: R,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VmServiceEvent {
    pub jsonrpc: String,
    pub method: String,
    pub params: VmServiceEventParams,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VmServiceEventParams {
    #[serde(rename = "streamId")]
    pub stream_id: StreamId,
    pub event: Event,
}

impl VmServiceProtocol for VmService {
    async fn add_breakpoint(
        &self,
        isolate_id: &str,
        script_id: &str,
        line: i32,
        column: Option<i32>,
    ) -> Result<BreakpointOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "scriptId".to_owned() => script_id.into(),
            "line".to_owned() => line.into(),
            "column".to_owned() => column.into(),
        };
        self.call("addBreakpoint", params).await
    }

    async fn add_breakpoint_with_script_uri(
        &self,
        isolate_id: &str,
        script_uri: &str,
        line: i32,
        column: Option<i32>,
    ) -> Result<BreakpointOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "scriptUri".to_owned() => script_uri.into(),
            "line".to_owned() => line.into(),
            "column".to_owned() => column.into(),
        };
        self.call("addBreakpointWithScriptUri", params).await
    }

    async fn add_breakpoint_at_entry(
        &self,
        isolate_id: &str,
        function_id: &str,
    ) -> Result<BreakpointOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "functionId".to_owned() => function_id.into(),
        };
        self.call("addBreakpointAtEntry", params).await
    }

    async fn clear_cpu_samples(&self, isolate_id: &str) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("clearCpuSamples", params).await
    }

    async fn clear_vm_timeline(&self) -> Result<Success> {
        self.call("clearVMTimeline", Map::new()).await
    }

    async fn invoke(
        &self,
        isolate_id: &str,
        target_id: &str,
        selector: &str,
        argument_ids: Vec<String>,
        disable_breakpoints: Option<bool>,
    ) -> Result<InstanceRefOrSentinelOrErrorRef> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "targetId".to_owned() => target_id.into(),
            "selector".to_owned() => selector.into(),
            "argumentIds".to_owned() => argument_ids.into(),
            "disableBreakpoints".to_owned() => disable_breakpoints.into(),
        };
        self.call("invoke", params).await
    }

    async fn evaluate(
        &self,
        isolate_id: &str,
        frame_index: i32,
        expression: &str,
        scope: Option<HashMap<String, String>>,
        disable_breakpoints: Option<bool>,
    ) -> Result<InstanceRefOrSentinelOrErrorRef> {
        let mut params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "frameIndex".to_owned() => frame_index.into(),
            "expression".to_owned() => expression.into(),
            "disableBreakpoints".to_owned() => disable_breakpoints.into(),
        };
        if let Some(scope) = scope {
            params.insert("scope".to_owned(), serde_json::to_value(scope).unwrap());
        }
        self.call("evaluate", params).await
    }

    async fn evaluate_in_frame(
        &self,
        isolate_id: &str,
        frame_index: i32,
        expression: &str,
        scope: Option<HashMap<String, String>>,
        disable_breakpoints: Option<bool>,
    ) -> Result<InstanceRefOrSentinelOrErrorRef> {
        let mut params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "frameIndex".to_owned() => frame_index.into(),
            "expression".to_owned() => expression.into(),
            "disableBreakpoints".to_owned() => disable_breakpoints.into(),
        };
        if let Some(scope) = scope {
            params.insert("scope".to_owned(), serde_json::to_value(scope).unwrap());
        }
        self.call("evaluateInFrame", params).await
    }

    async fn get_allocation_profile(
        &self,
        isolate_id: &str,
        reset: Option<bool>,
        gc: Option<bool>,
    ) -> Result<AllocationProfileOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "reset".to_owned() => reset.into(),
            "gc".to_owned() => gc.into(),
        };
        self.call("getAllocationProfile", params).await
    }

    async fn get_allocation_traces(
        &self,
        isolate_id: &str,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
        class_id: Option<&str>,
    ) -> Result<CpuSamples> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "timeOriginMicros".to_owned() => time_origin_micros.into(),
            "timeExtentMicros".to_owned() => time_extent_micros.into(),
            "classId".to_owned() => class_id.into(),
        };
        self.call("getAllocationTraces", params).await
    }

    async fn get_class_list(&self, isolate_id: &str) -> Result<ClassListOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("getClassList", params).await
    }

    async fn get_cpu_samples(
        &self,
        isolate_id: &str,
        time_origin_micros: i32,
        time_extent_micros: i32,
    ) -> Result<CpuSamplesOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "timeOriginMicros".to_owned() => time_origin_micros.into(),
            "timeExtentMicros".to_owned() => time_extent_micros.into(),
        };
        self.call("getCpuSamples", params).await
    }

    async fn get_flag_list(&self) -> Result<FlagList> {
        self.call("getFlagList", Map::new()).await
    }

    async fn get_inbound_references(
        &self,
        isolate_id: &str,
        target_id: &str,
        limit: i32,
    ) -> Result<InboundReferencesOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "targetId".to_owned() => target_id.into(),
            "limit".to_owned() => limit.into(),
        };
        self.call("getInboundReferences", params).await
    }

    async fn get_instances(
        &self,
        isolate_id: &str,
        object_id: &str,
        limit: i32,
        include_subclasses: Option<bool>,
        include_implementers: Option<bool>,
    ) -> Result<InstanceSetOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectId".to_owned() => object_id.into(),
            "limit".to_owned() => limit.into(),
            "includeSubclasses".to_owned() => include_subclasses.into(),
            "includeImplementers".to_owned() => include_implementers.into(),
        };
        self.call("getInstances", params).await
    }

    async fn get_instances_as_list(
        &self,
        isolate_id: &str,
        object_id: &str,
        include_subclasses: Option<bool>,
        include_implementers: Option<bool>,
    ) -> Result<InstanceRefOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectId".to_owned() => object_id.into(),
            "includeSubclasses".to_owned() => include_subclasses.into(),
            "includeImplementers".to_owned() => include_implementers.into(),
        };
        self.call("getInstancesAsList", params).await
    }

    async fn get_isolate(&self, isolate_id: &str) -> Result<IsolateOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("getIsolate", params).await
    }

    async fn get_isolate_group(&self, isolate_group_id: &str) -> Result<IsolateGroupOrSentinel> {
        let params = params! {
            "isolateGroupId".to_owned() => isolate_group_id.into(),
        };
        self.call("getIsolateGroup", params).await
    }

    async fn get_isolate_pause_event(&self, isolate_id: &str) -> Result<EventOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("getIsolatePauseEvent", params).await
    }

    async fn get_memory_usage(&self, isolate_id: &str) -> Result<MemoryUsageOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("getMemoryUsage", params).await
    }

    async fn get_isolate_group_memory_usage(
        &self,
        isolate_group_id: &str,
    ) -> Result<MemoryUsageOrSentinel> {
        let params = params! {
            "isolateGroupId".to_owned() => isolate_group_id.into(),
        };
        self.call("getIsolateGroupMemoryUsage", params).await
    }

    async fn get_scripts(&self, isolate_id: &str) -> Result<ScriptListOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("getScripts", params).await
    }

    async fn get_object(
        &self,
        isolate_id: &str,
        object_id: &str,
        offset: Option<i32>,
        count: Option<i32>,
    ) -> Result<ObjectOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectId".to_owned() => object_id.into(),
            "offset".to_owned() => offset.into(),
            "count".to_owned() => count.into(),
        };
        self.call("getObject", params).await
    }

    async fn get_perfetto_cpu_samples(
        &self,
        isolate_id: &str,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
    ) -> Result<PerfettoCpuSamplesOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "timeOriginMicros".to_owned() => time_origin_micros.into(),
            "timeExtentMicros".to_owned() => time_extent_micros.into(),
        };
        self.call("getPerfettoCpuSamples", params).await
    }

    async fn get_perfecto_vm_timeline(
        &self,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
    ) -> Result<PerfettoTimeline> {
        let params = params! {
            "timeOriginMicros".to_owned() => time_origin_micros.into(),
            "timeExtentMicros".to_owned() => time_extent_micros.into(),
        };
        self.call("getPerfettoVMTimeline", params).await
    }

    async fn get_ports(&self, isolate_id: &str) -> Result<PortList> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("getPorts", params).await
    }

    async fn get_retaining_path(
        &self,
        isolate_id: &str,
        target_id: &str,
        limit: i32,
    ) -> Result<RetainingPathOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "targetId".to_owned() => target_id.into(),
            "limit".to_owned() => limit.into(),
        };
        self.call("getRetainingPath", params).await
    }

    async fn get_process_memory_usage(&self) -> Result<ProcessMemoryUsage> {
        self.call("getProcessMemoryUsage", Map::new()).await
    }

    async fn get_stack(&self, isolate_id: &str, limit: Option<i32>) -> Result<StackOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "limit".to_owned() => limit.into(),
        };
        self.call("getStack", params).await
    }

    async fn get_supported_protocols(&self) -> Result<ProtocolList> {
        self.call("getSupportedProtocols", Map::new()).await
    }

    async fn get_source_report(
        &self,
        isolate_id: &str,
        reports: Vec<SourceReportKind>,
        script_id: Option<&str>,
        token_pos: Option<i32>,
        end_token_pos: Option<i32>,
        force_compile: Option<bool>,
        report_lines: Option<bool>,
        library_filters: Option<Vec<String>>,
        libraries_already_compiled: Option<Vec<String>>,
    ) -> Result<SourceReportOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "reports".to_owned() => serde_json::to_value(reports).unwrap(),
            "scriptId".to_owned() => script_id.into(),
            "tokenPos".to_owned() => token_pos.into(),
            "endTokenPos".to_owned() => end_token_pos.into(),
            "forceCompile".to_owned() => force_compile.into(),
            "reportLines".to_owned() => report_lines.into(),
            "libraryFilters".to_owned() => library_filters.into(),
            "librariesAlreadyCompiled".to_owned() => libraries_already_compiled.into(),
        };
        self.call("getSourceReport", params).await
    }

    async fn get_version(&self) -> Result<Version> {
        self.call("getVersion", Map::new()).await
    }

    async fn get_vm(&self) -> Result<VM> {
        self.call("getVM", Map::new()).await
    }

    async fn get_vm_timeline(
        &self,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
    ) -> Result<Timeline> {
        let params = params! {
            "timeOriginMicros".to_owned() => time_origin_micros.into(),
            "timeExtentMicros".to_owned() => time_extent_micros.into(),
        };
        self.call("getVMTimeline", params).await
    }

    async fn get_vm_timeline_flags(&self) -> Result<TimelineFlags> {
        self.call("getVMTimelineFlags", Map::new()).await
    }

    async fn get_vm_timeline_micros(&self) -> Result<Timestamp> {
        self.call("getVMTimelineMicros", Map::new()).await
    }

    async fn pause(&self, isolate_id: &str) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("pause", params).await
    }

    async fn kill(&self, isolate_id: &str) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("kill", params).await
    }

    async fn lookup_resolved_package_uris(
        &self,
        isolate_id: &str,
        uris: Vec<String>,
        local: Option<bool>,
    ) -> Result<UriList> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "uris".to_owned() => uris.into(),
            "local".to_owned() => local.into(),
        };
        self.call("lookupResolvedPackageUris", params).await
    }

    async fn lookup_package_uris(&self, isolate_id: &str, uris: Vec<String>) -> Result<UriList> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "uris".to_owned() => uris.into(),
        };
        self.call("lookupPackageUris", params).await
    }

    async fn register_service(&self, service: &str, alias: &str) -> Result<SuccessOrSentinel> {
        let params = params! {
            "service".to_owned() => service.into(),
            "alias".to_owned() => alias.into(),
        };
        self.call("registerService", params).await
    }

    async fn reload_sources(
        &self,
        isolate_id: &str,
        force: Option<bool>,
        pause: Option<bool>,
        root_lib_uri: Option<&str>,
        packages_uri: Option<&str>,
    ) -> Result<ReloadReportOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "force".to_owned() => force.into(),
            "pause".to_owned() => pause.into(),
            "rootLibUri".to_owned() => root_lib_uri.into(),
            "packagesUri".to_owned() => packages_uri.into(),
        };
        self.call("reloadSources", params).await
    }

    async fn remove_breakpoint(
        &self,
        isolate_id: &str,
        breakpoint_id: &str,
    ) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "breakpointId".to_owned() => breakpoint_id.into(),
        };
        self.call("removeBreakpoint", params).await
    }

    async fn request_heap_snapshot(&self, isolate_id: &str) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("requestHeapSnapshot", params).await
    }

    async fn resume(
        &self,
        isolate_id: &str,
        step: Option<StepOption>,
        frame_index: Option<i32>,
    ) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "step".to_owned() => serde_json::to_value(step).unwrap(),
            "frameIndex".to_owned() => frame_index.into(),
        };
        self.call("resume", params).await
    }

    async fn set_breakpoint_state(
        &self,
        isolate_id: &str,
        breakpoint_id: &str,
        enable: bool,
    ) -> Result<Breakpoint> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "breakpointId".to_owned() => breakpoint_id.into(),
            "enable".to_owned() => enable.into(),
        };
        self.call("setBreakpointState", params).await
    }

    async fn set_exception_pause_mode(
        &self,
        isolate_id: &str,
        mode: ExceptionPauseMode,
    ) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "mode".to_owned() => serde_json::to_value(mode).unwrap(),
        };
        self.call("setExceptionPauseMode", params).await
    }

    async fn set_isolate_pause_mode(
        &self,
        isolate_id: &str,
        exception_pause_mode: Option<ExceptionPauseMode>,
        should_pause_on_exit: Option<bool>,
    ) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "exceptionPauseMode".to_owned() => serde_json::to_value(exception_pause_mode).unwrap(),
            "shouldPauseOnExit".to_owned() => should_pause_on_exit.into(),
        };
        self.call("setIsolatePauseMode", params).await
    }

    async fn set_flag(&self, name: &str, value: &str) -> Result<SuccessOrError> {
        let params = params! {
            "name".to_owned() => name.into(),
            "value".to_owned() => value.into(),
        };
        self.call("setFlag", params).await
    }

    async fn set_library_debuggable(
        &self,
        isolate_id: &str,
        library_id: &str,
        is_debuggable: bool,
    ) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "libraryId".to_owned() => library_id.into(),
            "isDebuggable".to_owned() => is_debuggable.into(),
        };
        self.call("setLibraryDebuggable", params).await
    }

    async fn set_name(&self, isolate_id: &str, name: &str) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "name".to_owned() => name.into(),
        };
        self.call("setName", params).await
    }

    async fn set_trace_class_allocation(
        &self,
        isolate_id: &str,
        class_id: &str,
        enable: bool,
    ) -> Result<SuccessOrSentinel> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "classId".to_owned() => class_id.into(),
            "enable".to_owned() => enable.into(),
        };
        self.call("setTraceClassAllocation", params).await
    }

    async fn set_vm_name(&self, name: &str) -> Result<Success> {
        let params = params! {
            "name".to_owned() => name.into(),
        };
        self.call("setVMName", params).await
    }

    async fn set_vm_timeline_flags(&self, recorded_streams: Vec<String>) -> Result<Success> {
        let params = params! {
            "recordedStreams".to_owned() => recorded_streams.into(),
        };
        self.call("setVMTimelineFlags", params).await
    }

    async fn stream_cancel(&self, stream_id: StreamId) -> Result<Success> {
        let params = params! {
            "streamId".to_owned() => serde_json::to_value(stream_id).unwrap(),
        };
        self.call("streamCancel", params).await
    }

    async fn stream_enable(&self, stream_id: StreamId) -> Result<Success> {
        let params = params! {
            "streamId".to_owned() => serde_json::to_value(stream_id).unwrap(),
        };
        self.call("streamEnable", params).await
    }

    async fn stream_listen(&self, stream_id: StreamId) -> Result<Success> {
        let params = params! {
            "streamId".to_owned() => serde_json::to_value(stream_id).unwrap(),
        };
        self.call("streamListen", params).await
    }
}
