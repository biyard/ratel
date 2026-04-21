use crate::*;

/// Per-user aggregate counters for the Essence list. Maintained by atomic
/// DynamoDB `ADD` operations from `Essence::put` and the delete paths, so
/// the hero card can show accurate totals in one roundtrip instead of
/// paginating the entire index.
///
/// Singleton — there's exactly one row per user under
/// `pk = USER#{uid}`, `sk = UserEssenceStats`.
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserEssenceStats {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub total_sources: i64,
    #[serde(default)]
    pub total_words: i64,
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
                total_sources: 0,
                total_words: 0,
            }),
            Err(e) => {
                crate::error!("essence stats read failed: {e}");
                Err(crate::features::essence::types::EssenceError::ReadFailed.into())
            }
        }
    }
}
