use crate::features::spaces::discussions::dto::space_discussion_request::SpaceDiscussionRequest;
use crate::features::spaces::discussions::dto::space_discussion_response::SpaceDiscussionResponse;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::types::Partition;
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListDiscussionQueryParams {
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDiscussionRequest {
    pub discussion: SpaceDiscussionRequest,
}

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdateDiscussionRequest {
    pub discussion: SpaceDiscussionRequest,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct CreateDiscussionResponse {
    pub discussion: SpaceDiscussionResponse,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DeleteDiscussionResponse {
    pub discussion_pk: Partition,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct GetDiscussionResponse {
    pub discussion: SpaceDiscussionResponse,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ListDiscussionResponse {
    pub discussions: Vec<SpaceDiscussionResponse>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema)]
pub struct ListDiscussionMemberResponse {
    pub members: Vec<SpaceDiscussionMember>,
    // pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema)]
pub struct ListDiscussionParticipantResponse {
    pub participants: Vec<SpaceDiscussionParticipant>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateDiscussionResponse {
    pub discussion: SpaceDiscussionResponse,
}
