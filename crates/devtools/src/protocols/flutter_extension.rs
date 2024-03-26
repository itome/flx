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
