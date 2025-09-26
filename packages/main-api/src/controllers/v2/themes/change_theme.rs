use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Result, Theme, UserV2, UserV2RepositoryUpdateRequest,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

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
pub struct ChangeThemeRequest {
    #[schemars(description = "Theme Name")]
    pub theme: Theme,
}

use crate::utils::users::extract_user_id;

pub async fn change_theme_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<ChangeThemeRequest>,
) -> Result<Json<UserV2>> {
    let repo = UserV2::get_repository(pool.clone());
    let user_id = extract_user_id(&pool, auth).await?;

    let theme = req.theme;

    let user = repo
        .update(
            user_id,
            UserV2RepositoryUpdateRequest {
                theme: Some(theme),
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(user))
}
