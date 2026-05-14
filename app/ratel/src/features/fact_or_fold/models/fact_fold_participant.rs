use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Per-user round state. Distinct from the `Round.participant_pks`
/// list (which is just the headcount slot) — this row carries
/// privileged info (`is_insider`) and the heartbeat used by the
/// reconnect-grace policy.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldParticipant {
    pub pk: Partition,  // Partition::FactFold(round_id)
    pub sk: EntityType, // EntityType::FactFoldParticipant(user_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub user_pk: Partition,
    pub joined_at: i64,
    /// True only on the insider's row. Stored under the round pk so
    /// the matching service can write it transactionally with the
    /// stage transition; never echoed to non-insider players.
    pub is_insider: bool,
    /// Latest heartbeat — refreshed on each `POST /heartbeat`. The
    /// stage-advance handler uses `now - last_seen_at >
    /// reconnect_grace_sec` to decide forfeit.
    pub last_seen_at: i64,
    pub forfeited: bool,
}

#[cfg(feature = "server")]
impl FactFoldParticipant {
    pub fn keys(round_id: &str, user_id: &str) -> (Partition, EntityType) {
        (
            Partition::FactFold(round_id.to_string()),
            EntityType::FactFoldParticipant(user_id.to_string()),
        )
    }

    pub fn new(round_id: &str, user_pk: Partition, is_insider: bool) -> Self {
        let user_id = user_pk
            .to_string()
            .strip_prefix("USER#")
            .unwrap_or(&user_pk.to_string())
            .to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let (pk, sk) = Self::keys(round_id, &user_id);
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk,
            joined_at: now,
            is_insider,
            last_seen_at: now,
            forfeited: false,
        }
    }
}
