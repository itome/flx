use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emulator {
    pub id: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "platformType")]
    pub platform_type: String,
}
