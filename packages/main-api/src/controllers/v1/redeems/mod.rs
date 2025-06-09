use crate::utils::users::extract_user_with_allowing_anonymous;
use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::post,
    },
};
use dto::*;

#[derive(Clone, Debug)]
pub struct RedeemCodeController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: RedeemCodeRepository,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct RedeemCodeByIdPath {
    id: i64,
}

impl RedeemCodeController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = RedeemCode::get_repository(pool.clone());
        Self { pool, repo }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/:id", post(Self::act_redeem_code_by_id))
            .with_state(self.clone())
    }

    pub async fn act_redeem_code_by_id(
        State(ctrl): State<RedeemCodeController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(RedeemCodeByIdPath { id }): Path<RedeemCodeByIdPath>,
        Json(body): Json<RedeemCodeByIdAction>,
    ) -> Result<Json<RedeemCode>> {
        tracing::debug!("use_redeem_code {:?}", body);

        match body {
            RedeemCodeByIdAction::UseCode(RedeemCodeUseCodeRequest { code }) => {
                let res = ctrl.use_redeem_code(id, auth, code).await?;
                Ok(Json(res))
            }
        }
    }
}

impl RedeemCodeController {
    async fn use_redeem_code(
        &self,
        id: i64,
        auth: Option<Authorization>,
        code: String,
    ) -> Result<RedeemCode> {
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        let mut tx = self.pool.begin().await?;

        let redeem_code = RedeemCode::query_builder()
            .id_equals(id)
            .user_id_equals(user.id)
            .query()
            .map(RedeemCode::from)
            .fetch_optional(&mut *tx)
            .await?;
        if redeem_code.is_none() {
            return Err(Error::RedeemCodeNotFound);
        }

        let redeem_code = redeem_code.unwrap();

        // Check if the code matches
        let code_index = redeem_code.codes.iter().position(|c| c == &code);
        if code_index.is_none() {
            return Err(Error::InvalidRedeemCode);
        };
        let code_index = code_index.unwrap() as i32;

        // Check if the code has already been used
        if redeem_code.used.contains(&code_index) {
            return Ok(redeem_code);
        }

        let res = self
            .repo
            .update_with_tx(
                &mut *tx,
                id,
                RedeemCodeRepositoryUpdateRequest {
                    used: {
                        let mut used = redeem_code.used.clone();
                        used.push(code_index);
                        Some(used)
                    },
                    ..Default::default()
                },
            )
            .await?;

        tx.commit().await?;

        Ok(res.map(RedeemCode::from).ok_or(Error::RedeemCodeNotFound)?)
    }
}
