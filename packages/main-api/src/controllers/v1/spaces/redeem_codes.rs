use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::get,
    },
};
use dto::*;
use uuid::Uuid;

use crate::utils::users::extract_user_with_allowing_anonymous;

#[derive(Clone, Debug)]
pub struct SpaceRedeemCodeController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceRedeemPath {
    pub space_id: i64,
}

impl SpaceRedeemCodeController {
    pub async fn get_redeem_code(
        State(ctrl): State<SpaceRedeemCodeController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SpaceRedeemPath { space_id }): Path<SpaceRedeemPath>,
    ) -> Result<Json<RedeemCode>> {
        let res = ctrl.get_redeem_code_by_id(space_id, auth).await?;
        Ok(Json(res))
    }

    async fn get_redeem_code_by_id(
        &self,
        meta_id: i64,
        auth: Option<Authorization>,
    ) -> Result<RedeemCode> {
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        let mut tx = self.pool.begin().await?;

        let redeem_code = RedeemCode::query_builder()
            .meta_id_equals(meta_id)
            .user_id_equals(user.id)
            .query()
            .map(RedeemCode::from)
            .fetch_optional(&self.pool)
            .await?;
        let redeem_code = if redeem_code.is_none() {
            let redeem_code_repo = RedeemCode::get_repository(self.pool.clone());
            let mut codes = vec![];
            let space = Space::query_builder()
                .id_equals(meta_id)
                .query()
                .map(Space::from)
                .fetch_one(&self.pool)
                .await?;
            for _ in 0..space.num_of_redeem_codes {
                let id = Uuid::new_v4().to_string();
                codes.push(id);
            }
            let res = redeem_code_repo
                .insert_with_tx(&mut *tx, user.id, space.id, codes, vec![])
                .await?;
            if res.is_none() {
                tracing::error!("failed to insert redeem codes for space {meta_id}");
                return Err(Error::RedeemCodeCreationFailure);
            } else {
                res.unwrap()
            }
        } else {
            redeem_code.unwrap()
        };
        tx.commit().await?;

        Ok(redeem_code)
    }

    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_redeem_code))
            .with_state(self.clone())
    }
}
