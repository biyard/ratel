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

    let mut files = files.unwrap_or_default();

    // Lazy migration: auto-generate IDs for files that don't have them
    let mut needs_update = false;
    for file in &mut files.files {
        if file.id.is_empty() {
            file.id = uuid::Uuid::new_v4().to_string();
            needs_update = true;
        }
    }

    // Save back to DynamoDB if we generated any IDs
    if needs_update {
        SpaceFile::updater(&pk, sk)
            .with_files(files.files.clone())
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(GetSpaceFileResponse {
        files: files.clone().files,
    }))
}
