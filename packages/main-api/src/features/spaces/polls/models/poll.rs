use crate::{
    config::{self, Config},
    types::*,
    utils::time::get_now_timestamp_millis,
};
// use aws_sdk_sts::Client as StsClient;
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

        let sk = sk.unwrap_or_else(|| {
            let uuid = uuid::Uuid::new_v4().to_string();
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

    // FIXME: fix to lambda start handler
    // pub async fn resolve_self_lambda_arn() -> crate::Result<String> {
    //     let conf = config::get();
    //     let sdk_config = aws_config::load_from_env().await;

    //     let sts = StsClient::new(&sdk_config);
    //     let account_id = sts
    //         .get_caller_identity()
    //         .send()
    //         .await
    //         .map_err(|e| crate::Error::Unknown(format!("sts error: {e:?}")))?
    //         .account
    //         .ok_or_else(|| crate::Error::Unknown("no account in sts response".into()))?;

    //     let region_from_env = std::env::var("AWS_REGION").ok();
    //     let region =
    //         region_from_env.ok_or_else(|| crate::Error::Unknown("AWS_REGION not set".into()))?;

    //     let function_name = std::env::var("AWS_LAMBDA_FUNCTION_NAME")
    //         .map_err(|_| crate::Error::Unknown("AWS_LAMBDA_FUNCTION_NAME not set".into()))?;

    //     Ok(format!(
    //         "arn:aws:lambda:{region}:{account_id}:function:{function_name}"
    //     ))
    // }

    // async fn create_poll_start_schedule(poll: &Poll) -> crate::Result<()> {
    //     let aws_config = Config::from(&config);
    //     let client = SchedulerClient::new(&aws_config);

    //     let lambda_arn =
    //         std::env::var("POLL_EVENT_LAMBDA_ARN").expect("POLL_EVENT_LAMBDA_ARN is required");
    //     let role_arn =
    //         std::env::var("POLL_EVENT_ROLE_ARN").expect("POLL_EVENT_ROLE_ARN is required");

    //     let schedule_name = format!("poll-start-{}-{}", poll.pk, poll.sk);

    //     // let start_at: DateTime<Utc> = poll.start_at;
    //     // let start_at_str = start_at.to_rfc3339();

    //     // let schedule_expr = format!("at({})", start_at_str);

    //     // let input_json = serde_json::to_string(&PollStartEventInput {
    //     //     space_pk: poll.pk.to_string(),
    //     //     poll_sk: poll.sk.to_string(),
    //     //     event_type: "poll_start".to_string(),
    //     // })?;

    //     // let ftw = FlexibleTimeWindow::builder()
    //     //     .mode(FlexibleTimeWindowMode::Off)
    //     //     .build();

    //     // let target = Target::builder()
    //     //     .arn(lambda_arn)
    //     //     .role_arn(role_arn)
    //     //     .input(input_json)
    //     //     .build()?;

    //     // client
    //     //     .create_schedule()
    //     //     .name(schedule_name)
    //     //     .group_name("default")
    //     //     .schedule_expression(schedule_expr)
    //     //     .flexible_time_window(ftw)
    //     //     .target(target)
    //     //     .send()
    //     //     .await?;

    //     Ok(())
    // }

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
