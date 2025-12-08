use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::SpaceFile;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::types::file_location::FileLocation;
use crate::*;
use std::str::FromStr;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct GetSpaceFileResponse {
    pub files: Vec<File>,
}

#[derive(Debug, Clone, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct GetSpaceFilesQuery {
    /// Filter by location (Overview, Board, Files)
    pub location: Option<String>,
}

pub async fn get_files_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    axum::extract::Query(query): axum::extract::Query<GetSpaceFilesQuery>,
) -> Result<Json<GetSpaceFileResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceFile::keys(&space_pk);

    let files = SpaceFile::get(&dynamo.client, &pk.clone(), Some(sk.clone())).await?;

    let mut all_files = files.unwrap_or_default().files;

    // Filter by location if specified
    if let Some(location_str) = query.location {
        if let Ok(filter_location) = FileLocation::from_str(&location_str) {
            all_files = all_files
                .into_iter()
                .filter(|f| f.locations.contains(&filter_location))
                .collect();
        }
    }

    Ok(Json(GetSpaceFileResponse { files: all_files }))
}
