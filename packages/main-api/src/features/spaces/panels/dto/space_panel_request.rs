use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
pub struct SpacePanelRequest {
    pub name: String,
    pub quotas: i64,
    pub attributes: Vec<Attribute>,
}
