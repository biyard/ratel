use crate::{
    models::space::{DiscussionMemberResponse, DiscussionParticipantResponse},
    types::*,
};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
pub struct SpaceDiscussionCreateRequest {
    pub discussion_pk: Option<String>,
    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub user_ids: Vec<Partition>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceDiscussion {
    pub pk: Partition,
    pub sk: EntityType,
    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub meeting_id: Option<String>,
    pub pipeline_id: String,

    pub media_pipeline_arn: Option<String>,
    pub record: Option<String>,
}

impl SpaceDiscussion {
    pub fn new(
        space_pk: Partition,
        name: String,
        description: String,
        started_at: i64,
        ended_at: i64,
        meeting_id: Option<String>,
        pipeline_id: String,
        media_pipeline_arn: Option<String>,
        record: Option<String>,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: space_pk,
            sk: EntityType::SpaceDiscussion(uid),
            started_at,
            ended_at,

            name,
            description,
            meeting_id,
            pipeline_id,

            media_pipeline_arn,
            record,
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

    pub members: Vec<DiscussionMemberResponse>,
    pub participants: Vec<DiscussionParticipantResponse>,
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
