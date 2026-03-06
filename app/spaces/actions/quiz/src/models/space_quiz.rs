use common::utils::time::get_now_timestamp_millis;

use crate::macros::DynamoEntity;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceQuiz {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,

    pub user_response_count: i64,

    pub started_at: i64,
    pub ended_at: i64,

    pub retry_count: i64,

    #[serde(default)]
    pub questions: Vec<Question>,
}

#[cfg(feature = "server")]
impl SpaceQuiz {
    pub fn new(space_pk: SpacePartition) -> crate::Result<Self> {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceQuiz(uuid::Uuid::now_v7().to_string());
        let now = get_now_timestamp_millis();

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            title: String::new(),
            description: String::new(),
            user_response_count: 0,
            started_at: now,
            ended_at: now + 7 * 24 * 60 * 60 * 1000,
            retry_count: 0,
            questions: vec![],
        })
    }

    pub fn can_edit(user_role: &SpaceUserRole) -> crate::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(crate::Error::NoPermission),
        }
    }
}
