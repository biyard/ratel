use crate::*;
use types::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct TopicArticle {
    pub pk: Partition,
    #[dynamo(prefix = "TS", name = "find_by_created_at", index = "gsi1", pk)]
    pub sk: EntityType,

    pub title: String,
    pub content: String,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub created_by: Partition,
    pub view_count: i64,
}

impl TopicArticle {
    pub fn new(
        space_pk: Partition,
        topic_name: String,
        title: String,
        content: String,
        created_by: Partition,
    ) -> Self {
        match space_pk {
            Partition::Space(_) => {}
            _ => panic!("Partition must be of type Space"),
        }

        let article_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp_micros();
        let combined_id = format!("{}#{}", topic_name, article_id);

        Self {
            pk: space_pk,
            sk: EntityType::TopicArticle(combined_id),
            title,
            content,
            created_at: now,
            updated_at: now,
            created_by,
            view_count: 0,
        }
    }

    pub fn keys(space_pk: &Partition, topic_name: &str, article_id: &str) -> (Partition, EntityType) {
        let combined_id = format!("{}#{}", topic_name, article_id);
        (
            space_pk.clone(),
            EntityType::TopicArticle(combined_id),
        )
    }

    pub async fn get_article(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
        article_id: &str,
    ) -> crate::Result<Option<Self>> {
        let (pk, sk) = Self::keys(space_pk, topic_name, article_id);
        Self::get(cli, pk, Some(sk)).await
    }

    pub async fn list_by_topic(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
        limit: Option<i32>,
        bookmark: Option<String>,
    ) -> crate::Result<(Vec<Self>, Option<String>)> {
        let sk_prefix = format!("TOPIC_ARTICLE#{}", topic_name);

        let mut options = TopicArticleQueryOption::builder()
            .sk(sk_prefix)
            .limit(limit.unwrap_or(20));

        if let Some(b) = bookmark {
            options = options.bookmark(b);
        }

        Self::query(cli, space_pk, options).await
    }

    pub async fn delete_all_by_topic(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        topic_name: &str,
    ) -> crate::Result<()> {
        let mut bookmark = None::<String>;
        loop {
            let (articles, next_bookmark) = Self::list_by_topic(cli, space_pk, topic_name, Some(100), bookmark).await?;

            if articles.is_empty() {
                break;
            }

            let tx_items = articles
                .into_iter()
                .map(|article| Self::delete_transact_write_item(article.pk, article.sk))
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
