use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(untagged)]
pub enum UpdatePollRequest {
    Title { title: String },
    Time { started_at: i64, ended_at: i64 },
    Question { questions: Vec<Question> },
    ResponseEditable { response_editable: bool },
    CanisterUploadEnabled { canister_upload_enabled: bool },
}

#[mcp_tool(name = "update_poll", description = "Update a poll (title, time range, questions, response_editable). Requires creator role.")]
#[post("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn update_poll(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Poll sort key (e.g. 'SpacePoll#<uuid>')")]
    poll_sk: SpacePollEntityType,
    #[mcp(description = "Poll update data as JSON. Supported variants: {\"title\": \"...\"}, {\"started_at\": <ms>, \"ended_at\": <ms>}, {\"questions\": [...]}, {\"response_editable\": true}")]
    req: UpdatePollRequest,
) -> Result<String> {
    SpacePoll::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.clone().into();

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Lock all poll edits once the action has started. The creator UI
    // already disables the inputs, but defend the API surface here too.
    let action_pk =
        CompositePartition::<SpacePartition, String>(space_pk.clone().into(), poll_sk.to_string());
    let space_action = SpaceAction::get(cli, &action_pk, Some(EntityType::SpaceAction))
        .await
        .map_err(|e| Error::InternalServerError(format!("Failed to get space action: {e:?}")))?
        .ok_or(Error::NotFound("Space action not found".into()))?;
    if crate::features::spaces::pages::actions::is_action_locked(
        space.status.clone(),
        space_action.started_at,
    ) {
        return Err(Error::BadRequest(
            "Poll cannot be edited after the action has started".into(),
        ));
    }

    let mut poll_updater = SpacePoll::updater(&space_pk, &poll_sk_entity).with_updated_at(now);
    let mut action_updater =
        SpaceAction::updater(&action_pk, &EntityType::SpaceAction).with_updated_at(now);
    let mut update_action = false;

    match req {
        UpdatePollRequest::Title { title } => {
            poll_updater = poll_updater.with_title(title.clone());
            action_updater = action_updater.with_title(title);
            update_action = true;
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
                .with_description(description.clone());
            action_updater = action_updater.with_description(description);
            update_action = true;
        }
        UpdatePollRequest::ResponseEditable { response_editable } => {
            poll_updater = poll_updater.with_response_editable(response_editable);
        }
        UpdatePollRequest::CanisterUploadEnabled {
            canister_upload_enabled,
        } => {
            let env = crate::common::config::Environment::default();
            if env == crate::common::config::Environment::Production {
                return Err(Error::BadRequest(
                    "Canister upload is not available in production".into(),
                ));
            }
            poll_updater = poll_updater.with_canister_upload_enabled(canister_upload_enabled);
            // When enabling encrypted upload, force response_editable to false
            // because encrypted votes cannot be edited after submission.
            if canister_upload_enabled {
                poll_updater = poll_updater.with_response_editable(false);
            }
        }
    }

    poll_updater.execute(cli).await?;
    if update_action {
        action_updater.execute(cli).await?;
    }

    Ok("success".to_string())
}
