use crate::types::SpaceVisibility;

use super::*;
use bdk::prelude::*;

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct DeliberationResponse {
    pub pk: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub likes: i64,
    pub comments: i64,
    pub rewards: i64,
    pub shares: i64,

    pub visibility: SpaceVisibility,
    pub title: String,

    pub post_pk: String,
    pub user_pk: String,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct DeliberationDetailResponse {
    #[serde(flatten)]
    pub deliberation: DeliberationResponse,

    pub summary: DeliberationSummaryResponse,
    pub recommendation: DeliberationRecommentationResponse,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum DeliberationMetadata {
    DeliberationSpace(DeliberationSpace),
    DeliberationSpaceSurvey(DeliberationSpaceSurvey),
    DeliberationSpaceSummary(DeliberationSpaceSummary),
    DeliberationSpaceResponse(DeliberationSpaceResponse),
    DeliberationSpaceRecommendation(DeliberationSpaceRecommendation),
    DeliberationSpaceQuestion(DeliberationSpaceQuestion),
    DeliberationSpaceParticipant(DeliberationSpaceParticipant),
    DeliberationSpaceMember(DeliberationSpaceMember),
    DeliberationSpaceElearning(DeliberationSpaceElearning),
    DeliberationSpaceDiscussion(DeliberationSpaceDiscussion),
}
