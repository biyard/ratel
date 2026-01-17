use crate::features::spaces::polls::poll::Poll;
use crate::time::get_now_timestamp_millis;
use crate::types::{EntityType, Partition};
use crate::{Error, Result, config};
use aws_config::SdkConfig;
use aws_sdk_scheduler::{
    Client as SchedulerClient,
    types::{
        ActionAfterCompletion, EventBridgeParameters, FlexibleTimeWindow, FlexibleTimeWindowMode,
        ScheduleState, Target,
    },
};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct PollScheduler {
    client: SchedulerClient,
    region: String,
    account_id: String,
    env: String,
    group_name: String,
}

impl PollScheduler {
    pub fn new(sdk_config: &SdkConfig) -> Self {
        let client = SchedulerClient::new(sdk_config);

        let region = sdk_config
            .region()
            .map(|r| r.as_ref().to_string())
            .unwrap_or_else(|| "ap-northeast-2".to_string());

        let cfg = config::get();
        let account_id = cfg.account_id.to_string();
        let env = cfg.env.to_string();

        Self {
            client,
            region,
            account_id,
            env,
            group_name: "default".to_string(),
        }
    }

    fn sanitize_schedule_name(raw: &str) -> String {
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

    fn bus_arn(&self) -> String {
        let bus_name = format!("ratel-{}-bus", self.env);
        format!(
            "arn:aws:events:{}:{}:event-bus/{}",
            self.region, self.account_id, bus_name
        )
    }

    fn scheduler_role_arn(&self) -> String {
        let role_name = format!("ratel-{}-{}-survey-scheduler-role", self.env, self.region);
        format!("arn:aws:iam::{}:role/{}", self.account_id, role_name)
    }

    fn should_skip(&self, at_millis: i64, name: &str) -> Result<bool> {
        if self.env == "local" {
            return Ok(true);
        }

        if self.account_id.is_empty() {
            tracing::error!("account id is not found");
            return Ok(true);
        }

        let now = get_now_timestamp_millis();
        if at_millis <= now {
            tracing::error!("{} skipped: at_millis({}) <= now({})", name, at_millis, now,);
            return Ok(true);
        }

        Ok(false)
    }

    fn to_at_expression(&self, at_millis: i64) -> Result<String> {
        let at: DateTime<Utc> = DateTime::<Utc>::from_timestamp_millis(at_millis)
            .ok_or_else(|| Error::InternalServerError("invalid at_millis".into()))?;
        let at_str = at.format("%Y-%m-%dT%H:%M:%S").to_string();
        Ok(format!("at({})", at_str))
    }

    fn poll_ids(poll: &Poll) -> Result<(String, String)> {
        let space_id = match &poll.pk {
            Partition::Space(id) => id.clone(),
            _ => {
                return Err(Error::InvalidPartitionKey(
                    "Poll must be under Space partition".into(),
                ));
            }
        };

        let survey_id = match &poll.sk {
            EntityType::SpacePoll(id) => id.clone(),
            _ => {
                return Err(Error::InternalServerError(
                    "Poll sk must be EntityType::SpacePoll".into(),
                ));
            }
        };

        Ok((space_id, survey_id))
    }

    fn make_ftw() -> Result<FlexibleTimeWindow> {
        FlexibleTimeWindow::builder()
            .mode(FlexibleTimeWindowMode::Off)
            .build()
            .map_err(|e| Error::InternalServerError(e.to_string()))
    }

    fn make_eb_params(source: &str, detail_type: &str) -> Result<EventBridgeParameters> {
        EventBridgeParameters::builder()
            .source(source)
            .detail_type(detail_type)
            .build()
            .map_err(|e| Error::InternalServerError(e.to_string()))
    }

    fn make_target(&self, eb_params: EventBridgeParameters, input_json: String) -> Result<Target> {
        Target::builder()
            .arn(self.bus_arn())
            .role_arn(self.scheduler_role_arn())
            .event_bridge_parameters(eb_params)
            .input(input_json)
            .build()
            .map_err(|e| Error::InternalServerError(e.to_string()))
    }

    async fn upsert_schedule(
        &self,
        schedule_name: String,
        schedule_expr: String,
        target: Target,
    ) -> Result<()> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;

        let ftw = Self::make_ftw()?;

        let update_result = self
            .client
            .update_schedule()
            .name(schedule_name.clone())
            .group_name(&self.group_name)
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

        self.client
            .create_schedule()
            .name(schedule_name)
            .group_name(&self.group_name)
            .schedule_expression(schedule_expr)
            .flexible_time_window(ftw)
            .state(ScheduleState::Enabled)
            .action_after_completion(ActionAfterCompletion::Delete)
            .target(target)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("create_schedule failed: {e:?}");

                let code = e.code().unwrap_or("Unknown");
                let msg = e.message().unwrap_or("No message");
                tracing::error!("aws error code={code}, message={msg}");

                Error::InternalServerError(e.to_string())
            })?;

        Ok(())
    }

    async fn schedule_eventbridge(
        &self,
        schedule_name_key: &str,
        at_millis: i64,
        source: &str,
        detail_type: &str,
        detail_json: serde_json::Value,
    ) -> Result<()> {
        let skip = self.should_skip(at_millis, schedule_name_key)?;
        if skip {
            return Ok(());
        }

        let schedule_name =
            Self::sanitize_schedule_name(&format!("{}-{}", detail_type, schedule_name_key));
        let schedule_expr = self.to_at_expression(at_millis)?;

        let eb_params = Self::make_eb_params(source, detail_type)?;
        let target = self.make_target(eb_params, detail_json.to_string())?;

        self.upsert_schedule(schedule_name, schedule_expr, target)
            .await
    }

    pub async fn schedule_start_notification(&self, poll: &Poll, started_at: i64) -> Result<()> {
        let (_space_id, survey_id) = Self::poll_ids(poll)?;

        let detail = serde_json::json!({
            "space_id": match &poll.pk {
                Partition::Space(id) => id.clone(),
                _ => return Err(Error::InvalidPartitionKey("Poll must be under Space partition".into())),
            },
            "survey_id": survey_id.clone(),
        });

        self.schedule_eventbridge(
            &format!("poll-start-{}", survey_id),
            started_at,
            "ratel.spaces",
            "SurveyFetcher",
            detail,
        )
        .await
    }

    pub async fn schedule_upsert_analyze(
        &self,
        space_pk: Partition,
        lda_topics: usize,
        tf_idf_keywords: usize,
        network_top_nodes: usize,

        remove_topics: Vec<String>,
    ) -> Result<()> {
        let space_id = match &space_pk {
            Partition::Space(id) => id.clone(),
            _ => return Err(Error::InvalidPartitionKey("Not Space Partition".into())),
        };

        let at_millis = get_now_timestamp_millis() + 10_000;

        let detail = serde_json::json!({
            "space_id": space_id,
            "lda_topics": lda_topics,
            "tf_idf_keywords": tf_idf_keywords,
            "network_top_nodes": network_top_nodes,
            "remove_keywords": remove_topics,
        });

        self.schedule_eventbridge(
            &detail.clone()["space_id"].as_str().unwrap_or("space"),
            at_millis,
            "ratel.spaces",
            "SurveyFetcher",
            detail,
        )
        .await
    }

    pub async fn schedule_download_analyze(&self, space_pk: Partition) -> Result<()> {
        let space_id = match &space_pk {
            Partition::Space(id) => id.clone(),
            _ => return Err(Error::InvalidPartitionKey("Not Space Partition".into())),
        };

        let at_millis = get_now_timestamp_millis() + 10_000;

        let detail = serde_json::json!({
            "space_id": space_id,
        });

        self.schedule_eventbridge(
            &detail.clone()["space_id"].as_str().unwrap_or("space"),
            at_millis,
            "ratel.spaces",
            "SurveyFetcher",
            detail,
        )
        .await
    }
}

#[cfg(test)]
impl PollScheduler {
    pub fn mock(sdk_config: &SdkConfig) -> Self {
        Self {
            client: SchedulerClient::new(sdk_config),
            region: "ap-northeast-2".to_string(),
            account_id: "000000000000".to_string(),
            env: "test".to_string(),
            group_name: "default".to_string(),
        }
    }
}
