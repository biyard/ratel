use crate::*;

/// Per-user aggregate counters for the Essence list. Maintained by atomic
/// DynamoDB `ADD` operations from `Essence::put` and the delete paths, so
/// the hero card can show accurate totals in one roundtrip instead of
/// paginating the entire index.
///
/// Singleton — there's exactly one row per user under
/// `pk = USER#{uid}`, `sk = UserEssenceStats`.
///
/// Per-kind fields (`total_notion`/`total_post`/`total_comment`/`total_poll`
/// /`total_quiz`) mirror the client `KindFilter` chips. `total_comment` is
/// the aggregate of `PostComment` + `DiscussionComment` so the client can
/// use a single counter per chip and the server-side kind-filter query
/// returns a consistent total.
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserEssenceStats {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub total_sources: i64,
    #[serde(default)]
    pub total_words: i64,

    #[serde(default)]
    pub total_notion: i64,
    #[serde(default)]
    pub total_post: i64,
    #[serde(default)]
    pub total_comment: i64,
    #[serde(default)]
    pub total_poll: i64,
    #[serde(default)]
    pub total_quiz: i64,
}

#[cfg(feature = "server")]
impl UserEssenceStats {
    /// Fetch the counter row, returning zeros when the user has never had
    /// any Essence rows (the first `put` will create it via atomic ADD).
    pub async fn get_or_default(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: Partition,
    ) -> Result<Self> {
        let sk = EntityType::UserEssenceStats;
        match Self::get(cli, user_pk.clone(), Some(sk.clone())).await {
            Ok(Some(row)) => Ok(row),
            Ok(None) => Ok(Self {
                pk: user_pk,
                sk,
                ..Default::default()
            }),
            Err(e) => {
                crate::error!("essence stats read failed: {e}");
                Err(crate::features::essence::types::EssenceError::ReadFailed.into())
            }
        }
    }
}
