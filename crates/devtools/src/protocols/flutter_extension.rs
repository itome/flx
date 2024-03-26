use color_eyre::Result;
use futures::Future;
use serde::{Deserialize, Serialize};

pub trait FlutterExtensionProtocol {
    fn list_views(&self) -> impl Future<Output = Result<FlutterViewList>> + Send;

    fn get_display_refresh_rate(
        &self,
        view_id: &str,
    ) -> impl Future<Output = Result<DisplayRefreshRate>> + Send;
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
