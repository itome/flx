use crate::{
    params,
    protocols::flutter_extension::{
        CrossAxisAlignment, DiagnosticNode, DiagnosticPathNode, DisplayRefreshRate, Dump, FlexFit,
        FlutterExtensionProtocol, FlutterViewList, MainAxisAlignment, Response, ResultResponse,
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

    async fn structured_errors(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.inspector.structuredErrors", params)
            .await
    }

    async fn show(&self, isolate_id: &str, enabled: Option<bool>) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.inspector.show", params).await
    }

    async fn track_rebuild_dirty_widgets(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.inspector.trackRebuildDirtyWidgets", params)
            .await
    }

    async fn track_repaint_widgets(
        &self,
        isolate_id: &str,
        enabled: Option<bool>,
    ) -> Result<Togglable> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "enabled".to_owned() => enabled.into(),
        };
        self.call("ext.flutter.inspector.trackRepaintWidgets", params)
            .await
    }

    async fn dispose_all_groups(&self, isolate_id: &str, object_group: &str) -> Result<Response> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.disposeAllGroups", params)
            .await
    }

    async fn dispose_group(&self, isolate_id: &str, object_group: &str) -> Result<Response> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.disposeGroup", params)
            .await
    }

    async fn is_widget_tree_ready(&self, isolate_id: &str) -> Result<ResultResponse<bool>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.inspector.isWidgetTreeReady", params)
            .await
    }

    async fn dispose_id(
        &self,
        isolate_id: &str,
        object_id: Option<&str>,
        object_group: &str,
    ) -> Result<Response> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => object_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.disposeId", params).await
    }

    async fn set_pub_root_directories(
        &self,
        isolate_id: &str,
        args: Vec<&str>,
    ) -> Result<Response> {
        let mut params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        for (i, arg) in args.iter().enumerate() {
            params.insert(format!("arg{}", i), (*arg).into());
        }
        self.call("ext.flutter.inspector.setPubRootDirectories", params)
            .await
    }

    async fn add_pub_root_directories(
        &self,
        isolate_id: &str,
        args: Vec<&str>,
    ) -> Result<Response> {
        let mut params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        for (i, arg) in args.iter().enumerate() {
            params.insert(format!("arg{}", i), (*arg).into());
        }
        self.call("ext.flutter.inspector.addPubRootDirectories", params)
            .await
    }

    async fn remove_pub_root_directories(
        &self,
        isolate_id: &str,
        args: Vec<&str>,
    ) -> Result<Response> {
        let mut params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        for (i, arg) in args.iter().enumerate() {
            params.insert(format!("arg{}", i), (*arg).into());
        }
        self.call("ext.flutter.inspector.removePubRootDirectories", params)
            .await
    }

    async fn get_pub_root_directories(
        &self,
        isolate_id: &str,
    ) -> Result<ResultResponse<Vec<String>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.inspector.getPubRootDirectories", params)
            .await
    }

    async fn set_selection_by_id(
        &self,
        isolate_id: &str,
        object_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<bool>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => object_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.setSelectionById", params)
            .await
    }

    async fn get_parent_chain(
        &self,
        isolate_id: &str,
        object_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<Vec<DiagnosticPathNode>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => object_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getParentChain", params)
            .await
    }

    async fn get_properties(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<Vec<DiagnosticNode>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => diagnosticable_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getProperties", params)
            .await
    }

    async fn get_children(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<Vec<DiagnosticNode>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => diagnosticable_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getChildren", params).await
    }

    async fn get_children_summary_tree(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<Vec<DiagnosticNode>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => diagnosticable_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getChildrenSummaryTree", params)
            .await
    }

    async fn get_children_details_subtree(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<Vec<DiagnosticNode>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => diagnosticable_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getChildrenDetailsSubtree", params)
            .await
    }

    async fn get_root_widget(
        &self,
        isolate_id: &str,
        object_group: Option<&str>,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getRootWidget", params)
            .await
    }

    async fn get_root_widget_summary_tree(
        &self,
        isolate_id: &str,
        object_group: Option<&str>,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getRootWidgetSummaryTree", params)
            .await
    }

    async fn get_root_widget_summary_tree_with_previews(
        &self,
        isolate_id: &str,
        object_group: Option<&str>,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call(
            "ext.flutter.inspector.getRootWidgetSummaryTreeWithPreviews",
            params,
        )
        .await
    }

    async fn get_details_subtree(
        &self,
        isolate_id: &str,
        diagnosticable_id: Option<&str>,
        subtree_depth: Option<i64>,
        object_group: &str,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => diagnosticable_id.into(),
            "subtreeDepth".to_owned() => subtree_depth.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getDetailsSubtree", params)
            .await
    }

    async fn get_selected_widget(
        &self,
        isolate_id: &str,
        previous_selection_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => previous_selection_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getSelectedWidget", params)
            .await
    }

    async fn get_selected_summary_widget(
        &self,
        isolate_id: &str,
        previous_selection_id: Option<&str>,
        object_group: &str,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "arg".to_owned() => previous_selection_id.into(),
            "objectGroup".to_owned() => object_group.into(),
        };
        self.call("ext.flutter.inspector.getSelectedSummaryWidget", params)
            .await
    }

    async fn is_widget_creation_tracked(&self, isolate_id: &str) -> Result<ResultResponse<bool>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
        };
        self.call("ext.flutter.inspector.isWidgetCreationTracked", params)
            .await
    }

    async fn screenshot(
        &self,
        isolate_id: &str,
        id: &str,
        width: f64,
        height: f64,
        margin: Option<f64>,
        max_pixel_ratio: Option<f64>,
        debug_paint: Option<bool>,
    ) -> Result<ResultResponse<Option<String>>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
            "width".to_owned() => width.to_string().into(),
            "height".to_owned() => height.to_string().into(),
            "margin".to_owned() => margin.map(|m| m.to_string()).into(),
            "maxPixelRatio".to_owned() => max_pixel_ratio.map(|m| m.to_string()).into(),
            "debugPaint".to_owned() => debug_paint.map(|d| d.to_string()).into(),
        };
        self.call("ext.flutter.inspector.screenshot", params).await
    }

    async fn get_layout_explorer_node(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        object_group: &str,
        subtree_depth: Option<i64>,
    ) -> Result<ResultResponse<DiagnosticNode>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
            "groupName".to_owned() => object_group.into(),
            "subtreeDepth".to_owned() => subtree_depth.map(|e| e.to_string()).into(),
        };
        self.call("ext.flutter.inspector.getLayoutExplorerNode", params)
            .await
    }

    async fn set_flex_fit(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        flex_fit: FlexFit,
    ) -> Result<ResultResponse<bool>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
            "flexFit".to_owned() => serde_json::to_string(&flex_fit).unwrap().into(),
        };
        self.call("ext.flutter.inspector.setFlexFit", params).await
    }

    async fn set_flex_factor(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        flex_factor: i64,
    ) -> Result<ResultResponse<bool>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
            "flexFactor".to_owned() => flex_factor.to_string().into(),
        };
        self.call("ext.flutter.inspector.setFlexFactor", params)
            .await
    }

    async fn set_flex_properties(
        &self,
        isolate_id: &str,
        id: Option<&str>,
        main_axis_alignment: MainAxisAlignment,
        cross_axis_alignment: CrossAxisAlignment,
    ) -> Result<ResultResponse<bool>> {
        let params = params! {
            "isolateId".to_owned() => isolate_id.into(),
            "id".to_owned() => id.into(),
            "mainAxisAlignment".to_owned() => serde_json::to_string(&main_axis_alignment).unwrap().into(),
            "crossAxisAlignment".to_owned() => serde_json::to_string(&cross_axis_alignment).unwrap().into(),
        };
        self.call("ext.flutter.inspector.setFlexProperties", params)
            .await
    }
}
