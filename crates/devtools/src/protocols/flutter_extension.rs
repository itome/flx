use color_eyre::Result;
use futures::Future;
use serde::{Deserialize, Serialize};

pub trait FlutterExtensionProtocol {
    fn list_views(&self) -> impl Future<Output = Result<FlutterViewList>> + Send;

    fn get_display_refresh_rate(
        &self,
        view_id: &str,
    ) -> impl Future<Output = Result<DisplayRefreshRate>> + Send;

    // Service extensions for flutter rendering package
    // https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/rendering/service_extensions.dart

    fn invert_oversized_image(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn debug_paint(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn debug_paint_baseline_enabled(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn repaint_rainbow(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn debug_dump_layer_tree(&self, isolate_id: &str) -> impl Future<Output = Result<Dump>> + Send;

    fn debug_disable_physical_shape_layers(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn debug_disable_opacity_layers(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn debug_dump_render_tree(&self, isolate_id: &str)
        -> impl Future<Output = Result<Dump>> + Send;

    fn debug_dump_semantics_tree_in_traversal_order(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<Dump>> + Send;

    fn debug_dump_semantics_tree_in_inverse_hit_test_order(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<Dump>> + Send;

    fn profile_render_object_paints(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn profile_render_object_layouts(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    // Service extensions for flutter scheduler package
    // https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/scheduler/service_extensions.dart
    fn time_dilation(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> impl Future<Output = Result<TimeDilation>> + Send;

    // Service extensions for flutter services package
    // https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/services/service_extensions.dart
    fn profile_platform_channels(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn evict(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> impl Future<Output = Result<Value>> + Send;

    // Service extensions for flutter foundation package
    // https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/foundation/service_extensions.dart
    fn reassemble(&self, isolate_id: &str) -> impl Future<Output = Result<Response>> + Send;

    fn exit(&self, isolate_id: &str) -> impl Future<Output = Result<Response>> + Send;

    fn connected_vm_service_uri(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> impl Future<Output = Result<Value>> + Send;

    fn active_dev_tools_server_address(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> impl Future<Output = Result<Value>> + Send;

    fn platform_override(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> impl Future<Output = Result<Value>> + Send;

    fn brightness_override(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> impl Future<Output = Result<Value>> + Send;

    // Service extensions for flutter widget package
    // https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/widgets/service_extensions.dart
    fn debug_dump_app(&self, isolate_id: &str) -> impl Future<Output = Result<Dump>> + Send;

    fn debug_dump_focus_tree(&self, isolate_id: &str) -> impl Future<Output = Result<Dump>> + Send;

    fn show_performance_overlay(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn did_send_first_frame_event(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn did_send_first_frame_rasterized_event(
        &self,
        isolate_id: &str,
        enabled: Option<&str>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn profile_widget_builds(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn profile_user_widget_builds(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn debug_allow_banner(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FlutterViewList {
    pub r#type: String,
    pub views: Vec<FlutterView>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FlutterView {
    pub r#type: String,
    pub id: String,
    pub isolate: IsolateRefInFlutterExtension,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DisplayRefreshRate {
    pub r#type: String,
    pub fps: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IsolateRefInFlutterExtension {
    pub r#type: String,
    pub id: String,
    pub name: String,
    pub number: i64,
    #[serde(rename = "fixedId")]
    pub fixed_id: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Togglable {
    pub r#type: String,
    pub enabled: bool,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Dump {
    pub r#type: String,
    pub data: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Value {
    pub r#type: String,
    pub value: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub r#type: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimeDilation {
    pub r#type: String,
    #[serde(rename = "timeDilation")]
    pub time_dilation: f32,
    pub method: String,
}
