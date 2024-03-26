use crate::{
    params,
    protocols::flutter_extension::{DisplayRefreshRate, FlutterExtensionProtocol, FlutterViewList},
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
}
