use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

use crate::features::spaces::polls::PollStatus;
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct Poll {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub topic: String,       // Poll Title
    pub description: String, // Poll Description

    pub user_response_count: i64, // Participants count

    pub started_at: i64,
    pub ended_at: i64,
    pub response_editable: bool, // Whether users can edit their responses

    #[serde(default)]
    pub questions: Vec<Question>, // Questions in the survey
}

impl Poll {
    pub fn new(pk: Partition, sk: Option<EntityType>) -> crate::Result<Self> {
        if !matches!(pk, Partition::Space(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "PollSpace must be under Space partition".to_string(),
            ));
        }

        let sk = match sk {
            Some(EntityType::SpacePoll(s)) if !s.is_empty() => EntityType::SpacePoll(s),
            _ => {
                let uuid = uuid::Uuid::new_v4().to_string();
                EntityType::SpacePoll(uuid)
            }
        };

        let now = get_now_timestamp_millis();

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_response_count: 0,

            response_editable: false,
            started_at: now,
            ended_at: now + 7 * 24 * 60 * 60 * 1000, // Default to 7 days later

            topic: String::new(),
            description: String::new(),
            questions: Vec::new(),
        })
    }

    pub async fn delete_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<()> {
        let space_id = match space_pk {
            Partition::Space(v) => v.to_string(),
            _ => "".to_string(),
        };

        let poll = Poll::get(
            &cli,
            space_pk.clone(),
            Some(EntityType::SpacePoll(space_id.clone())),
        )
        .await?;

        if poll.is_none() {
            return Ok(());
        }

        Poll::delete(
            &cli,
            &space_pk.clone(),
            Some(EntityType::SpacePoll(space_id.clone())),
        )
        .await?;

        Ok(())
    }

    pub fn status(&self) -> PollStatus {
        let now = get_now_timestamp_millis();
        if now < self.started_at {
            PollStatus::NotStarted
        } else if now >= self.started_at && now <= self.ended_at {
            PollStatus::InProgress
        } else {
            PollStatus::Finish
        }
    }
}

impl TryFrom<Partition> for Poll {
    type Error = crate::Error;

    fn try_from(value: Partition) -> Result<Self, Self::Error> {
        let uuid = match value {
            Partition::Space(ref s) => s.clone(),
            _ => return Err(crate::Error::Unknown("server error".to_string())),
        };

        Poll::new(value, Some(EntityType::SpacePoll(uuid)))
    }
}
