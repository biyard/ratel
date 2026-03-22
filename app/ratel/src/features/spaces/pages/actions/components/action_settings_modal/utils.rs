use crate::features::spaces::pages::actions::*;

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
    if started_at >= ended_at {
        return Err(Error::BadRequest("Invalid time range".to_string()));
    }

    for action in actions {
        if !supports_action_settings(&action) {
            continue;
        }

        update_space_action(
            space_id.clone(),
            action.action_id.clone(),
            UpdateSpaceActionRequest::Time {
                started_at,
                ended_at,
            },
        )
        .await?;
    }

    Ok(())
}

