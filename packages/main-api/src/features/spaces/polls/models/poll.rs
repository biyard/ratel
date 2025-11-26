use crate::{
    Error,
    config::{self},
    models::{SpaceCommon, email_template::email_template::EmailTemplate},
    types::{email_operation::EmailOperation, *},
    utils::{
        aws::{DynamoClient, SesClient, get_aws_config},
        html::start_survey_html,
        time::get_now_timestamp_millis,
        uuid::sorted_uuid,
    },
};
use aws_config::Region;
use aws_sdk_s3::{
    Client, Config,
    config::Credentials,
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
};
use aws_sdk_scheduler::Client as SchedulerClient;
use aws_sdk_scheduler::types::{
    ActionAfterCompletion, EventBridgeParameters, FlexibleTimeWindow, FlexibleTimeWindowMode,
    ScheduleState, Target,
};
use chrono::Utc;
// use aws_sdk_sts::Client as StsClient;
use bdk::prelude::*;
use by_axum::axum::Json;
use chrono::DateTime;

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
    ) -> Result<Json<()>, Error> {
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

        email.send_email(&dynamo, &ses, false).await?;

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

    pub async fn _schedule_start_notification(&self) -> crate::Result<()> {
        let cfg = config::get();
        let sdk_config = get_aws_config();
        let client = SchedulerClient::new(&sdk_config);

        let region = sdk_config
            .region()
            .map(|r| r.as_ref().to_string())
            .unwrap_or_else(|| "ap-northeast-2".to_string());

        let account_id = std::env::var("AWS_ACCOUNT_ID")
            .map_err(|_| crate::Error::Unknown("AWS_ACCOUNT_ID is required".into()))?;

        let pk_str = self.pk.to_string();
        let sk_str = self.sk.to_string();
        let schedule_name =
            Self::sanitize_schedule_name(&format!("poll-start-{}-{}", pk_str, sk_str));

        let now = get_now_timestamp_millis();
        if self.started_at <= now {
            return Ok(());
        }

        let start_at: DateTime<Utc> = DateTime::<Utc>::from_timestamp_millis(self.started_at)
            .ok_or_else(|| crate::Error::Unknown("invalid started_at".into()))?;

        let start_at_str = start_at.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let schedule_expr = format!("at({})", start_at_str);

        let space_id = match &self.pk {
            Partition::Space(id) => id.clone(),
            _ => {
                return Err(crate::Error::InvalidPartitionKey(
                    "Poll must be under Space partition".into(),
                ));
            }
        };
        let survey_id = match &self.sk {
            EntityType::SpacePoll(id) => id.clone(),
            _ => {
                return Err(crate::Error::Unknown(
                    "Poll sk must be EntityType::SpacePoll".into(),
                ));
            }
        };

        let bus_name = format!("ratel-{}-bus", cfg.env);
        let bus_arn = format!("arn:aws:events:{region}:{account_id}:event-bus/{bus_name}");

        let scheduler_role_name = format!("ratel-{}-{}-survey-scheduler-role", cfg.env, region);
        let scheduler_role_arn = format!("arn:aws:iam::{account_id}:role/{scheduler_role_name}");

        let input_json = serde_json::json!({
            "space_id": space_id,
            "survey_id": survey_id,
            "started_at": self.started_at,
        })
        .to_string();

        let ftw = FlexibleTimeWindow::builder()
            .mode(FlexibleTimeWindowMode::Off)
            .build()
            .map_err(|e| crate::Error::Unknown(e.to_string()))?;

        let eb_params = EventBridgeParameters::builder()
            .source("ratel.spaces")
            .detail_type("SurveyFetcher")
            .build()
            .map_err(|e| crate::Error::Unknown(e.to_string()))?;

        let target = Target::builder()
            .arn(bus_arn)
            .role_arn(scheduler_role_arn)
            .event_bridge_parameters(eb_params)
            .input(input_json)
            .build()
            .map_err(|e| crate::Error::Unknown(e.to_string()))?;

        let update_result = client
            .update_schedule()
            .name(schedule_name.clone())
            .group_name("default")
            .schedule_expression(schedule_expr.clone())
            .flexible_time_window(ftw.clone())
            .state(ScheduleState::Enabled)
            .action_after_completion(ActionAfterCompletion::Delete)
            .target(target.clone())
            .send()
            .await;

        if update_result.is_ok() {
            return Ok(());
        }

        client
            .create_schedule()
            .name(schedule_name)
            .group_name("default")
            .schedule_expression(schedule_expr)
            .flexible_time_window(ftw)
            .state(ScheduleState::Enabled)
            .action_after_completion(ActionAfterCompletion::Delete)
            .target(target)
            .send()
            .await
            .map_err(|e| crate::Error::Unknown(e.to_string()))?;

        Ok(())
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

    fn try_from(value: Partition) -> Result<Self, Self::Error> {
        let uuid = match value {
            Partition::Space(ref s) => s.clone(),
            _ => return Err(crate::Error::Unknown("server error".to_string())),
        };

        Poll::new(value, Some(EntityType::SpacePoll(uuid)))
    }
}
