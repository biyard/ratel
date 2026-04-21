#[cfg(feature = "server")]
use aws_sdk_dynamodb::types::TransactWriteItem;

use super::*;

use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::features::membership::models::{
    ensure_team_membership_monthly_refill, ensure_user_membership_monthly_refill, Membership,
    TeamMembership, UserMembership,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSpaceActionRequest {
    Credits { credits: u64 },
    Status { status: SpaceActionStatus },
    Dependencies { depends_on: Vec<String> },
    Prerequisite { prerequisite: bool },
    Title { title: String },
}

#[post(
    "/api/spaces/{space_id}/actions/{action_id}",
    role: SpaceUserRole,
    user: crate::features::auth::User,
    space: SpaceCommon
)]
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
        .map_err(|e| {
            crate::error!("Failed to get space action: {e:?}");
            SpaceActionError::ActionLoadFailed
        })?
        .ok_or(Error::NotFound("Space action not found".into()))?;

    match req {
        UpdateSpaceActionRequest::Credits { credits } => {
            update_credits(
                cli,
                &user,
                &space,
                &space_id,
                &action_id,
                &pk,
                credits,
                &mut space_action,
                now,
            )
            .await?;
        }
        UpdateSpaceActionRequest::Status { status } => {
            if !SpaceActionStatus::allows_transition(space_action.status.as_ref(), &status) {
                return Err(SpaceActionError::InvalidStatusTransition.into());
            }
            space_action.status = Some(status.clone());
            SpaceAction::updater(&pk, &EntityType::SpaceAction)
                .with_status(status)
                .with_updated_at(now)
                .execute(cli)
                .await
                .map_err(|e| {
                    crate::error!("Failed to update action status: {e:?}");
                    SpaceActionError::ActionUpdateFailed
                })?;
        }
        UpdateSpaceActionRequest::Dependencies { depends_on } => {
            if depends_on.iter().any(|id| id == &action_id) {
                return Err(SpaceActionError::InvalidDependency.into());
            }
            space_action.depends_on = depends_on.clone();
            SpaceAction::updater(&pk, &EntityType::SpaceAction)
                .with_depends_on(depends_on)
                .with_updated_at(now)
                .execute(cli)
                .await
                .map_err(|e| {
                    crate::error!("Failed to update action dependencies: {e:?}");
                    SpaceActionError::ActionUpdateFailed
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
                    crate::error!("Failed to update space action: {e:?}");
                    SpaceActionError::ActionUpdateFailed
                })?;
        }
        UpdateSpaceActionRequest::Title { title } => {
            space_action.title = title.clone();
            SpaceAction::updater(&pk, &EntityType::SpaceAction)
                .with_title(title)
                .with_updated_at(now)
                .execute(cli)
                .await
                .map_err(|e| {
                    crate::error!("Failed to update space action title: {e:?}");
                    SpaceActionError::ActionUpdateFailed
                })?;
        }
    }

    space_action.updated_at = now;
    Ok(space_action)
}

#[cfg(feature = "server")]
async fn update_credits(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    space: &SpaceCommon,
    space_id: &SpacePartition,
    action_id: &str,
    pk: &CompositePartition<SpacePartition, String>,
    credits: u64,
    space_action: &mut SpaceAction,
    now: i64,
) -> Result<()> {
    let behavior = space_action.space_action_type.to_behavior();

    if credits > 0 {
        set_credits(
            cli,
            user,
            space,
            space_id,
            action_id,
            pk,
            credits,
            space_action,
            &behavior,
            now,
        )
        .await
    } else {
        remove_credits(
            cli,
            user,
            space,
            space_id,
            action_id,
            pk,
            space_action,
            &behavior,
            now,
        )
        .await
    }
}

