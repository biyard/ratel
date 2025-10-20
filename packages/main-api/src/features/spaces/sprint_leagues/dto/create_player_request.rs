use bdk::prelude::*;

use super::super::PlayerImage;

#[derive(Debug, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct CreatePlayerRequest {
    pub name: String,
    pub description: String,
    pub player_image: PlayerImage,
}
