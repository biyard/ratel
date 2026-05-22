use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Per-user marker row asserting "this user has already joined a
/// round backed by `subject_id`". `pick_next_subject` short-circuits
/// when it finds this row, so the same user never gets bound to
/// rounds for the same active subject window twice.
///
/// Layout: `Partition::User(user_id) + EntityType::FactFoldSubjectPlay(subject_id)`.
/// Written transactionally with the lobby join. Reads are O(1) `get`
/// — the existence of the row is the whole signal.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldSubjectPlay {
    pub pk: Partition,  // Partition::User(user_id)
    pub sk: EntityType, // EntityType::FactFoldSubjectPlay(subject_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// Round the user was bound into for this subject. Useful for
    /// post-mortems / showing the user a link back to their result.
    pub round_id: String,
}

#[cfg(feature = "server")]
impl FactFoldSubjectPlay {
    pub fn keys(user_id: &str, subject_id: &str) -> (Partition, EntityType) {
        (
            Partition::User(user_id.to_string()),
            EntityType::FactFoldSubjectPlay(subject_id.to_string()),
        )
    }

    pub fn new(user_id: &str, subject_id: &str, round_id: &str) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let (pk, sk) = Self::keys(user_id, subject_id);
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            round_id: round_id.to_string(),
        }
    }

    /// True iff a marker exists — caller treats `Some(_)` as "already
    /// played", `None` as "free to join".
    pub async fn exists(
        cli: &aws_sdk_dynamodb::Client,
        user_id: &str,
        subject_id: &str,
    ) -> crate::common::Result<bool> {
        let (pk, sk) = Self::keys(user_id, subject_id);
        let row = FactFoldSubjectPlay::get(cli, &pk, Some(sk)).await?;
        Ok(row.is_some())
    }
}
