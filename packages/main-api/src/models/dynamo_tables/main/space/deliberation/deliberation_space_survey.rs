use crate::types::*;

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceSurvey {
    pub pk: Partition,
    pub sk: EntityType,

    pub status: SurveyStatus,
    pub started_at: i64,
    pub ended_at: i64,
}

impl DeliberationSpaceSurvey {
    pub fn new(pk: Partition, status: SurveyStatus, started_at: i64, ended_at: i64) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::DeliberationSpaceSurvey(uid);

        Self {
            pk,
            sk,
            status,
            started_at,
            ended_at,
        }
    }
}
