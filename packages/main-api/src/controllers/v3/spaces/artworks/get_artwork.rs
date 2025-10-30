use bdk::prelude::axum::extract::{Json, Path, State};

use crate::{
    AppState, Error,
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    features::spaces::artworks::{GetSpaceArtworkResponse, SpaceArtwork},
    types::{EntityType, Partition},
};

pub async fn get_space_artwork_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<GetSpaceArtworkResponse>, Error> {
    let space_pk = match space_pk {
        Partition::Space(_) => space_pk,
        _ => return Err(Error::InvalidSpacePartitionKey),
    };

    // Check if SpaceArtwork exists
    let artwork = SpaceArtwork::get(&dynamo.client, &space_pk, Some(EntityType::SpaceArtwork))
        .await?
        .ok_or(Error::ArtworkNotFound)?;

    Ok(Json(GetSpaceArtworkResponse::from(artwork)))
}
