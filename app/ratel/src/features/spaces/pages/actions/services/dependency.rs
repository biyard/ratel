#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::*;

/// Checks whether every action listed in `action.depends_on` has reached
/// `Finish` status.
///
/// Uses DynamoDB BatchGetItem so a single round-trip handles all
/// dependencies (up to 100 per batch), regardless of list length.
#[cfg(feature = "server")]
pub async fn dependencies_met(
    cli: &aws_sdk_dynamodb::Client,
    space_id: &SpacePartition,
    action: &crate::features::spaces::pages::actions::models::SpaceAction,
) -> crate::common::Result<bool> {
    if action.depends_on.is_empty() {
        return Ok(true);
    }

    let keys: Vec<(CompositePartition<SpacePartition, String>, EntityType)> = action
        .depends_on
        .iter()
        .map(|dep_id| {
            (
                CompositePartition(space_id.clone(), dep_id.clone()),
                EntityType::SpaceAction,
            )
        })
        .collect();

    let expected = keys.len();

    let deps = crate::features::spaces::pages::actions::models::SpaceAction::batch_get(cli, keys)
        .await
        .map_err(|e| {
            crate::error!("failed to load dependency actions: {e:?}");
            SpaceActionError::ActionLoadFailed
        })?;

    // Missing dependency (not returned by batch_get) or any dep not yet
    // Finish → not met. We never return partial-ok.
    if deps.len() != expected {
        return Ok(false);
    }

    Ok(deps
        .iter()
        .all(|d| matches!(d.status, Some(SpaceActionStatus::Finish))))
}
