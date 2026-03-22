use crate::features::spaces::pages::actions::*;
use crate::features::spaces::pages::actions::actions::discussion::controllers::{
    UpdateDiscussionRequest, update_discussion,
};
use crate::features::spaces::pages::actions::actions::poll::controllers::{
    UpdatePollRequest, update_poll,
};
use crate::features::spaces::pages::actions::actions::quiz::controllers::{
    UpdateQuizRequest, update_quiz,
};

use super::reward_cards::RewardPreviewData;

pub fn selected_actions(actions: &[SpaceActionSummary], selected_ids: &[String]) -> Vec<SpaceActionSummary> {
    selected_ids
        .iter()
        .filter_map(|selected_id| {
            actions
                .iter()
                .find(|action| supports_action_settings(action) && &action.action_id == selected_id)
                .cloned()
        })
        .collect()
}

pub fn available_actions(actions: &[SpaceActionSummary], selected_ids: &[String]) -> Vec<SpaceActionSummary> {
    actions
        .iter()
        .filter(|action| supports_action_settings(action))
        .filter(|action| {
            !selected_ids
                .iter()
                .any(|selected_id| selected_id == &action.action_id)
        })
        .cloned()
        .collect()
}

pub fn supports_action_settings(action: &SpaceActionSummary) -> bool {
    !matches!(action.action_type, SpaceActionType::Follow)
}

pub fn action_label(action: &SpaceActionSummary, lang: &Language, untitled: &str) -> String {
    let title = if action.title.trim().is_empty() {
        untitled.to_string()
    } else {
        action.title.trim().to_string()
    };

    format!("{}: {}", action.action_type.translate(lang), title)
}

pub fn reward_preview_items(actions: &[SpaceActionSummary]) -> Vec<RewardPreviewData> {
    if actions.is_empty() {
        return Vec::new();
    }

    // TODO: Replace these placeholder reward preview values with the actual
    // action-settings rewards API response once that endpoint is implemented.
    vec![RewardPreviewData {
        total_reward: 100_000,
        credits: 10,
        points: 10_000,
    }]
}

pub fn reward_credit_summary() -> (i64, i64) {
    // TODO: Replace these placeholder credit values with the actual membership
    // and reward usage API responses once action-settings reward integration is implemented.
    (2, 50)
}

pub async fn apply_selected_action_dates(
    space_id: SpacePartition,
    actions: Vec<SpaceActionSummary>,
    started_at: i64,
    ended_at: i64,
) -> Result<()> {
    for action in actions {
        match action.action_type {
            SpaceActionType::Poll => {
                let entity_type: EntityType = action
                    .action_id
                    .parse()
                    .map_err(|_| Error::BadRequest("Invalid poll action id".to_string()))?;
                let poll_id: SpacePollEntityType = entity_type
                    .try_into()
                    .map_err(|_| Error::BadRequest("Invalid poll action id".to_string()))?;

                update_poll(
                    space_id.clone(),
                    poll_id,
                    UpdatePollRequest::Time {
                        started_at,
                        ended_at,
                    },
                )
                .await?;
            }
            SpaceActionType::TopicDiscussion => {
                let entity_type: EntityType = action
                    .action_id
                    .parse()
                    .map_err(|_| Error::BadRequest("Invalid discussion action id".to_string()))?;
                let discussion_id: SpacePostEntityType = entity_type
                    .try_into()
                    .map_err(|_| Error::BadRequest("Invalid discussion action id".to_string()))?;

                update_discussion(
                    space_id.clone(),
                    discussion_id,
                    UpdateDiscussionRequest {
                        title: None,
                        html_contents: None,
                        category_name: None,
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                    },
                )
                .await?;
            }
            SpaceActionType::Quiz => {
                let entity_type: EntityType = action
                    .action_id
                    .parse()
                    .map_err(|_| Error::BadRequest("Invalid quiz action id".to_string()))?;
                let quiz_id: SpaceQuizEntityType = entity_type
                    .try_into()
                    .map_err(|_| Error::BadRequest("Invalid quiz action id".to_string()))?;

                update_quiz(
                    space_id.clone(),
                    quiz_id,
                    UpdateQuizRequest {
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                        ..Default::default()
                    },
                )
                .await?;
            }
            SpaceActionType::Follow => {
                continue;
            }
        }
    }

    Ok(())
}

