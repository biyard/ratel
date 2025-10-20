use crate::features::spaces::discussions::dto::space_discussion_member_response::SpaceDiscussionMemberResponse;
use crate::features::spaces::discussions::dto::space_discussion_participant_response::SpaceDiscussionParticipantResponse;
use crate::features::spaces::discussions::models::space_discussion::SpaceDiscussion;
use crate::types::{EntityType, Partition};
use bdk::prelude::*;

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

    pub is_member: bool,
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
            is_member: false,
            members: vec![],
            participants: vec![],
        }
    }
}
