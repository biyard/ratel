use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceRecommendation {
    pub pk: Partition,
    pub sk: EntityType,

    pub html_contents: String,
}

impl SpaceRecommendation {
    pub fn new(pk: Partition, html_contents: String) -> Self {
        let sk = EntityType::SpaceRecommendation;

        Self {
            pk,
            sk,
            html_contents,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SpaceRecommendationResponse {
    pub html_contents: String,
}

impl From<SpaceRecommendation> for SpaceRecommendationResponse {
    fn from(recommendation: SpaceRecommendation) -> Self {
        Self {
            html_contents: recommendation.clone().html_contents,
        }
    }
}
