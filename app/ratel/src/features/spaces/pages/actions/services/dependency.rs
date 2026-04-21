#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::*;

/// Checks whether `user_pk` has personally completed every action listed
/// in `action.depends_on`.
///
/// Completion is evaluated per action type (Poll → UserAnswer exists,
/// Quiz → attempt exists, Discussion → user commented, Follow → user
/// followed the target). The shared dispatcher in
/// `common::types::space_user_role::has_completed_prerequisite_action`
/// is reused so prerequisite and dependency logic stay in lock-step.
///
/// One BatchGetItem round-trip loads every dependency `SpaceAction`;
/// per-type completion checks then run concurrently with `try_join_all`.
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

    // Load every dependency action in one BatchGetItem call.
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

    // A missing dependency (deleted or wrong id) is conservatively
    // treated as not-met.
    if deps.len() != expected {
        return Ok(false);
    }

    // Per-user completion checks run in parallel. Each check dispatches
    // by action type and may hit DynamoDB; doing them concurrently keeps
    // respond latency roughly flat in `depends_on.len()`.
    let checks = deps.iter().map(|dep| {
        crate::common::has_completed_prerequisite_action(cli, space, dep, user_pk)
    });

    let results = try_join_all(checks).await?;
    Ok(results.into_iter().all(|completed| completed))
}
