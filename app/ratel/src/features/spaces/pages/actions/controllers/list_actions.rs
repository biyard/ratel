use super::*;
#[cfg(feature = "server")]
use crate::features::auth::models::user::OptionalUser;

#[get("/api/spaces/{space_pk}/actions", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_actions(space_pk: SpacePartition) -> Result<Vec<SpaceActionSummary>> {
    let cli = crate::features::spaces::pages::actions::config::get()
        .common
        .dynamodb();
    let space_pk: Partition = space_pk.into();

    let (space_actions, _) = SpaceAction::find_by_space(cli, &space_pk, SpaceAction::opt())
        .await
        .map_err(|e| Error::InternalServerError(format!("failed to load actions: {e:?}")))?;

    let mut actions: Vec<SpaceActionSummary> = space_actions.into_iter().map(Into::into).collect();

    // Sort by started_at descending
    actions.sort_by(|a, b| b.started_at.cmp(&a.started_at));

    debug!("actions: {:?}", actions);
    Ok(actions)
}
