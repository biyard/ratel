#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::*;

#[cfg(feature = "server")]
pub async fn dependencies_met(
    cli: &aws_sdk_dynamodb::Client,
    space_id: &SpacePartition,
    action: &crate::features::spaces::pages::actions::models::SpaceAction,
) -> crate::common::Result<bool> {
    if action.depends_on.is_empty() {
        return Ok(true);
    }

    for dep_id in &action.depends_on {
        let dep_pk = CompositePartition(space_id.clone(), dep_id.clone());
        let dep = crate::features::spaces::pages::actions::models::SpaceAction::get(
            cli,
            dep_pk,
            Some(EntityType::SpaceAction),
        )
        .await
        .map_err(|e| {
            crate::error!("failed to load dependency action: {e:?}");
            SpaceActionError::ActionLoadFailed
        })?;

        match dep {
            Some(dep) if matches!(dep.status, Some(SpaceActionStatus::Finish)) => continue,
            _ => return Ok(false),
        }
    }

    Ok(true)
}
