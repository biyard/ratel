use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(untagged)]
pub enum UpdatePollRequest {
    Title { title: String },
    Question { questions: Vec<Question> },
    ResponseEditable { response_editable: bool },
    CanisterUploadEnabled { canister_upload_enabled: bool },
}

#[mcp_tool(name = "update_poll", description = "Update a poll (title, questions, response_editable). Requires creator role.")]
#[post("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole)]
pub async fn update_poll(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Poll sort key (e.g. 'SpacePoll#<uuid>')")]
    poll_sk: SpacePollEntityType,
    #[mcp(description = "Poll update data as JSON. Supported variants: {\"title\": \"...\"}, {\"questions\": [...]}, {\"response_editable\": true}")]
    req: UpdatePollRequest,
) -> Result<String> {
    SpacePoll::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.clone().into();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut poll_updater = SpacePoll::updater(&space_pk, &poll_sk_entity).with_updated_at(now);

    let action_pk =
        CompositePartition::<SpacePartition, String>(space_pk.clone().into(), poll_sk.to_string());
    let mut action_updater =
        SpaceAction::updater(&action_pk, &EntityType::SpaceAction).with_updated_at(now);
    let mut update_action = false;

    match req {
        UpdatePollRequest::Title { title } => {
            poll_updater = poll_updater.with_title(title.clone());
            action_updater = action_updater.with_title(title);
            update_action = true;
        }
        UpdatePollRequest::Question { questions } => {
            if questions.is_empty() {
                return Err(SpacePollError::QuestionsEmpty.into());
            }
            poll_updater = poll_updater.with_questions(questions);
        }
        UpdatePollRequest::ResponseEditable { response_editable } => {
            poll_updater = poll_updater.with_response_editable(response_editable);
        }
        UpdatePollRequest::CanisterUploadEnabled {
            canister_upload_enabled,
        } => {
            let env = crate::common::config::Environment::default();
            if env == crate::common::config::Environment::Production {
                return Err(SpacePollError::InvalidQuestionFormat.into());
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
