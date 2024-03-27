use std::collections::HashMap;

use color_eyre::Result;
use futures::Future;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Map, Value};

pub trait VmServiceProtocol {
    fn add_breakpoint(
        &self,
        isolate_id: &str,
        script_id: &str,
        line: i32,
        column: Option<i32>,
    ) -> impl Future<Output = Result<BreakpointOrSentinel>> + Send;

    fn add_breakpoint_with_script_uri(
        &self,
        isolate_id: &str,
        script_uri: &str,
        line: i32,
        column: Option<i32>,
    ) -> impl Future<Output = Result<BreakpointOrSentinel>> + Send;

    fn add_breakpoint_at_entry(
        &self,
        isolate_id: &str,
        function_id: &str,
    ) -> impl Future<Output = Result<BreakpointOrSentinel>> + Send;

    fn clear_cpu_samples(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn clear_vm_timeline(&self) -> impl Future<Output = Result<Success>> + Send;

    fn invoke(
        &self,
        isolate_id: &str,
        target_id: &str,
        selector: &str,
        argument_ids: Vec<String>,
        disable_breakpoints: Option<bool>,
    ) -> impl Future<Output = Result<InstanceRefOrSentinelOrErrorRef>> + Send;

    fn evaluate(
        &self,
        isolate_id: &str,
        frame_index: i32,
        expression: &str,
        scope: Option<HashMap<String, String>>,
        disable_breakpoints: Option<bool>,
    ) -> impl Future<Output = Result<InstanceRefOrSentinelOrErrorRef>> + Send;

    fn evaluate_in_frame(
        &self,
        isolate_id: &str,
        frame_index: i32,
        expression: &str,
        scope: Option<HashMap<String, String>>,
        disable_breakpoints: Option<bool>,
    ) -> impl Future<Output = Result<InstanceRefOrSentinelOrErrorRef>> + Send;

    fn get_allocation_profile(
        &self,
        isolate_id: &str,
        reset: Option<bool>,
        gc: Option<bool>,
    ) -> impl Future<Output = Result<AllocationProfileOrSentinel>> + Send;

    fn get_allocation_traces(
        &self,
        isolate_id: &str,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
        class_id: Option<&str>,
    ) -> impl Future<Output = Result<CpuSamples>> + Send;

    fn get_class_list(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<ClassListOrSentinel>> + Send;

    fn get_cpu_samples(
        &self,
        isolate_id: &str,
        time_origin_micros: i32,
        time_extent_micros: i32,
    ) -> impl Future<Output = Result<CpuSamplesOrSentinel>> + Send;

    fn get_flag_list(&self) -> impl Future<Output = Result<FlagList>> + Send;

    fn get_inbound_references(
        &self,
        isolate_id: &str,
        target_id: &str,
        limit: i32,
    ) -> impl Future<Output = Result<InboundReferencesOrSentinel>> + Send;

    fn get_instances(
        &self,
        isolate_id: &str,
        object_id: &str,
        limit: i32,
        include_subclasses: Option<bool>,
        include_implementers: Option<bool>,
    ) -> impl Future<Output = Result<InstanceSetOrSentinel>> + Send;

    fn get_instances_as_list(
        &self,
        isolate_id: &str,
        object_id: &str,
        include_subclasses: Option<bool>,
        include_implementers: Option<bool>,
    ) -> impl Future<Output = Result<InstanceRefOrSentinel>> + Send;

    fn get_isolate(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<IsolateOrSentinel>> + Send;

    fn get_isolate_group(
        &self,
        isolate_group_id: &str,
    ) -> impl Future<Output = Result<IsolateGroupOrSentinel>> + Send;

    fn get_isolate_pause_event(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<EventOrSentinel>> + Send;

    fn get_memory_usage(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<MemoryUsageOrSentinel>> + Send;

    fn get_isolate_group_memory_usage(
        &self,
        isolate_group_id: &str,
    ) -> impl Future<Output = Result<MemoryUsageOrSentinel>> + Send;

    fn get_scripts(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<ScriptListOrSentinel>> + Send;

    fn get_object(
        &self,
        isolate_id: &str,
        object_id: &str,
        offset: Option<i32>,
        count: Option<i32>,
    ) -> impl Future<Output = Result<ObjectOrSentinel>> + Send;

    fn get_perfetto_cpu_samples(
        &self,
        isolate_id: &str,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
    ) -> impl Future<Output = Result<PerfettoCpuSamplesOrSentinel>> + Send;

    fn get_perfecto_vm_timeline(
        &self,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
    ) -> impl Future<Output = Result<PerfettoTimeline>> + Send;

    fn get_ports(&self, isolate_id: &str) -> impl Future<Output = Result<PortList>> + Send;

    fn get_retaining_path(
        &self,
        isolate_id: &str,
        target_id: &str,
        limit: i32,
    ) -> impl Future<Output = Result<RetainingPathOrSentinel>> + Send;

    fn get_process_memory_usage(&self) -> impl Future<Output = Result<ProcessMemoryUsage>> + Send;

    fn get_stack(
        &self,
        isolate_id: &str,
        limit: Option<i32>,
    ) -> impl Future<Output = Result<StackOrSentinel>> + Send;

    fn get_supported_protocols(&self) -> impl Future<Output = Result<ProtocolList>> + Send;

    #[allow(clippy::too_many_arguments)]
    fn get_source_report(
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
    ) -> impl Future<Output = Result<SourceReportOrSentinel>> + Send;

    fn get_version(&self) -> impl Future<Output = Result<Version>> + Send;

    fn get_vm(&self) -> impl Future<Output = Result<VM>> + Send;

    fn get_vm_timeline(
        &self,
        time_origin_micros: Option<i32>,
        time_extent_micros: Option<i32>,
    ) -> impl Future<Output = Result<Timeline>> + Send;

    fn get_vm_timeline_flags(&self) -> impl Future<Output = Result<TimelineFlags>> + Send;

    fn get_vm_timeline_micros(&self) -> impl Future<Output = Result<Timestamp>> + Send;

    fn pause(&self, isolate_id: &str) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn kill(&self, isolate_id: &str) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn lookup_resolved_package_uris(
        &self,
        isolate_id: &str,
        uris: Vec<String>,
        local: Option<bool>,
    ) -> impl Future<Output = Result<UriList>> + Send;

    fn lookup_package_uris(
        &self,
        isolate_id: &str,
        uris: Vec<String>,
    ) -> impl Future<Output = Result<UriList>> + Send;

    fn register_service(
        &self,
        service: &str,
        alias: &str,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn reload_sources(
        &self,
        isolate_id: &str,
        force: Option<bool>,
        pause: Option<bool>,
        root_lib_uri: Option<&str>,
        packages_uri: Option<&str>,
    ) -> impl Future<Output = Result<ReloadReportOrSentinel>> + Send;

    fn remove_breakpoint(
        &self,
        isolate_id: &str,
        breakpoint_id: &str,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn request_heap_snapshot(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn resume(
        &self,
        isolate_id: &str,
        step: Option<StepOption>,
        frame_index: Option<i32>,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn set_breakpoint_state(
        &self,
        isolate_id: &str,
        breakpoint_id: &str,
        enable: bool,
    ) -> impl Future<Output = Result<Breakpoint>> + Send;

    #[deprecated]
    fn set_exception_pause_mode(
        &self,
        isolate_id: &str,
        mode: ExceptionPauseMode,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn set_isolate_pause_mode(
        &self,
        isolate_id: &str,
        exception_pause_mode: Option<ExceptionPauseMode>,
        should_pause_on_exit: Option<bool>,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn set_flag(
        &self,
        name: &str,
        value: &str,
    ) -> impl Future<Output = Result<SuccessOrError>> + Send;

    fn set_library_debuggable(
        &self,
        isolate_id: &str,
        library_id: &str,
        is_debuggable: bool,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn set_name(
        &self,
        isolate_id: &str,
        name: &str,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn set_trace_class_allocation(
        &self,
        isolate_id: &str,
        class_id: &str,
        enable: bool,
    ) -> impl Future<Output = Result<SuccessOrSentinel>> + Send;

    fn set_vm_name(&self, name: &str) -> impl Future<Output = Result<Success>> + Send;

    fn set_vm_timeline_flags(
        &self,
        recorded_streams: Vec<String>,
    ) -> impl Future<Output = Result<Success>> + Send;

    fn stream_cancel(&self, stream_id: StreamId) -> impl Future<Output = Result<Success>> + Send;

    fn stream_enable(&self, stream_id: StreamId) -> impl Future<Output = Result<Success>> + Send;

    fn stream_listen(&self, stream_id: StreamId) -> impl Future<Output = Result<Success>> + Send;
}

// Public types from the Dart VM Service Protocol
// https://github.com/dart-lang/sdk/blob/main/runtime/vm/service/service.md#public-types
//

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
pub struct AllocationProfile {
    pub members: Vec<ClassHeapStats>,
    pub memory_usage: MemoryUsage,
    #[serde(rename = "dateLastAccumulatorReset")]
    pub date_last_accumulator_reset: Option<u32>,
    #[serde(rename = "dateLastServiceGC")]
    pub date_last_service_gc: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BoundField {
    pub decl: FieldRef,
    pub name: StringOrInt,
    pub value: InstanceRefOrSentinel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BoundVariable {
    pub name: String,
    pub value: InstanceRefOrSentinel,
    #[serde(rename = "declarationTokenPos")]
    pub declaration_token_pos: u32,
    #[serde(rename = "scopeStartTokenPos")]
    pub scope_start_token_pos: u32,
    #[serde(rename = "scopeEndTokenPos")]
    pub scope_end_token_pos: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Breakpoint {
    #[serde(rename = "breakpointNumber")]
    pub breakpoint_number: u32,
    pub enabled: bool,
    pub resolved: bool,
    #[serde(rename = "isSyntheticAsyncContinuation")]
    pub is_synthetic_async_continuation: Option<bool>,
    pub location: Box<SourceLocationOrUnresolvedSouceLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClassRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub name: String,
    pub location: Option<SourceLocation>,
    pub library: LibraryRef,
    #[serde(rename = "typeParameters")]
    pub type_parameters: Option<Vec<InstanceRef>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Class {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub name: String,
    pub location: Option<SourceLocation>,
    pub library: LibraryRef,
    #[serde(rename = "typeParameters")]
    pub type_parameters: Option<Vec<InstanceRef>>,
    pub error: Option<Error>,
    pub r#abstract: bool,
    pub r#const: bool,
    #[serde(rename = "isSealed")]
    pub is_sealed: bool,
    #[serde(rename = "isMixinClass")]
    pub is_mixin_class: bool,
    #[serde(rename = "isBaseClass")]
    pub is_base_class: bool,
    #[serde(rename = "isInterfaceClass")]
    pub is_interface_class: bool,
    #[serde(rename = "isFinal")]
    pub is_final: bool,
    #[serde(rename = "traceAllocations")]
    pub trace_allocations: bool,
    #[serde(rename = "super")]
    pub super_class: Option<ClassRef>,
    #[serde(rename = "superType")]
    pub super_type: Option<InstanceRef>,
    pub interfaces: Vec<InstanceRef>,
    pub mixin: Option<InstanceRef>,
    pub fields: Vec<FieldRef>,
    pub functions: Vec<FunctionRef>,
    pub subclasses: Vec<ClassRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClassHeapStats {
    pub r#type: String,
    pub class: ClassRef,
    #[serde(rename = "accumulatedSize")]
    pub accumulated_size: u32,
    #[serde(rename = "bytesCurrent")]
    pub bytes_current: u32,
    #[serde(rename = "instancesAccumulated")]
    pub instances_accumulated: u32,
    #[serde(rename = "instancesCurrent")]
    pub instances_current: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClassList {
    pub r#type: String,
    pub classes: Vec<ClassRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CodeRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub name: String,
    pub kind: CodeKind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Code {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub name: String,
    pub kind: CodeKind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CodeKind {
    Dart,
    Native,
    Stub,
    Tag,
    Collected,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContextRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub length: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Context {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub length: u32,
    pub parent: Option<ContextRef>,
    pub variables: Vec<ContextElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContextElement {
    pub value: InstanceRefOrSentinel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CpuSamples {
    pub r#type: String,
    #[serde(rename = "samplePeriod")]
    pub sample_period: u32,
    #[serde(rename = "maxStackDepth")]
    pub max_stack_depth: u32,
    #[serde(rename = "sampleCount")]
    pub sample_count: u32,
    #[serde(rename = "timeOriginMicros")]
    pub time_origin_micros: u32,
    #[serde(rename = "timeExtentMicros")]
    pub time_extent_micros: u32,
    pub pid: u32,
    pub functions: Vec<ProfileFunction>,
    pub samples: Vec<CpuSample>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CpuSamplesEvent {
    #[serde(rename = "samplePeriod")]
    pub sample_period: u32,
    #[serde(rename = "maxStackDepth")]
    pub max_stack_depth: u32,
    #[serde(rename = "sampleCount")]
    pub sample_count: u32,
    #[serde(rename = "timeOriginMicros")]
    pub time_origin_micros: u32,
    #[serde(rename = "timeExtentMicros")]
    pub time_extent_micros: u32,
    pub pid: u32,
    pub functions: Vec<ObjectRefOrNativeFunction>,
    pub samples: Vec<CpuSample>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CpuSample {
    pub tid: u32,
    pub timestamp: u32,
    #[serde(rename = "vmTag")]
    pub vm_tag: Option<String>,
    #[serde(rename = "userTag")]
    pub user_tag: Option<String>,
    pub truncated: Option<bool>,
    pub stack: Vec<u32>,
    #[serde(rename = "identityHashCode")]
    pub identity_hash_code: Option<u32>,
    #[serde(rename = "classId")]
    pub class_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ErrorRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub kind: ErrorKind,
    pub message: String,
    pub exception: Option<InstanceRef>,
    pub stacktrace: Option<InstanceRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnhandledException,
    LanguageError,
    InternalError,
    TerminationError,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    pub r#type: String,
    pub kind: EventKind,
    #[serde(rename = "isolateGroup")]
    pub isolate_group: Option<IsolateGroupRef>,
    pub isolate: Option<IsolateRef>,
    pub vm: Option<VMRef>,
    pub timestamp: u64,
    pub breakpoint: Option<Breakpoint>,
    #[serde(rename = "pauseBreakpoints")]
    pub pause_breakpoints: Option<Vec<Breakpoint>>,
    #[serde(rename = "topFrame")]
    pub top_frame: Option<Frame>,
    pub exception: Option<InstanceRef>,
    pub bytes: Option<String>,
    pub inspectee: Option<InstanceRef>,
    #[serde(rename = "gcType")]
    pub gc_type: Option<String>,
    #[serde(rename = "extensionRPC")]
    pub extension_rpc: Option<String>,
    #[serde(rename = "extensionKind")]
    pub extension_kind: Option<String>,
    #[serde(rename = "extensionData")]
    pub extension_data: Option<Map<String, Value>>,
    #[serde(rename = "timelineEvents")]
    pub timeline_events: Option<Vec<TimelineEvent>>,
    #[serde(rename = "updatedStreams")]
    pub updated_streams: Option<Vec<String>>,
    #[serde(rename = "atAsyncSuspension")]
    pub at_async_suspension: Option<bool>,
    pub status: Option<String>,
    #[serde(rename = "logRecord")]
    pub log_record: Option<LogRecord>,
    pub service: Option<String>,
    pub method: Option<String>,
    pub alias: Option<String>,
    pub flag: Option<String>,
    #[serde(rename = "newValue")]
    pub new_value: Option<String>,
    pub last: Option<bool>,
    #[serde(rename = "updatedTag")]
    pub updated_tag: Option<String>,
    #[serde(rename = "previousTag")]
    pub previous_tag: Option<String>,
    #[serde(rename = "cpuSamples")]
    pub cpu_samples: Option<CpuSamplesEvent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventKind {
    VMUpdate,
    VMFlagUpdate,
    IsolateStart,
    IsolateRunnable,
    IsolateExit,
    IsolateUpdate,
    IsolateReload,
    ServiceExtensionAdded,
    PauseStart,
    PauseExit,
    PauseBreakpoint,
    PauseInterrupted,
    PauseException,
    PausePostRequest,
    Resume,
    None,
    BreakpointAdded,
    BreakpointResolved,
    BreakpointRemoved,
    BreakpointUpdated,
    GC,
    WriteEvent,
    Inspect,
    Extension,
    Logging,
    TimelineEvents,
    TimelineStreamSubscriptionsUpdate,
    ServiceRegistered,
    ServiceUnregistered,
    UserTagChanged,
    CpuSamples,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FieldRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub name: String,
    pub owner: ObjectRef,
    #[serde(rename = "declaredType")]
    pub declared_type: InstanceRef,
    pub r#const: bool,
    pub r#final: bool,
    pub r#static: bool,
    pub location: Option<SourceLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Field {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub name: String,
    pub owner: ObjectRef,
    #[serde(rename = "declaredType")]
    pub declared_type: InstanceRef,
    pub r#const: bool,
    pub r#final: bool,
    pub r#static: bool,
    pub location: Option<SourceLocation>,
    #[serde(rename = "staticValue")]
    pub static_value: Option<InstanceRefOrSentinel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Flag {
    pub name: String,
    pub comment: String,
    pub modified: bool,
    #[serde(rename = "valueAsString")]
    pub value_as_string: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FlagList {
    pub r#type: String,
    pub flags: Vec<Flag>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Frame {
    pub r#type: String,
    pub index: u32,
    pub function: Option<FunctionRef>,
    pub code: Option<CodeRef>,
    pub location: Option<SourceLocation>,
    pub vars: Option<Vec<BoundVariable>>,
    pub kind: Option<FrameKind>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FrameKind {
    Regular,
    AsyncCausal,
    AsyncSuspensionMarker,
    AsyncActivation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FunctionRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub name: String,
    pub owner: Box<LibraryRefOrClassRefOrFunctionRef>,
    pub r#static: bool,
    pub r#const: bool,
    pub implicit: bool,
    pub r#abstract: bool,
    #[serde(rename = "isGetter")]
    pub is_getter: bool,
    #[serde(rename = "isSetter")]
    pub is_setter: bool,
    pub location: Option<SourceLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Function {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub name: String,
    pub owner: Box<LibraryRefOrClassRefOrFunctionRef>,
    pub r#static: bool,
    pub r#const: bool,
    pub implicit: bool,
    pub r#abstract: bool,
    #[serde(rename = "isGetter")]
    pub is_getter: bool,
    #[serde(rename = "isSetter")]
    pub is_setter: bool,
    pub location: Option<SourceLocation>,
    pub signature: InstanceRef,
    pub code: Option<CodeRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InstanceRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub kind: InstanceKind,
    #[serde(rename = "identityHashCode")]
    pub identity_hash_code: u32,
    pub class: ClassRef,
    #[serde(rename = "valueAsString")]
    pub value_as_string: Option<String>,
    #[serde(rename = "valueAsStringIsTruncated")]
    pub value_as_string_is_truncated: Option<bool>,
    pub length: Option<u32>,
    pub name: Option<String>,
    #[serde(rename = "typeClass")]
    pub type_class: Option<ClassRef>,
    #[serde(rename = "parameterizedClass")]
    pub parameterized_class: Option<ClassRef>,
    #[serde(rename = "returnType")]
    pub return_type: Option<Box<InstanceRef>>,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "typeParameters")]
    pub type_parameters: Option<Vec<InstanceRef>>,
    pub pattern: Option<Box<InstanceRef>>,
    #[serde(rename = "closureFunction")]
    pub closure_function: Option<FunctionRef>,
    #[serde(rename = "closureContext")]
    pub closure_context: Option<ContextRef>,
    #[serde(rename = "closureReceiver")]
    pub closure_receiver: Option<Box<InstanceRef>>,
    #[serde(rename = "portId")]
    pub port_id: Option<u32>,
    #[serde(rename = "allocationLocation")]
    pub allocation_location: Option<Box<InstanceRef>>,
    #[serde(rename = "debugName")]
    pub debug_name: Option<String>,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Instance {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub size: Option<u32>,
    pub kind: InstanceKind,
    #[serde(rename = "identityHashCode")]
    pub identity_hash_code: u32,
    pub class: ClassRef,
    #[serde(rename = "valueAsString")]
    pub value_as_string: Option<String>,
    #[serde(rename = "valueAsStringIsTruncated")]
    pub value_as_string_is_truncated: Option<bool>,
    pub length: Option<u32>,
    pub offset: Option<u32>,
    pub count: Option<u32>,
    pub name: Option<String>,
    #[serde(rename = "typeClass")]
    pub type_class: Option<ClassRef>,
    #[serde(rename = "parameterizedClass")]
    pub parameterized_class: Option<ClassRef>,
    #[serde(rename = "returnType")]
    pub return_type: Option<Box<InstanceRef>>,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "typeParameters")]
    pub type_parameters: Option<Vec<InstanceRef>>,
    pub fields: Vec<BoundField>,
    pub elements: Option<Vec<InstanceRefOrSentinel>>,
    pub associations: Option<Vec<MapAssociation>>,
    pub bytes: Option<String>,
    #[serde(rename = "mirrorReferent")]
    pub mirror_referent: Option<ObjectRef>,
    pub pattern: Option<Box<InstanceRef>>,
    #[serde(rename = "closureFunction")]
    pub closure_function: Option<FunctionRef>,
    #[serde(rename = "closureContext")]
    pub closure_context: Option<ContextRef>,
    #[serde(rename = "closureReceiver")]
    pub closure_receiver: Option<Box<InstanceRef>>,
    #[serde(rename = "isCaseSensitive")]
    pub is_case_sensitive: Option<bool>,
    #[serde(rename = "isMultiLine")]
    pub is_multi_line: Option<bool>,
    #[serde(rename = "propertyKey")]
    pub property_key: Option<ObjectRef>,
    #[serde(rename = "propertyValue")]
    pub property_value: Option<ObjectRef>,
    pub target: Option<ObjectRef>,
    #[serde(rename = "typeArguments")]
    pub type_arguments: Option<TypeArgumentsRef>,
    #[serde(rename = "parameterIndex")]
    pub parameter_index: Option<u32>,
    #[serde(rename = "targetType")]
    pub target_type: Option<Box<InstanceRef>>,
    pub bound: Option<Box<InstanceRef>>,
    #[serde(rename = "portId")]
    pub port_id: Option<u32>,
    #[serde(rename = "allocationLocation")]
    pub allocation_location: Option<Box<InstanceRef>>,
    #[serde(rename = "debugName")]
    pub debug_name: Option<String>,
    pub label: Option<String>,
    pub callback: Option<InstanceRef>,
    #[serde(rename = "callbackAddress")]
    pub callback_address: Option<InstanceRef>,
    #[serde(rename = "allEntries")]
    pub all_entries: Option<InstanceRef>,
    pub value: Option<InstanceRef>,
    pub token: Option<InstanceRef>,
    pub detach: Option<InstanceRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InstanceKind {
    PlainInstance,
    Null,
    Bool,
    Double,
    Int,
    String,
    List,
    Map,
    Set,
    Float32x4,
    Float64x2,
    Int32x4,
    Uint8ClampedList,
    Uint8List,
    Uint16List,
    Uint32List,
    Uint64List,
    Int8List,
    Int16List,
    Int32List,
    Int64List,
    Float32List,
    Float64List,
    Int32x4List,
    Float32x4List,
    Float64x2List,
    Record,
    StackTrace,
    Closure,
    MirrorReference,
    RegExp,
    WeakProperty,
    WeakReference,
    Type,
    TypeParameter,
    TypeRef,
    FunctionType,
    RecordType,
    BoundedType,
    ReceivePort,
    UserTag,
    Finalizer,
    NativeFinalizer,
    FinalizerEntry,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IsolateRef {
    pub r#type: String,
    pub id: String,
    pub number: String,
    pub name: String,
    #[serde(rename = "isSystemIsolate")]
    pub is_system_isolate: bool,
    #[serde(rename = "isolateGroupId")]
    pub isolate_group_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Isolate {
    pub r#type: String,
    pub id: String,
    pub number: String,
    pub name: String,
    #[serde(rename = "isSystemIsolate")]
    pub is_system_isolate: bool,
    #[serde(rename = "isolateGroupId")]
    pub isolate_group_id: String,
    #[serde(rename = "isolateFlags")]
    pub isolate_flags: Vec<IsolateFlag>,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    pub runnable: bool,
    #[serde(rename = "livePorts")]
    pub live_ports: u32,
    #[serde(rename = "pauseOnExit")]
    pub pause_on_exit: bool,
    #[serde(rename = "pauseEvent")]
    pub pause_event: Event,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IsolateFlag {
    pub name: String,
    #[serde(rename = "valueAsString")]
    pub value_as_string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IsolateGroupRef {
    pub r#type: String,
    pub id: String,
    pub number: String,
    pub name: String,
    #[serde(rename = "isSystemIsolateGroup")]
    pub is_system_isolate_group: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IsolateGroup {
    pub r#type: String,
    pub id: String,
    pub number: String,
    pub name: String,
    #[serde(rename = "isSystemIsolateGroup")]
    pub is_system_isolate_group: bool,
    pub isolates: Vec<IsolateRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InboundReferences {
    pub r#type: String,
    pub references: Vec<InboundReference>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InboundReference {
    pub source: Object,
    #[serde(rename = "parentListIndex")]
    pub parent_list_index: Option<u32>,
    #[serde(rename = "parentField")]
    pub parent_field: Option<FieldRefOrStringOrInt>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InstanceSet {
    pub r#type: String,
    #[serde(rename = "totalCount")]
    pub total_count: u32,
    pub instances: Vec<ObjectRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LibraryRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub name: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Library {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub name: String,
    pub uri: String,
    pub debuggable: bool,
    pub dependencies: Vec<LibraryDependency>,
    pub scripts: Vec<ScriptRef>,
    pub variables: Vec<FieldRef>,
    pub functions: Vec<FunctionRef>,
    pub classes: Vec<ClassRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LibraryDependency {
    #[serde(rename = "isImport")]
    pub is_import: bool,
    #[serde(rename = "isDeferred")]
    pub is_deferred: bool,
    pub prefix: Option<String>,
    pub target: LibraryRef,
    pub shows: Option<Vec<String>>,
    pub hides: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LogRecord {
    pub r#type: String,
    pub message: InstanceRef,
    pub time: u32,
    pub level: u32,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u32,
    #[serde(rename = "loggerName")]
    pub logger_name: InstanceRef,
    pub zone: InstanceRef,
    pub error: InstanceRef,
    #[serde(rename = "stackTrace")]
    pub stack_trace: InstanceRef,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MapAssociation {
    pub key: InstanceRefOrSentinel,
    pub value: InstanceRefOrSentinel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MemoryUsage {
    pub r#type: String,
    #[serde(rename = "externalUsage")]
    pub external_usage: u32,
    #[serde(rename = "heapCapacity")]
    pub heap_capacity: u32,
    #[serde(rename = "heapUsage")]
    pub heap_usage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub r#type: String,
    pub index: u32,
    pub name: String,
    #[serde(rename = "messageObjectId")]
    pub message_object_id: String,
    pub size: u32,
    pub handler: Option<FunctionRef>,
    pub location: Option<SourceLocation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NativeFunction {
    pub r#type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NullRef {
    pub r#type: String,
    #[serde(rename = "valueAsString")]
    pub value_as_string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Null {
    pub r#type: String,
    #[serde(rename = "valueAsString")]
    pub value_as_string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ObjectRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Object {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Parameter {
    #[serde(rename = "parameterType")]
    pub parameter_type: InstanceRef,
    pub fixed: bool,
    pub name: Option<String>,
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerfettoCpuSamples {
    pub r#type: String,
    #[serde(rename = "samplePeriod")]
    pub sample_period: u32,
    #[serde(rename = "maxStackDepth")]
    pub max_stack_depth: u32,
    #[serde(rename = "sampleCount")]
    pub sample_count: u32,
    #[serde(rename = "timeOriginMicros")]
    pub time_origin_micros: u32,
    #[serde(rename = "timeExtentMicros")]
    pub time_extent_micros: u32,
    pub pid: u32,
    pub samples: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerfettoTimeline {
    pub r#type: String,
    pub trace: String,
    #[serde(rename = "timeOriginMicros")]
    pub time_origin_micros: u32,
    #[serde(rename = "timeExtentMicros")]
    pub time_extent_micros: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PortList {
    pub r#type: String,
    pub ports: Vec<InstanceRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfileFunction {
    pub kind: String,
    #[serde(rename = "inclusiveTicks")]
    pub inclusive_ticks: u32,
    #[serde(rename = "exclusiveTicks")]
    pub exclusive_ticks: u32,
    #[serde(rename = "resolvedUrl")]
    pub resolved_url: String,
    pub function: Box<FunctionRefOrNativeFunction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProtocolList {
    pub r#type: String,
    pub protocols: Vec<Protocol>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Protocol {
    pub protocol_name: String,
    pub major: u32,
    pub minor: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProcessMemoryUsage {
    pub r#type: String,
    pub root: ProcessMemoryItem,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProcessMemoryItem {
    pub name: String,
    pub description: String,
    pub size: u32,
    pub children: Vec<ProcessMemoryItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReloadReport {
    pub r#type: String,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RetainingObject {
    pub value: Object,
    #[serde(rename = "parentListIndex")]
    pub parent_list_index: Option<u32>,
    #[serde(rename = "parentMapKey")]
    pub parent_map_key: Option<ObjectRef>,
    #[serde(rename = "parentField")]
    pub parent_field: Option<FieldRefOrStringOrInt>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RetainingPath {
    pub length: u32,
    #[serde(rename = "gcRootType")]
    pub gc_root_type: String,
    pub elements: Vec<RetainingObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sentinel {
    pub r#type: String,
    pub kind: SentinelKind,
    #[serde(rename = "valueAsString")]
    pub value_as_string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SentinelKind {
    Collected,
    Expired,
    NotInitialized,
    BeingInitialized,
    OptimizedOut,
    Free,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Script {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<Box<ClassRef>>,
    pub size: Option<u32>,
    pub uri: String,
    pub library: LibraryRef,
    #[serde(rename = "lineOffset")]
    pub line_offset: Option<u32>,
    #[serde(rename = "columnOffset")]
    pub column_offset: Option<u32>,
    pub source: Option<String>,
    #[serde(rename = "tokenPosTable")]
    pub token_pos_table: Option<Vec<Vec<u32>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScriptList {
    pub r#type: String,
    pub scripts: Vec<ScriptRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub r#type: String,
    pub script: ScriptRef,
    #[serde(rename = "tokenPos")]
    pub token_pos: u32,
    #[serde(rename = "endTokenPos")]
    pub end_token_pos: Option<u32>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SourceReport {
    pub r#type: String,
    pub ranges: Vec<SourceReportRange>,
    pub scripts: Vec<ScriptRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SourceReportCoverage {
    pub hits: Vec<u32>,
    pub misses: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SourceReportKind {
    Coverage,
    PossibleBreakpoints,
    BranchCoverage,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SourceReportRange {
    #[serde(rename = "scriptIndex")]
    pub script_index: u32,
    #[serde(rename = "startPos")]
    pub start_pos: u32,
    #[serde(rename = "endPos")]
    pub end_pos: u32,
    pub compiled: bool,
    pub error: Option<Error>,
    pub coverage: Option<SourceReportCoverage>,
    #[serde(rename = "possibleBreakpoints")]
    pub possible_breakpoints: Option<Vec<u32>>,
    #[serde(rename = "branchCoverage")]
    pub branch_coverage: Option<SourceReportCoverage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stack {
    pub r#type: String,
    pub frames: Vec<Frame>,
    #[serde(rename = "asyncCausalFrames")]
    pub async_causal_frames: Option<Vec<Frame>>,
    #[serde(rename = "awaiterFrames")]
    pub awaiter_frames: Option<Vec<Frame>>,
    pub messages: Vec<Message>,
    pub truncated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExceptionPauseMode {
    None,
    Unhandled,
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StepOption {
    Into,
    Over,
    OverAsyncSuspension,
    Out,
    Rewind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Success {
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Timeline {
    pub r#type: String,
    #[serde(rename = "traceEvents")]
    pub trace_events: Vec<TimelineEvent>,
    #[serde(rename = "timeOriginMicros")]
    pub time_origin_micros: u32,
    #[serde(rename = "timeExtentMicros")]
    pub time_extent_micros: u32,
}

pub type TimelineEvent = Map<String, Value>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimelineFlags {
    pub r#type: String,
    #[serde(rename = "recorderName")]
    pub recorder_name: String,
    #[serde(rename = "availableStreams")]
    pub available_streams: Vec<String>,
    #[serde(rename = "recordedStreams")]
    pub recorded_streams: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Timestamp {
    pub r#type: String,
    pub timestamp: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeArgumentsRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeArguments {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub name: String,
    pub types: Vec<InstanceRef>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeParametersRef {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeParameters {
    pub r#type: String,
    pub id: String,
    #[serde(rename = "fixedId")]
    pub fixed_id: Option<bool>,
    pub class: Option<ClassRef>,
    pub size: Option<u32>,
    pub names: InstanceRef,
    pub bounds: TypeArgumentsRef,
    pub defaults: TypeArgumentsRef,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UnresolvedSouceLocation {
    pub r#type: String,
    pub script: Option<ScriptRef>,
    #[serde(rename = "scriptUri")]
    pub script_uri: Option<String>,
    #[serde(rename = "tokenPos")]
    pub token_pos: Option<u32>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UriList {
    pub r#type: String,
    pub uris: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Version {
    pub r#type: String,
    pub major: u32,
    pub minor: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VMRef {
    pub r#type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VM {
    pub r#type: String,
    pub name: String,
    #[serde(rename = "architectureBits")]
    pub architecture_bits: u32,
    #[serde(rename = "hostCPU")]
    pub host_cpu: String,
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    #[serde(rename = "targetCPU")]
    pub target_cpu: String,
    pub version: String,
    pub pid: u32,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    pub isolates: Vec<IsolateRef>,
    #[serde(rename = "isolateGroups")]
    pub isolate_groups: Vec<IsolateGroupRef>,
    #[serde(rename = "systemIsolates")]
    pub system_isolates: Vec<IsolateRef>,
    #[serde(rename = "systemIsolateGroups")]
    pub system_isolate_groups: Vec<IsolateGroupRef>,
}

/* ----------------- */
/* Combination types */
/* ----------------- */

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BreakpointOrSentinel {
    Breakpoint(Breakpoint),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InstanceRefOrSentinel {
    InstanceRef(InstanceRef),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InstanceRefOrSentinelOrErrorRef {
    InstanceRef(InstanceRef),
    Sentinel(Sentinel),
    ErrorRef(ErrorRef),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum LibraryRefOrClassRefOrFunctionRef {
    LibraryRef(LibraryRef),
    ClassRef(ClassRef),
    FunctionRef(FunctionRef),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StringOrInt {
    String(String),
    Int(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SourceLocationOrUnresolvedSouceLocation {
    SourceLocation(SourceLocation),
    UnresolvedSouceLocation(UnresolvedSouceLocation),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum FieldRefOrStringOrInt {
    FieldRef(FieldRef),
    String(String),
    Int(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum FunctionRefOrNativeFunction {
    FunctionRef(FunctionRef),
    NativeFunction(NativeFunction),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ObjectRefOrNativeFunction {
    ObjectRef(ObjectRef),
    NativeFunction(NativeFunction),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InstanceRefOrSentinelOrString {
    InstanceRef(InstanceRef),
    Sentinel(Sentinel),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SuccessOrSentinel {
    Success(Success),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AllocationProfileOrSentinel {
    AllocationProfile(AllocationProfile),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ClassListOrSentinel {
    ClassList(ClassList),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum CpuSamplesOrSentinel {
    CpuSamples(CpuSamplesEvent),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InboundReferencesOrSentinel {
    InboundReferences(InboundReferences),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum InstanceSetOrSentinel {
    InstanceSet(InstanceSet),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum IsolateOrSentinel {
    Isolate(Isolate),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum IsolateGroupOrSentinel {
    IsolateGroup(IsolateGroup),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum EventOrSentinel {
    Event(Event),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MemoryUsageOrSentinel {
    MemoryUsage(MemoryUsage),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ScriptListOrSentinel {
    ScriptList(ScriptList),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ObjectOrSentinel {
    Object(Object),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PerfettoCpuSamplesOrSentinel {
    PerfettoCpuSamples(PerfettoCpuSamples),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RetainingPathOrSentinel {
    RetainingPath(RetainingPath),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum StackOrSentinel {
    Stack(Stack),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SourceReportOrSentinel {
    SourceReport(SourceReport),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ReloadReportOrSentinel {
    ReloadReport(ReloadReport),
    Sentinel(Sentinel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SuccessOrError {
    Success(Success),
    Error(Error),
}
