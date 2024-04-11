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
    //
    /// "object_group" specifying what group is used to manage lifetimes of
    /// object references in the returned JSON (which is freed by calling dispose_group)
    /// If "object_group" is omitted, the returned JSON will not include any object
    /// references to avoid leaking memory.
    ///
    /// You can use any string as the object group name.
    /// For example, official devtool is using the following object group names:
    ///   selection_1, selection_2, ...
    ///   tree_1, tree_2, tree_3, ...
    ///   console_1, console_2, ...
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

    fn structured_errors(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn show(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn track_rebuild_dirty_widgets(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn track_repaint_widgets(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> impl Future<Output = Result<Togglable>> + Send;

    fn dispose_all_groups(
        &self,
        isolate_id: &str,
        object_group: &str,
    ) -> impl Future<Output = Result<Response>> + Send;

    fn dispose_group(
        &self,
        isolate_id: &str,
        object_group: &str,
    ) -> impl Future<Output = Result<Response>> + Send;

    fn is_widget_tree_ready(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<ResultResponse<bool>>> + Send;

    fn dispose_id(
        &self,
        isolate_id: &str,
        object_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<Response>> + Send;

    #[deprecated(
        note = "Use add_pub_root_directories instead. This feature was deprecated after v3.18.0-2.0.pre."
    )]
    fn set_pub_root_directories(
        &self,
        isolate_id: &str,
        args: Vec<&str>,
    ) -> impl Future<Output = Result<Response>> + Send;

    fn add_pub_root_directories(
        &self,
        isolate_id: &str,
        args: Vec<&str>,
    ) -> impl Future<Output = Result<Response>> + Send;

    fn remove_pub_root_directories(
        &self,
        isolate_id: &str,
        args: Vec<&str>,
    ) -> impl Future<Output = Result<Response>> + Send;

    fn get_pub_root_directories(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<ResultResponse<Vec<String>>>> + Send;

    fn set_selection_by_id(
        &self,
        isolate_id: &str,
        object_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<bool>>> + Send;

    fn get_parent_chain(
        &self,
        isolate_id: &str,
        object_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<Vec<DiagnosticPathNode>>>> + Send;

    fn get_properties(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<Vec<DiagnosticNode>>>> + Send;

    fn get_children(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<Vec<DiagnosticNode>>>> + Send;

    fn get_children_summary_tree(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<Vec<DiagnosticNode>>>> + Send;

    fn get_children_details_subtree(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<Vec<DiagnosticNode>>>> + Send;

    fn get_root_widget(
        &self,
        isolate_id: &str,
        object_group: Option<&str>,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn get_root_widget_summary_tree(
        &self,
        isolate_id: &str,
        object_group: Option<&str>,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn get_root_widget_summary_tree_with_previews(
        &self,
        isolate_id: &str,
        object_group: Option<&str>,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn get_details_subtree(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        subtree_depth: Option<i64>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn get_selected_widget(
        &self,
        isolate_id: &str,
        previous_selection_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn get_selected_summary_widget(
        &self,
        isolate_id: &str,
        previous_selection_id: Option<&str>,
        object_group: &str,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn is_widget_creation_tracked(
        &self,
        isolate_id: &str,
    ) -> impl Future<Output = Result<ResultResponse<bool>>> + Send;

    #[allow(clippy::too_many_arguments)]
    fn screenshot(
        &self,
        isolate_id: &str,
        id: &str,
        width: f64,
        height: f64,
        margin: Option<f64>,
        max_pixel_ratio: Option<f64>,
        debug_paint: Option<bool>,
    ) -> impl Future<Output = Result<ResultResponse<Option<String>>>> + Send;

    fn get_layout_explorer_node(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        object_group: &str,
        subtree_depth: Option<i64>,
    ) -> impl Future<Output = Result<ResultResponse<DiagnosticNode>>> + Send;

    fn set_flex_fit(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        flex_fit: FlexFit,
    ) -> impl Future<Output = Result<ResultResponse<bool>>> + Send;

    fn set_flex_factor(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        flex_factor: i64,
    ) -> impl Future<Output = Result<ResultResponse<bool>>> + Send;

    fn set_flex_properties(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        main_axis_alignment: MainAxisAlignment,
        cross_axis_alignment: CrossAxisAlignment,
    ) -> impl Future<Output = Result<ResultResponse<bool>>> + Send;
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResultResponse<R> {
    pub r#type: String,
    pub method: String,
    pub result: R,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DiagnosticPathNode {
    pub node: DiagnosticNode,
    pub children: Vec<DiagnosticPathNode>,
    #[serde(rename = "childIndex")]
    pub child_index: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DiagnosticNode {
    pub description: Option<String>,
    pub r#type: String,
    pub name: Option<String>,
    #[serde(rename = "showSeparator")]
    pub show_separator: Option<bool>,
    pub level: Option<DiagnosticLevel>,
    #[serde(rename = "showName")]
    pub show_name: Option<bool>,
    #[serde(rename = "emptyBodyDescription")]
    pub empty_body_description: Option<String>,
    pub style: Option<DiagnosticsTreeStyle>,
    #[serde(rename = "allowTruncate")]
    pub allow_truncate: Option<bool>,
    #[serde(rename = "hasChildren")]
    pub has_children: Option<bool>,
    #[serde(rename = "linePrefix")]
    pub line_prefix: Option<String>,
    #[serde(rename = "allowWrap")]
    pub allow_wrap: Option<bool>,
    #[serde(rename = "allowNameWrap")]
    pub allow_name_wrap: Option<bool>,
    #[serde(rename = "valueId")]
    pub value_id: Option<String>,
    #[serde(rename = "summaryTree")]
    pub summary_tree: Option<bool>,
    #[serde(rename = "locationId")]
    pub location_id: Option<i64>,
    #[serde(rename = "creationLocation")]
    pub creation_location: Option<Location>,
    #[serde(rename = "createdByLocalProject")]
    pub created_by_local_project: Option<bool>,
    pub properties: Option<Vec<DiagnosticNode>>,
    pub children: Option<Vec<DiagnosticNode>>,
    pub truncated: Option<bool>,
    #[serde(rename = "renderObject")]
    pub render_object: Option<Box<DiagnosticNode>>,
    #[serde(rename = "parentRenderElement")]
    pub parent_render_element: Option<Box<DiagnosticNode>>,
    pub constraints: Option<Constraints>,
    #[serde(rename = "isBox")]
    pub is_box: Option<bool>,
    pub size: Option<Size>,
    #[serde(rename = "flexFactor")]
    pub flex_factor: Option<i64>,
    #[serde(rename = "flexFit")]
    pub flex_fit: Option<FlexFit>,
    #[serde(rename = "parentData")]
    pub parent_data: Option<Offset>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "fine")]
    Fine,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "hint")]
    Hint,
    #[serde(rename = "summary")]
    Summary,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "off")]
    Off,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DiagnosticsTreeStyle {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "sparse")]
    Sparse,
    #[serde(rename = "offstage")]
    Offstage,
    #[serde(rename = "dense")]
    Dense,
    #[serde(rename = "transition")]
    Transition,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "whitespace")]
    Whitespace,
    #[serde(rename = "flat")]
    Flat,
    #[serde(rename = "singleLine")]
    SingleLine,
    #[serde(rename = "errorProperty")]
    ErrorProperty,
    #[serde(rename = "shallow")]
    Shallow,
    #[serde(rename = "truncateChildren")]
    TruncateChildren,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FlexFit {
    #[serde(rename = "tight")]
    Tight,
    #[serde(rename = "loose")]
    Loose,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MainAxisAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "spaceBetween")]
    SpaceBetween,
    #[serde(rename = "spaceAround")]
    SpaceAround,
    #[serde(rename = "spaceEvenly")]
    SpaceEvenly,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CrossAxisAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "stretch")]
    Stretch,
    #[serde(rename = "baseline")]
    Baseline,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Location {
    pub file: String,
    pub line: i64,
    pub column: i64,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Constraints {
    pub r#type: String,
    pub description: String,
    #[serde(rename = "minWidth")]
    pub min_width: Option<String>,
    #[serde(rename = "maxWidth")]
    pub max_width: Option<String>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "maxHeight")]
    pub max_height: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Size {
    pub width: String,
    pub height: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Offset {
    #[serde(rename = "offsetX")]
    pub x: String,
    #[serde(rename = "offsetY")]
    pub y: String,
}

#[cfg(test)]
mod test {
    use crate::vm_service::VmServiceResponse;

    use super::*;

    #[test]
    fn parse_diagnostic_node() {
        let response = include_str!("../../test/get_root_widget_response.txt");
        let node =
            serde_json::from_str::<VmServiceResponse<ResultResponse<DiagnosticNode>>>(response);
        assert!(node.is_ok());
    }
}
