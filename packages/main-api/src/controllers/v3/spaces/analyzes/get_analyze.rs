use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::{
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    *,
};

pub async fn get_analyze_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<SpaceAnalyze>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let analyze =
        SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze)).await?;

    Ok(Json(analyze.unwrap_or_default()))
}
