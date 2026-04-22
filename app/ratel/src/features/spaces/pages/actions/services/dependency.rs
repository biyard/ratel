#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::*;

#[cfg(feature = "server")]
pub async fn dependencies_met(
    cli: &aws_sdk_dynamodb::Client,
    space: &crate::common::models::space::SpaceCommon,
    action: &crate::features::spaces::pages::actions::models::SpaceAction,
    user_pk: &Partition,
) -> crate::common::Result<bool> {
    use futures::future::try_join_all;

    if action.depends_on.is_empty() {
        return Ok(true);
    }

    let space_id: SpacePartition = match &space.pk {
        Partition::Space(id) => SpacePartition(id.clone()),
        _ => return Ok(false),
    };

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

    if deps.len() != expected {
        return Ok(false);
    }

    let checks = deps
        .iter()
        .map(|dep| crate::common::has_completed_prerequisite_action(cli, space, dep, user_pk));

    let results = try_join_all(checks).await?;
    Ok(results.into_iter().all(|completed| completed))
}
