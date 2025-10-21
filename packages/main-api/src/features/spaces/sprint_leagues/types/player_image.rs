use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct PlayerImage {
    pub select: SpriteSheet,
    pub run: SpriteSheet,
    pub win: String,
    pub lose: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SpriteSheet {
    // For Animation
    pub json: String,
    pub image: String,
}
