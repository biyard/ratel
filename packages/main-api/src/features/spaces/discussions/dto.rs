use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::types::attendee_info::AttendeeInfo;
use crate::types::meeting_info::MeetingInfo;
use crate::types::{EntityType, Partition};
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
pub struct SpaceDiscussionRequest {
    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub user_ids: Vec<Partition>,
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct DiscussionUser {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "PascalCase")]
pub struct MeetingData {
    pub meeting: MeetingInfo,
    pub attendee: AttendeeInfo,
    pub participants: Vec<DiscussionUser>,
    pub record: Option<String>,
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceDiscussionResponse {
    pub pk: Partition,

    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub meeting_id: Option<String>,
    pub pipeline_id: String,

    pub media_pipeline_arn: Option<String>,
    pub record: Option<String>,

    pub members: Vec<SpaceDiscussionMemberResponse>,
    pub participants: Vec<SpaceDiscussionParticipantResponse>,
}

impl From<SpaceDiscussion> for SpaceDiscussionResponse {
    fn from(discussion: SpaceDiscussion) -> Self {
        Self {
            pk: match discussion.sk {
                EntityType::SpaceDiscussion(v) => Partition::Discussion(v.to_string()),
                _ => Partition::Discussion("".to_string()),
            },
            started_at: discussion.started_at,
            ended_at: discussion.ended_at,
            name: discussion.name,
            description: discussion.description,
            meeting_id: discussion.meeting_id,
            pipeline_id: discussion.pipeline_id,
            media_pipeline_arn: discussion.media_pipeline_arn,
            record: discussion.record,
            members: vec![],
            participants: vec![],
        }
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceDiscussionMemberResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl From<SpaceDiscussionMember> for SpaceDiscussionMemberResponse {
    fn from(member: SpaceDiscussionMember) -> Self {
        Self {
            user_pk: member.user_pk,
            author_display_name: member.author_display_name,
            author_profile_url: member.author_profile_url,
            author_username: member.author_username,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceDiscussionParticipantResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub participant_id: String,
}

impl From<SpaceDiscussionParticipant> for SpaceDiscussionParticipantResponse {
    fn from(p: SpaceDiscussionParticipant) -> Self {
        Self {
            user_pk: p.clone().user_pk,
            author_display_name: p.clone().author_display_name,
            author_profile_url: p.clone().author_profile_url,
            author_username: p.clone().author_username,
            participant_id: p.id(),
        }
    }
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
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UpdateDiscussionResponse {
    pub discussion: SpaceDiscussionResponse,
}
