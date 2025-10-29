use bdk::prelude::*;
use serde::Deserialize;

use crate::features::spaces::panels::SpacePanelResponse;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListPanelQueryParams {
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ListPanelResponse {
    pub panels: Vec<SpacePanelResponse>,
    pub bookmark: Option<String>,
}
