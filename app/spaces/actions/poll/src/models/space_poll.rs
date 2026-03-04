use common::utils::time::get_now_timestamp_millis;

use crate::*;

use crate::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePoll {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub description: String,

    pub user_response_count: i64, // Participants count

    pub started_at: i64,
    pub ended_at: i64,

    pub response_editable: bool, // Whether users can edit their responses

    #[serde(default)]
    pub questions: Vec<Question>, // Questions in the survey

    #[serde(default)]
    pub total_score: i64,
    #[serde(default)]
    pub total_point: i64,
}

#[cfg(feature = "server")]
impl SpacePoll {
    pub fn new(space_pk: SpacePartition) -> crate::Result<Self> {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpacePoll(uuid::Uuid::now_v7().to_string());

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
            title: String::new(),
            description: String::new(),
            questions: Vec::new(),
            total_point: 0,
            total_score: 0,
        })
    }

    pub fn is_default_poll(&self) -> bool {
        match &self.sk {
            EntityType::SpacePoll(id) => {
                if let Partition::Space(space_id) = &self.pk {
                    return id == space_id;
                }
                false
            }
            _ => false,
        }
    }

    pub fn sanitize_schedule_name(raw: &str) -> String {
        let mut s: String = raw
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                    c
                } else {
                    '-'
                }
            })
            .collect();

        if s.len() > 64 {
            s.truncate(64);
        }
        s
    }

    pub async fn delete_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<()> {
        let space_id = match space_pk {
            Partition::Space(v) => v.to_string(),
            _ => return Ok(()),
        };

        let poll =
            SpacePoll::get(cli, space_pk, Some(EntityType::SpacePoll(space_id.clone()))).await?;

        if poll.is_none() {
            return Ok(());
        }

        SpacePoll::delete(cli, space_pk, Some(EntityType::SpacePoll(space_id.clone()))).await?;

        Ok(())
    }
}

impl SpacePoll {
    pub fn can_view(_user_role: &SpaceUserRole) -> crate::Result<()> {
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

    pub fn can_edit(user_role: &SpaceUserRole) -> crate::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(crate::Error::NoPermission),
        }
    }

    pub fn can_respond(&self, user_role: &SpaceUserRole) -> crate::Result<()> {
        match user_role {
            SpaceUserRole::Creator | SpaceUserRole::Participant => {
                if self.status() == PollStatus::InProgress {
                    return Ok(());
                }
                return Err(Error::BadRequest("Poll is not in progress".into()));
            }
            _ => Err(crate::Error::NoPermission),
        }
    }
}
#[cfg(feature = "server")]
impl TryFrom<Partition> for SpacePoll {
    type Error = crate::Error;

    fn try_from(value: Partition) -> crate::Result<Self> {
        let uuid = match value {
            Partition::Space(ref s) => s.clone(),
            _ => {
                return Err(crate::Error::InternalServerError(
                    "server error".to_string(),
                ));
            }
        };

        let pk = value;
        let sk = EntityType::SpacePoll(uuid);
        let now = get_now_timestamp_millis();

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_response_count: 0,
            response_editable: false,
            started_at: now,
            ended_at: now + 7 * 24 * 60 * 60 * 1000,
            title: String::new(),
            description: String::new(),
            questions: Vec::new(),
            total_point: 0,
            total_score: 0,
        })
    }
}
