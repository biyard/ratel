use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::actions::quiz::macros::DynamoEntity;
use crate::features::spaces::pages::actions::actions::quiz::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceQuiz {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub user_response_count: i64,

    pub retry_count: i64,

    #[serde(default)]
    pub pass_score: i64,

    #[serde(default)]
    pub questions: Vec<Question>,
    #[serde(default)]
    pub files: Vec<File>,
}

#[cfg(feature = "server")]
impl SpaceQuiz {
    pub fn new(
        space_pk: SpacePartition,
    ) -> crate::features::spaces::pages::actions::actions::quiz::Result<Self> {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceQuiz(uuid::Uuid::now_v7().to_string());
        let now = get_now_timestamp_millis();

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_response_count: 0,
            retry_count: 0,
            pass_score: 0,
            questions: vec![],
            files: vec![],
        })
    }

    pub fn can_edit(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::quiz::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(crate::features::spaces::pages::actions::actions::quiz::Error::NoPermission),
        }
    }

    pub fn can_participate(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::quiz::Result<()> {
        match user_role {
            SpaceUserRole::Participant => Ok(()),
            _ => Err(crate::features::spaces::pages::actions::actions::quiz::Error::NoPermission),
        }
    }

    pub fn can_view(
        _user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::quiz::Result<()> {
        Ok(())
    }

    pub fn can_respond(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::quiz::Result<()> {
        match user_role {
            SpaceUserRole::Creator | SpaceUserRole::Participant => Ok(()),
            _ => Err(crate::features::spaces::pages::actions::actions::quiz::Error::NoPermission),
        }
    }
}
