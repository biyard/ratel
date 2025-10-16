use crate::types::*;
use bdk::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]
pub struct SprintLeaguePlayer {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub description: String,

    pub player_images: PlayerImage,

    pub voter: i64,
}

impl SprintLeaguePlayer {
    pub fn new(
        pk: Partition,
        name: String,
        description: String,
        player_images: PlayerImage,
    ) -> crate::Result<Self> {
        let uuid = Uuid::new_v4().to_string();
        if !matches!(pk, Partition::SprintLeague(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "SprintLeaguePlayer must be under SprintLeague partition".to_string(),
            ));
        }

        Ok(Self {
            pk,
            sk: EntityType::SprintLeaguePlayer(uuid),
            player_images,
            name,
            description,
            voter: 0,
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct PlayerImage {
    pub select: SpriteSheet,
    pub run: SpriteSheet,
    pub win: String,
    pub lose: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SpriteSheet {
    // For Animation
    pub json: String,
    pub image: String,
}
