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

    #[serde(default)]
    pub user_pk: Partition,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub profile_url: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub user_type: UserType,
}

impl From<SpaceFollowAction>
    for crate::features::spaces::pages::actions::types::SpaceActionSummary
{
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
            quiz_score: None,
            quiz_total_score: None,
            quiz_passed: None,
            started_at: None,
            ended_at: None,
            user_participated: false,
            credits: 0,
            prerequisite: false,
        }
    }
}

#[cfg(feature = "server")]
impl SpaceFollowAction {
    pub fn new(space_pk: SpacePartition) -> Self {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceActionFollow(uuid::Uuid::now_v7().to_string());
        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk: Partition::default(),
            display_name: String::new(),
            profile_url: String::new(),
            username: String::new(),
            user_type: UserType::default(),
        }
    }

    pub fn new_user(
        space_pk: SpacePartition,
        user_pk: Partition,
        display_name: String,
        profile_url: String,
        username: String,
        user_type: UserType,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceActionFollow(user_pk.to_string());

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_pk,
            display_name,
            profile_url,
            username,
            user_type,
        }
    }

    pub fn user_keys(space_pk: &SpacePartition, user_pk: &Partition) -> (Partition, EntityType) {
        let pk: Partition = space_pk.clone().into();
        let sk = EntityType::SpaceActionFollow(user_pk.to_string());
        (pk, sk)
    }

    pub fn can_edit(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::follow::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(crate::features::spaces::pages::actions::actions::follow::Error::NoPermission),
        }
    }
}
