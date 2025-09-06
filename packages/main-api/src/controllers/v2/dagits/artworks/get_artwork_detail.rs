use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    ArtworkDetail, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};

use crate::utils::users::extract_user_id;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct GetArtworkDetailPathParams {
    #[schemars(description = "ID of the artwork to retrieve")]
    pub artwork_id: i64,
}

pub async fn get_artwork_detail_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(GetArtworkDetailPathParams { artwork_id }): Path<GetArtworkDetailPathParams>,
) -> Result<Json<ArtworkDetail>> {
    let user_id = extract_user_id(&pool, auth).await?;
    let artwork = ArtworkDetail::query_builder()
        .artwork_id_equals(artwork_id)
        .owner_id_equals(user_id)
        .query()
        .map(ArtworkDetail::from)
        .fetch_one(&pool)
        .await?;

    Ok(Json(artwork))
}
