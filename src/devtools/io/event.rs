use super::types::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VmServiceEvent {
    pub jsonrpc: String,
    pub method: String,
    pub params: Event,
}
