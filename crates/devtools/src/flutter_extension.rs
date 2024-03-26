use crate::{
    params,
    protocols::flutter_extension::{
        DisplayRefreshRate, Dump, FlutterExtensionProtocol, FlutterViewList, TimeDilation,
        Togglable, Value,
    },
    vm_service::VmService,
};
use color_eyre::Result;
use serde_json::Map;

impl FlutterExtensionProtocol for VmService {
    async fn list_views(&self) -> Result<FlutterViewList> {
        self.call("_flutter.listViews", Map::new()).await
    }

    async fn get_display_refresh_rate(&self, view_id: &str) -> Result<DisplayRefreshRate> {
        let params = params! {
            "viewId".to_owned() => view_id.into(),
        };
        self.call("_flutter.getDisplayRefreshRate", params).await
    }

    async fn invert_oversized_image(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.invertOversizedImages", params).await
    }

    async fn debug_paint(&self, isolate_id: &str, enabled: Option<bool>) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.debugPaint", params).await
    }

    async fn debug_paint_baseline_enabled(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.debugPaintBaselinesEnabled", params)
            .await
    }

    async fn repaint_rainbow(&self, isolate_id: &str, enabled: Option<bool>) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.repaintRainbow", params).await
    }

    async fn debug_dump_layer_tree(&self, isolate_id: &str) -> Result<Dump> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.debugDumpLayerTree", params).await
    }

    async fn debug_disable_physical_shape_layers(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.debugDisablePhysicalShapeLayers", params)
            .await
    }

    async fn debug_disable_opacity_layers(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.debugDisableOpacityLayers", params)
            .await
    }

    async fn debug_dump_render_tree(&self, isolate_id: &str) -> Result<Dump> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.debugDumpRenderTree", params).await
    }

    async fn debug_dump_semantics_tree_in_traversal_order(&self, isolate_id: &str) -> Result<Dump> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.debugDumpSemanticsTreeInTraversalOrder", params)
            .await
    }

    async fn debug_dump_semantics_tree_in_inverse_hit_test_order(
        &self,
        isolate_id: &str,
    ) -> Result<Dump> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call(
            "ext.flutter.debugDumpSemanticsTreeInInverseHitTestOrder",
            params,
        )
        .await
    }

    async fn profile_render_object_paints(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.profileRenderObjectPaints", params)
            .await
    }

    async fn profile_render_object_layouts(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.profileRenderObjectLayouts", params)
            .await
    }

    async fn time_dilation(&self, isolate_id: &str, value: Option<String>) -> Result<TimeDilation> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.timeDilation", params).await
    }

    async fn profile_platform_channels(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.profilePlatformChannels", params)
            .await
    }

    async fn evict(&self, isolate_id: &str, value: String) -> Result<Value> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.evict", params).await
    }
}