/// Set or update reward credits: validate membership limits, deduct delta, upsert reward.
#[cfg(feature = "server")]
async fn set_credits(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    space: &SpaceCommon,
    space_id: &SpacePartition,
    action_id: &str,
    pk: &CompositePartition<SpacePartition, String>,
    credits: u64,
    space_action: &mut SpaceAction,
    behavior: &RewardUserBehavior,
    now: i64,
) -> Result<()> {
    let old_credits = space_action.credits as i64;
    let credit_delta = credits as i64 - old_credits;
    let (point, period, condition) = get_or_create_reward(cli, behavior).await?;
    let total_points = (credits as i64 * point) as u64;
    space_action.credits = credits;
    space_action.total_points = total_points;

    let membership_item = if matches!(&space.user_pk, Partition::Team(_)) {
        let mut team_membership =
            TeamMembership::get(cli, space.user_pk.clone(), Some(EntityType::TeamMembership))
                .await
                .map_err(|e| {
                    crate::error!("Failed to get team membership: {e:?}");
                    SpaceActionError::MembershipCheckFailed
                })?
                .ok_or(SpaceRewardError::CreditsExceedBalance)?;
        team_membership = ensure_team_membership_monthly_refill(cli, team_membership).await?;

        let membership_pk: Partition = team_membership.membership_pk.clone().into();
        let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
            .await
            .map_err(|e| {
                crate::error!("Failed to get membership: {e:?}");
                SpaceActionError::MembershipCheckFailed
            })?;
        let max_per_space = membership.as_ref().map_or(0, |m| m.max_credits_per_space);
        if max_per_space > 0 && credits as i64 > max_per_space {
            return Err(SpaceRewardError::CreditsExceedMaxPerSpace.into());
        }

        team_membership.use_credits(credit_delta)?;

        TeamMembership::updater(&team_membership.pk, &team_membership.sk)
            .decrease_remaining_credits(credit_delta)
            .with_updated_at(now)
            .transact_write_item()
    } else {
        let mut user_membership =
            UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership))
                .await
                .map_err(|e| {
                    crate::error!("Failed to get user membership: {e:?}");
                    SpaceActionError::MembershipCheckFailed
                })?
                .ok_or(SpaceRewardError::CreditsExceedBalance)?;
        user_membership = ensure_user_membership_monthly_refill(cli, user_membership).await?;

        let membership_pk: Partition = user_membership.membership_pk.clone().into();
        let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
            .await
            .map_err(|e| {
                crate::error!("Failed to get membership: {e:?}");
                SpaceActionError::MembershipCheckFailed
            })?;
        let max_per_space = membership.as_ref().map_or(0, |m| m.max_credits_per_space);
        if max_per_space > 0 && credits as i64 > max_per_space {
            return Err(SpaceRewardError::CreditsExceedMaxPerSpace.into());
        }

        user_membership.use_credits(credit_delta)?;

        UserMembership::updater(&user_membership.pk, &user_membership.sk)
            .decrease_remaining_credits(credit_delta)
            .with_updated_at(now)
            .transact_write_item()
    };

    let items = vec![
        membership_item,
        upsert_space_reward_item(
            space_id, action_id, credits, behavior, point, period, condition, now,
        ),
        update_action_credits_item(pk, credits, total_points, now),
    ];
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::error!("Failed to execute transaction: {e:?}");
        SpaceActionError::TransactionFailed
    })?;

    Ok(())
}

/// Remove reward and refund credits back to user membership.
#[cfg(feature = "server")]
async fn remove_credits(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
    space: &SpaceCommon,
    space_id: &SpacePartition,
    action_id: &str,
    pk: &CompositePartition<SpacePartition, String>,
    space_action: &mut SpaceAction,
    behavior: &RewardUserBehavior,
    now: i64,
) -> Result<()> {
    let existing = SpaceReward::get_by_action(
        cli,
        space_id.clone(),
        action_id.to_string(),
        behavior.clone(),
    )
    .await;

    space_action.credits = 0;
    space_action.total_points = 0;

    let mut items = vec![];

    if let Ok(ref reward) = existing {
        if reward.credits > 0 {
            if matches!(&space.user_pk, Partition::Team(_)) {
                if let Some(ref team_membership) = TeamMembership::get(
                    cli,
                    space.user_pk.clone(),
                    Some(EntityType::TeamMembership),
                )
                .await
                .map_err(|e| {
                    crate::error!("Failed to get team membership: {e:?}");
                    SpaceActionError::MembershipCheckFailed
                })? {
                    let team_membership =
                        ensure_team_membership_monthly_refill(cli, team_membership.clone()).await?;
                    items.push(
                        TeamMembership::updater(&team_membership.pk, &team_membership.sk)
                            .increase_remaining_credits(reward.credits)
                            .with_updated_at(now)
                            .transact_write_item(),
                    );
                }
            } else if let Some(ref um) =
                UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership))
                    .await
                    .map_err(|e| {
                        crate::error!("Failed to get user membership: {e:?}");
                        SpaceActionError::MembershipCheckFailed
                    })?
            {
                let um = ensure_user_membership_monthly_refill(cli, um.clone()).await?;
                items.push(
                    UserMembership::updater(&um.pk, &um.sk)
                        .increase_remaining_credits(reward.credits)
                        .with_updated_at(now)
                        .transact_write_item(),
                );
            }
        }
        items.push(delete_reward_item(reward));
    }

    items.push(update_action_credits_item(pk, 0, 0, now));
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::error!("Failed to execute transaction: {e:?}");
        SpaceActionError::TransactionFailed
    })?;

    Ok(())
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
    .map_err(|e| {
        crate::error!("Failed to get reward template: {e:?}");
        SpaceActionError::RewardTemplateFailed
    })?;

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
                crate::error!("Failed to create reward template: {e:?}");
                SpaceActionError::RewardTemplateFailed
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
