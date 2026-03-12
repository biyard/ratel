use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::actions::follow::*;

use crate::features::spaces::pages::actions::actions::follow::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceFollowAction {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,
}

impl From<SpaceFollowAction> for crate::features::spaces::pages::actions::types::SpaceActionSummary {
    fn from(follow: SpaceFollowAction) -> Self {
        use crate::features::spaces::pages::actions::types::SpaceActionType;
        let action_id = follow.sk.to_string();
        Self {
            action_id,
            action_type: SpaceActionType::Follow,
            title: String::new(),
            description: String::new(),
            created_at: follow.created_at,
            updated_at: follow.updated_at,
            total_score: None,
            total_point: None,
            started_at: None,
            ended_at: None,
            user_participated: false,
        }
    }
}

#[cfg(feature = "server")]
impl SpaceFollowAction {
    pub fn new(space_pk: SpacePartition) -> Self {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceSubscription;
        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_edit(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::follow::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(
                crate::features::spaces::pages::actions::actions::follow::Error::NoPermission,
            ),
        }
    }
}
