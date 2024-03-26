use crate::{
    params,
    protocols::flutter_extension::{
        DisplayRefreshRate, Dump, FlutterExtensionProtocol, FlutterViewList, Response,
        TimeDilation, Togglable, Value,
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

    async fn time_dilation(&self, isolate_id: &str, value: Option<&str>) -> Result<TimeDilation> {
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

    async fn evict(&self, isolate_id: &str, value: Option<&str>) -> Result<Value> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.evict", params).await
    }

    async fn reassemble(&self, isolate_id: &str) -> Result<Response> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.reassemble", params).await
    }

    async fn exit(&self, isolate_id: &str) -> Result<Response> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.exit", params).await
    }

    async fn connected_vm_service_uri(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> Result<Value> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.connectedVmServiceUri", params).await
    }

    async fn active_dev_tools_server_address(
        &self,
        isolate_id: &str,
        value: Option<&str>,
    ) -> Result<Value> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.activeDevToolsServerAddress", params)
            .await
    }

    async fn platform_override(&self, isolate_id: &str, value: Option<&str>) -> Result<Value> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.platformOverride", params).await
    }

    async fn brightness_override(&self, isolate_id: &str, value: Option<&str>) -> Result<Value> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "value".to_owned() => value.into(),
        };
        self.call("ext.flutter.brightnessOverride", params).await
    }

    async fn debug_dump_app(&self, isolate_id: &str) -> Result<Dump> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.debugDumpApp", params).await
    }

    async fn debug_dump_focus_tree(&self, isolate_id: &str) -> Result<Dump> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.debugDumpFocusTree", params).await
    }

    async fn show_performance_overlay(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.showPerformanceOverlay", params)
            .await
    }

    async fn did_send_first_frame_event(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.didSendFirstFrameEvent", params)
            .await
    }

    async fn did_send_first_frame_rasterized_event(
        &self,
        isolate_id: &str,
        enabled: Option<&str>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.didSendFirstFrameRasterizedEvent", params)
            .await
    }

    async fn profile_widget_builds(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.profileWidgetBuilds", params).await
    }

    async fn profile_user_widget_builds(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.profileUserWidgetBuilds", params)
            .await
    }

    async fn debug_allow_banner(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.debugAllowBanner", params).await
    }
}
