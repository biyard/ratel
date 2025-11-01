use crate::*;
use types::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct TopicArticleReply {
    pub pk: Partition,
    #[dynamo(prefix = "TS", name = "find_by_created_at", index = "gsi1", pk)]
    pub sk: EntityType,

    pub content: String,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub created_by: Partition,

    // For threaded replies (optional parent reply)
    pub parent_reply_id: Option<String>,

    pub like_count: i64,
}

impl TopicArticleReply {
    pub fn new(
        space_pk: Partition,
        topic_name: String,
        article_id: String,
        content: String,
        created_by: Partition,
        parent_reply_id: Option<String>,
    ) -> Self {
        match space_pk {
            Partition::Space(_) => {}
            _ => panic!("Partition must be of type Space"),
        }

        let reply_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp_micros();
        let combined_article_id = format!("{}#{}", topic_name, article_id);

        Self {
            pk: space_pk,
            sk: EntityType::TopicArticleReply(combined_article_id, reply_id),
            content,
            created_at: now,
            updated_at: now,
            created_by,
            parent_reply_id,
            like_count: 0,
        }
    }

    pub fn keys(
        space_pk: &Partition,
        topic_name: &str,
        article_id: &str,
        reply_id: &str,
    ) -> (Partition, EntityType) {
        let combined_article_id = format!("{}#{}", topic_name, article_id);
        (
            space_pk.clone(),
            EntityType::TopicArticleReply(combined_article_id, reply_id.to_string()),
        )
    }

    pub async fn get_reply(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
        article_id: &str,
        reply_id: &str,
    ) -> crate::Result<Option<Self>> {
        let (pk, sk) = Self::keys(space_pk, topic_name, article_id, reply_id);
        Self::get(cli, pk, Some(sk)).await
    }

    pub async fn list_by_article(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
        article_id: &str,
        limit: Option<i32>,
        bookmark: Option<String>,
    ) -> crate::Result<(Vec<Self>, Option<String>)> {
        let sk_prefix = format!("TOPIC_ARTICLE_REPLY#{}#{}", topic_name, article_id);

        let mut options = TopicArticleReplyQueryOption::builder()
            .sk(sk_prefix)
            .limit(limit.unwrap_or(50));

        if let Some(b) = bookmark {
            options = options.bookmark(b);
        }

        Self::query(cli, space_pk, options).await
    }

    pub async fn delete_all_by_article(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
        article_id: &str,
    ) -> crate::Result<()> {
        let mut bookmark = None::<String>;
        loop {
            let (replies, next_bookmark) =
                Self::list_by_article(cli, space_pk, topic_name, article_id, Some(100), bookmark)
                    .await?;

            if replies.is_empty() {
                break;
            }

            let tx_items = replies
                .into_iter()
                .map(|reply| Self::delete_transact_write_item(reply.pk, reply.sk))
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

    pub async fn get_threaded_replies(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
        article_id: &str,
        parent_reply_id: &str,
    ) -> crate::Result<Vec<Self>> {
        let (all_replies, _) = Self::list_by_article(cli, space_pk, topic_name, article_id, Some(1000), None).await?;

        // Filter replies that have the specified parent
        let threaded_replies: Vec<Self> = all_replies
            .into_iter()
            .filter(|reply| {
                if let Some(parent_id) = &reply.parent_reply_id {
                    parent_id == parent_reply_id
                } else {
                    false
                }
            })
            .collect();

        Ok(threaded_replies)
    }
}
