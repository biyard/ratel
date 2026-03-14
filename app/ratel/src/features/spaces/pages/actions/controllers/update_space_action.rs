#[cfg(feature = "server")]
use aws_sdk_dynamodb::types::TransactWriteItem;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSpaceActionRequest {
    Credits { credits: u64 },
    Time { started_at: i64, ended_at: i64 },
    Prerequisite { prerequisite: bool },
}

#[post("/api/spaces/{space_id}/actions/{action_id}", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn update_space_action(
    space_id: SpacePartition,
    action_id: String,
    req: UpdateSpaceActionRequest,
) -> Result<SpaceAction> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id.clone(), action_id.clone());
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let mut space_action = SpaceAction::get(cli, pk.clone(), Some(EntityType::SpaceAction))
        .await
        .map_err(|e| Error::InternalServerError(format!("Failed to get space action: {e:?}")))?
        .ok_or(Error::NotFound("Space action not found".into()))?;

    match req {
        UpdateSpaceActionRequest::Credits { credits } => {
            let behavior = space_action.space_action_type.to_behavior();

            if credits > 0 {
                let (point, period, condition) = get_or_create_reward(cli, &behavior).await?;
                let total_points = (credits as i64 * point) as u64;

                space_action.credits = credits;
                space_action.total_points = total_points;

                let items = vec![
                    upsert_space_reward_item(
                        &space_id, &action_id, credits, &behavior, point, period, condition, now,
                    ),
                    update_action_credits_item(&pk, credits, total_points, now),
                ];
                crate::transact_write_items!(cli, items).map_err(|e| {
                    Error::InternalServerError(format!("Failed to execute transaction: {e:?}"))
                })?;
            } else {
                let existing = SpaceReward::get_by_action(
                    cli,
                    space_id.clone(),
                    action_id.clone(),
                    behavior.clone(),
                )
                .await;

                space_action.credits = 0;
                space_action.total_points = 0;

                let mut items = vec![];
                if let Ok(ref reward) = existing {
                    items.push(delete_reward_item(reward));
                }
                items.push(update_action_credits_item(&pk, 0, 0, now));
                crate::transact_write_items!(cli, items).map_err(|e| {
                    Error::InternalServerError(format!("Failed to execute transaction: {e:?}"))
                })?;
            }
        }
        UpdateSpaceActionRequest::Time {
            started_at,
            ended_at,
        } => {
            if started_at >= ended_at {
                return Err(Error::BadRequest("Invalid time range".into()));
            }
            space_action.started_at = started_at;
            space_action.ended_at = ended_at;
            SpaceAction::updater(&pk, &EntityType::SpaceAction)
                .with_started_at(started_at)
                .with_ended_at(ended_at)
                .with_updated_at(now)
                .execute(cli)
                .await
                .map_err(|e| {
                    Error::InternalServerError(format!("Failed to update space action: {e:?}"))
                })?;
        }
        UpdateSpaceActionRequest::Prerequisite { prerequisite } => {
            space_action.prerequisite = prerequisite;
            SpaceAction::updater(&pk, &EntityType::SpaceAction)
                .with_prerequisite(prerequisite)
                .with_updated_at(now)
                .execute(cli)
                .await
                .map_err(|e| {
                    Error::InternalServerError(format!("Failed to update space action: {e:?}"))
                })?;
        }
    }

    space_action.updated_at = now;
    Ok(space_action)
}

#[cfg(feature = "server")]
async fn get_or_create_reward(
    cli: &aws_sdk_dynamodb::Client,
    behavior: &RewardUserBehavior,
) -> Result<(i64, RewardPeriod, RewardCondition)> {
    use crate::common::models::reward::Reward;

    let existing = Reward::get(
        cli,
        crate::common::types::Partition::Reward,
        Some(behavior.clone()),
    )
    .await
    .map_err(|e| Error::InternalServerError(format!("Failed to get reward template: {e:?}")))?;

    match existing {
        Some(r) => Ok((r.point, r.period, r.condition)),
        None => {
            let reward = Reward::new(
                behavior.clone(),
                10000,
                RewardPeriod::Once,
                RewardCondition::None,
            );
            reward.create(cli).await.map_err(|e| {
                Error::InternalServerError(format!("Failed to create reward template: {e:?}"))
            })?;
            Ok((reward.point, reward.period, reward.condition))
        }
    }
}

#[cfg(feature = "server")]
fn upsert_space_reward_item(
    space_id: &SpacePartition,
    action_id: &str,
    credits: u64,
    behavior: &RewardUserBehavior,
    point: i64,
    period: RewardPeriod,
    condition: RewardCondition,
    now: i64,
) -> TransactWriteItem {
    let space_reward = SpaceReward::new(
        space_id.clone(),
        action_id.to_string(),
        behavior.clone(),
        String::new(),
        credits as i64,
        point,
        period,
        condition,
    );
    space_reward.upsert_transact_write_item()
}

#[cfg(feature = "server")]
fn delete_reward_item(reward: &SpaceReward) -> TransactWriteItem {
    SpaceReward::delete_transact_write_item(reward.pk.clone(), reward.sk.clone())
}

#[cfg(feature = "server")]
fn update_action_credits_item(
    pk: &CompositePartition<SpacePartition, String>,
    credits: u64,
    total_points: u64,
    now: i64,
) -> TransactWriteItem {
    SpaceAction::updater(pk, &EntityType::SpaceAction)
        .with_credits(credits)
        .with_total_points(total_points)
        .with_updated_at(now)
        .transact_write_item()
}
