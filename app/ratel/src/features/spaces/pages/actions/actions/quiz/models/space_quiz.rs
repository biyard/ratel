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

    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,

    pub user_response_count: i64,

    pub started_at: i64,
    pub ended_at: i64,

    pub retry_count: i64,

    #[serde(default)]
    pub pass_score: i64,

    #[serde(default)]
    pub questions: Vec<Question>,
    #[serde(default)]
    pub files: Vec<File>,
}

impl From<SpaceQuiz> for crate::features::spaces::pages::actions::types::SpaceAction {
    fn from(quiz: SpaceQuiz) -> Self {
        use crate::features::spaces::pages::actions::types::SpaceActionType;
        let action_id = quiz.sk.to_string();
        Self {
            action_id,
            action_type: SpaceActionType::Quiz,
            title: quiz.title,
            description: quiz.description,
            created_at: quiz.created_at,
            updated_at: quiz.updated_at,
            total_score: None,
            total_point: None,
            started_at: Some(quiz.started_at),
            ended_at: Some(quiz.ended_at),
            user_participated: false,
        }
    }
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
            title: String::new(),
            description: String::new(),
            user_response_count: 0,
            started_at: now,
            ended_at: now + 7 * 24 * 60 * 60 * 1000,
            retry_count: 0,
            pass_score: 0,
            questions: vec![],
            files: vec![],
        })
    }

    pub fn status(&self) -> QuizStatus {
        let now = get_now_timestamp_millis();
        if now < self.started_at {
            QuizStatus::NotStarted
        } else if now >= self.started_at && now <= self.ended_at {
            QuizStatus::InProgress
        } else {
            QuizStatus::Finish
        }
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
        &self,
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::quiz::Result<()> {
        match user_role {
            SpaceUserRole::Creator | SpaceUserRole::Participant => {
                if self.status() == QuizStatus::InProgress {
                    return Ok(());
                }
                return Err(Error::BadRequest("Poll is not in progress".into()));
            }
            _ => Err(crate::features::spaces::pages::actions::actions::quiz::Error::NoPermission),
        }
    }
}
