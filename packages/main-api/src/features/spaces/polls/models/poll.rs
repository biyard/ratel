use crate::email_operation::EmailOperation;
use crate::models::SpaceCommon;
use crate::models::email_template::email_template::EmailTemplate;
use crate::time::get_now_timestamp_millis;
use crate::utils::aws::PollScheduler;
use crate::utils::aws::get_aws_config;
use crate::utils::aws::{DynamoClient, SesClient};
use crate::*;
use aws_config::Region;
use aws_sdk_s3::{
    Client, Config,
    config::Credentials,
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
};
use aws_sdk_scheduler::types::{
    ActionAfterCompletion, EventBridgeParameters, FlexibleTimeWindow, FlexibleTimeWindowMode,
    ScheduleState, Target,
};
use aws_sdk_scheduler::{Client as SchedulerClient, error::SdkError};
use bdk::prelude::*;
use by_axum::axum::Json;
use chrono::DateTime;
use chrono::Utc;

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
struct StartSurveyEventInput {
    pub space_id: String,
    pub survey_id: String,
}

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
    pub async fn send_email(
        dynamo: &DynamoClient,
        ses: &SesClient,
        space: SpaceCommon,
        title: String,
        user_emails: Vec<String>,
        is_default: bool,
    ) -> Result<Json<()>> {
        let mut domain = crate::config::get().domain.to_string();
        if domain.contains("localhost") {
            domain = format!("http://{}", domain).to_string();
        } else {
            domain = format!("https://{}", domain).to_string();
        }

        let space_id = match space.pk.clone() {
            Partition::Space(v) => v.to_string(),
            _ => "".to_string(),
        };

        let url = format!("{}/spaces/SPACE%23{}", domain, space_id);
        let survey_title = if is_default {
            "Pre Poll Survey"
        } else {
            "Final Survey"
        };

        let email = EmailTemplate {
            targets: user_emails.clone(),
            operation: EmailOperation::StartSurvey {
                space_title: title,
                survey_title: survey_title.to_string(),
                author_profile: space.author_profile_url,
                author_display_name: space.author_display_name,
                author_username: space.author_username,
                connect_link: url,
            },
        };

        email.send_email(&dynamo, &ses).await?;
        Ok(Json(()))
    }

    pub fn new(pk: Partition, sk: Option<EntityType>) -> crate::Result<Self> {
        if !matches!(pk, Partition::Space(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "PollSpace must be under Space partition".to_string(),
            ));
        }

        let sk = sk.unwrap_or_else(|| {
            let uuid = sorted_uuid();
            EntityType::SpacePoll(uuid)
        });

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

    pub async fn schedule_start_notification(
        &self,
        scheduler: &PollScheduler,
        started_at: i64,
    ) -> crate::Result<()> {
        scheduler
            .schedule_start_notification(self, started_at)
            .await
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

    fn try_from(value: Partition) -> Result<Self> {
        let uuid = match value {
            Partition::Space(ref s) => s.clone(),
            _ => {
                return Err(crate::Error::InternalServerError(
                    "server error".to_string(),
                ));
            }
        };

        Poll::new(value, Some(EntityType::SpacePoll(uuid)))
    }
}
