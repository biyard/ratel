use crate::{
    models::{
        space::{DiscussionMemberResponse, DiscussionParticipantResponse},
        user::User,
    },
    types::*,
};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
pub struct DiscussionCreateRequest {
    pub id: Option<String>,
    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceDiscussion {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,
    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub meeting_id: Option<String>,
    pub pipeline_id: String,

    pub media_pipeline_arn: Option<String>,
    pub record: Option<String>,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl DeliberationSpaceDiscussion {
    pub fn new(
        deliberation_pk: Partition,
        name: String,
        description: String,
        started_at: i64,
        ended_at: i64,
        meeting_id: Option<String>,
        pipeline_id: String,
        media_pipeline_arn: Option<String>,
        record: Option<String>,

        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: deliberation_pk,
            sk: EntityType::DeliberationSpaceDiscussion(uid),
            started_at,
            ended_at,

            name,
            description,
            meeting_id,
            pipeline_id,

            media_pipeline_arn,
            record,

            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, schemars::JsonSchema)]
pub struct DeliberationDiscussionResponse {
    pub pk: String,

    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub meeting_id: Option<String>,
    pub pipeline_id: String,

    pub media_pipeline_arn: Option<String>,
    pub record: Option<String>,

    pub user_pk: String,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub members: Vec<DiscussionMemberResponse>,
    pub participants: Vec<DiscussionParticipantResponse>,
}

impl From<DeliberationSpaceDiscussion> for DeliberationDiscussionResponse {
    fn from(discussion: DeliberationSpaceDiscussion) -> Self {
        let pk = match discussion.sk {
            EntityType::DeliberationSpaceDiscussion(v) => v,
            _ => "".to_string(),
        };

        let user_pk = match discussion.user_pk {
            Partition::User(v) => v,
            Partition::Team(v) => v,
            _ => "".to_string(),
        };
        Self {
            pk,
            started_at: discussion.started_at,
            ended_at: discussion.ended_at,
            name: discussion.name,
            description: discussion.description,
            meeting_id: discussion.meeting_id,
            pipeline_id: discussion.pipeline_id,
            media_pipeline_arn: discussion.media_pipeline_arn,
            record: discussion.record,
            user_pk,
            author_display_name: discussion.author_display_name,
            author_profile_url: discussion.author_profile_url,
            author_username: discussion.author_username,
            members: vec![],
            participants: vec![],
        }
    }
}
