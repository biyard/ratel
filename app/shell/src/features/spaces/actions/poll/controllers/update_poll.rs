use crate::features::spaces::actions::poll::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePollRequest {
    Title { title: String },
    Time { started_at: i64, ended_at: i64 },
    Question { questions: Vec<Question> },
    ResponseEditable { response_editable: bool },
}

#[post("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole)]
pub async fn update_poll(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
    req: UpdatePollRequest,
) -> Result<String> {
    SpacePoll::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut poll_updater = SpacePoll::updater(&space_pk, &poll_sk_entity).with_updated_at(now);

    match req {
        UpdatePollRequest::Title { title } => {
            poll_updater = poll_updater.with_title(title);
        }
        UpdatePollRequest::Time {
            started_at,
            ended_at,
        } => {
            if started_at >= ended_at {
                return Err(Error::BadRequest("Invalid time range".into()));
            }
            poll_updater = poll_updater
                .with_started_at(started_at)
                .with_ended_at(ended_at);
        }
        UpdatePollRequest::Question { questions } => {
            if questions.is_empty() {
                return Err(Error::BadRequest("Questions cannot be empty".into()));
            }
            let description = questions
                .first()
                .map(|q| q.title().to_string())
                .unwrap_or_default();
            poll_updater = poll_updater
                .with_questions(questions)
                .with_description(description);
        }
        UpdatePollRequest::ResponseEditable { response_editable } => {
            poll_updater = poll_updater.with_response_editable(response_editable);
        }
    }

    poll_updater.execute(cli).await?;

    Ok("success".to_string())
}
