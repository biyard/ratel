use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::SpaceFile;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct GetSpaceFileResponse {
    pub files: Vec<File>,
}

pub async fn get_files_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<GetSpaceFileResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceFile::keys(&space_pk);

    let files = SpaceFile::get(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    let files = files.unwrap_or_default();

    Ok(Json(GetSpaceFileResponse {
        files: files.clone().files,
    }))
}
