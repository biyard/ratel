use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::actions::poll::*;

use crate::features::spaces::pages::actions::actions::poll::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePoll {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub title: String,
    #[serde(default)]
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

    #[serde(default)]
    pub canister_upload_enabled: bool,
}

#[cfg(feature = "server")]
impl SpacePoll {
    pub fn new(
        space_pk: SpacePartition,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<Self> {
        Self::new_with_published(space_pk, false)
    }

    pub fn new_with_published(
        space_pk: SpacePartition,
        is_published: bool,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<Self> {
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpacePoll(uuid::Uuid::now_v7().to_string());

        let now = get_now_timestamp_millis();
        let (started_at, ended_at) =
            crate::features::spaces::pages::actions::models::SpaceAction::default_schedule_with_published(now, is_published);

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_response_count: 0,
            response_editable: false,
            started_at,
            ended_at,
            title: String::new(),
            description: String::new(),
            questions: Vec::new(),
            total_point: 0,
            total_score: 0,
            canister_upload_enabled: false,
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
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<()> {
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

impl From<(SpacePoll, bool)>
    for crate::features::spaces::pages::actions::types::SpaceActionSummary
{
    fn from((poll, user_participated): (SpacePoll, bool)) -> Self {
        use crate::features::spaces::pages::actions::types::SpaceActionType;
        let action_id = poll.sk.to_string();
        Self {
            action_id,
            action_type: SpaceActionType::Poll,
            title: poll.title,
            description: poll.description,
            created_at: poll.created_at,
            updated_at: poll.updated_at,
            total_score: Some(poll.total_score),
            total_point: Some(poll.total_point),
            quiz_score: None,
            quiz_total_score: None,
            quiz_passed: None,
            started_at: Some(poll.started_at),
            ended_at: Some(poll.ended_at),
            user_participated,
            credits: 0,
            prerequisite: false,
            comment_count: None,
        }
    }
}

impl SpacePoll {
    pub fn can_view(
        _user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<()> {
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

    pub fn can_edit(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(crate::features::spaces::pages::actions::actions::poll::Error::NoPermission),
        }
    }

    pub fn can_respond(
        &self,
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<()> {
        match user_role {
            SpaceUserRole::Creator | SpaceUserRole::Participant => {
                if self.status() == PollStatus::InProgress {
                    return Ok(());
                }
                return Err(SpacePollError::PollNotInProgress.into());
            }
            _ => Err(crate::features::spaces::pages::actions::actions::poll::Error::NoPermission),
        }
    }
}
#[cfg(feature = "server")]
impl TryFrom<Partition> for SpacePoll {
    type Error = crate::features::spaces::pages::actions::actions::poll::Error;

    fn try_from(
        value: Partition,
    ) -> crate::features::spaces::pages::actions::actions::poll::Result<Self> {
        let uuid = match value {
            Partition::Space(ref s) => s.clone(),
            _ => {
                return Err(SpacePollError::CreateFailed.into());
            }
        };

        let pk = value;
        let sk = EntityType::SpacePoll(uuid);
        let now = get_now_timestamp_millis();
        let (started_at, ended_at) =
            crate::features::spaces::pages::actions::models::SpaceAction::default_schedule(now);

        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_response_count: 0,
            response_editable: false,
            started_at,
            ended_at,
            title: String::new(),
            description: String::new(),
            questions: Vec::new(),
            total_point: 0,
            total_score: 0,
            canister_upload_enabled: false,
        })
    }
}
