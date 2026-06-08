use crate::features::spaces::pages::actions::actions::discussion::*;

use crate::features::spaces::pages::actions::actions::discussion::macros::DynamoEntity;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// One row per (discussion, subscriber). Stored under the discussion's
/// `SpacePost` partition (the same partition its comments use), so listing all
/// subscribers of a discussion is a single partition Query — never a Scan.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SpacePostSubscription {
    pub pk: Partition,  // Partition::SpacePost(post_id) — the discussion partition
    pub sk: EntityType, // EntityType::SpacePostSubscription(user_pk string)

    pub space_pk: Partition,
    pub user_pk: Partition,
    pub created_at: i64,
}

impl SpacePostSubscription {
    /// Derive (pk, sk) for a (discussion, user) pair. `user_pk` is the full
    /// `Partition::User(..)`; its string form is the sk suffix so each user maps
    /// to exactly one subscription row per discussion.
    pub fn keys(
        space_post_pk: &SpacePostPartition,
        user_pk: &Partition,
    ) -> (Partition, EntityType) {
        let pk: Partition = space_post_pk.clone().into();
        let sk = EntityType::SpacePostSubscription(user_pk.to_string());
        (pk, sk)
    }

    pub fn new(
        space_post_pk: SpacePostPartition,
        space_pk: SpacePartition,
        user_pk: &Partition,
    ) -> Self {
        let (pk, sk) = Self::keys(&space_post_pk, user_pk);
        Self {
            pk,
            sk,
            space_pk: space_pk.into(),
            user_pk: user_pk.clone(),
            created_at: crate::common::utils::time::get_now_timestamp(),
        }
    }

    /// Sort-key prefix for partition queries that list all subscribers of a
    /// discussion.
    pub fn sk_prefix() -> String {
        "SPACE_POST_SUBSCRIPTION".to_string()
    }
}
