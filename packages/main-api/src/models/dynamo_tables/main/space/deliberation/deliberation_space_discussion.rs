use crate::{models::user::User, types::*};

use bdk::prelude::*;

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
