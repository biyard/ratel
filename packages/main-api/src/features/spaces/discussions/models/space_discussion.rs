use crate::{
    Error,
    features::spaces::discussions::{
        dto::SpaceDiscussionResponse, models::space_discussion_member::SpaceDiscussionMember,
    },
    types::*,
};

use bdk::prelude::*;

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

    pub fn keys(space_pk: &Partition, discussion_pk: &Partition) -> (Partition, EntityType) {
        let discussion_id = match discussion_pk {
            Partition::Discussion(v) => v.to_string(),
            _ => "".to_string(),
        };

        (space_pk.clone(), EntityType::SpaceDiscussion(discussion_id))
    }

    pub async fn get_discussion(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        discussion_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<SpaceDiscussionResponse, crate::Error2> {
        let (pk, sk) = Self::keys(space_pk, discussion_pk);
        let discussion = SpaceDiscussion::get(&cli, pk.clone(), Some(sk.clone())).await?;
        if discussion.is_none() {
            return Err(crate::Error2::NotFoundDiscussion);
        }

        let mut discussion: SpaceDiscussionResponse = discussion.unwrap().into();

        let (pk, sk) = SpaceDiscussionMember::keys(discussion_pk, user_pk);
        let member = SpaceDiscussionMember::get(&cli, pk, Some(sk)).await?;
        discussion.is_member = member.is_some();

        Ok(discussion)
    }

    pub async fn delete_all(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<()> {
        let mut bookmark = None::<String>;
        loop {
            let mut options = SpaceDiscussionQueryOption::builder()
                .sk("SPACE_DISCUSSION#".into())
                .limit(100);

            if let Some(b) = &bookmark {
                options = options.bookmark(b.clone());
            }

            let (discussions, next_bookmark) = SpaceDiscussion::query(cli, space_pk, options)
                .await
                .map_err(|e| {
                    tracing::debug!("Error querying space discussions for deletion: {:?}", e);
                    e
                })?;

            if discussions.is_empty() {
                break;
            }

            let tx_items = discussions
                .into_iter()
                .map(|discussion| {
                    SpaceDiscussion::delete_transact_write_item(discussion.pk, discussion.sk)
                })
                .collect::<Vec<_>>();

            cli.transact_write_items()
                .set_transact_items(Some(tx_items))
                .send()
                .await
                .map_err(|e| Error::InternalServerError(e.to_string()))?;

            match next_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        Ok(())
    }
}
