use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

use crate::{security::check_perm, utils::users::extract_user_id};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PromotionPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct PromotionController {
    repo: PromotionRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PromotionController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: PromotionQuery,
    ) -> Result<QueryResponse<PromotionSummary>> {
        let mut total_count = 0;
        let items: Vec<PromotionSummary> = PromotionSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn promote_feed(
        &self,
        auth: Option<Authorization>,
        PromotionPromoteFeedRequest {
            name,
            description,
            feed_id,
            start_at,
            end_at,
        }: PromotionPromoteFeedRequest,
    ) -> Result<Promotion> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let priority = 0;

        let promotion = self
            .repo
            .insert(
                name,
                description,
                PromotionType::Feed,
                PromotionStatus::Requested,
                start_at,
                end_at,
                priority,
                user_id,
                Some(feed_id),
            )
            .await?;

        Ok(promotion)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: PromotionUpdateRequest,
    ) -> Result<Promotion> {
        check_perm(
            &self.pool,
            auth,
            RatelResource::Promotions,
            GroupPermission::ManagePromotions,
        )
        .await?;

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Promotion> {
        check_perm(
            &self.pool,
            auth,
            RatelResource::Promotions,
            GroupPermission::ManagePromotions,
        )
        .await?;

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    async fn approve(&self, id: i64, auth: Option<Authorization>) -> Result<Promotion> {
        check_perm(
            &self.pool,
            auth,
            RatelResource::Promotions,
            GroupPermission::ManagePromotions,
        )
        .await?;

        let res = self
            .repo
            .update(
                id,
                PromotionRepositoryUpdateRequest::new().with_status(PromotionStatus::Approved),
            )
            .await?;

        Ok(res)
    }

    async fn hot_promotion(
        &self,
        _auth: Option<Authorization>,
        _param: PromotionReadAction,
    ) -> Result<Promotion> {
        let now = chrono::Utc::now().timestamp_millis();

        let promotion = Promotion::query_builder()
            .start_at_less_than_equals(now)
            .end_at_greater_than_equals(now)
            .order_by_priority_asc()
            .query()
            .map(Promotion::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(promotion)
    }
}

impl PromotionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Promotion::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_promotion_by_id).post(Self::act_promotion_by_id),
            )
            .with_state(self.clone())
            .route("/", post(Self::act_promotion).get(Self::get_promotion))
            .with_state(self.clone()))
    }

    pub async fn act_promotion(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<PromotionAction>,
    ) -> Result<Json<Promotion>> {
        tracing::debug!("act_promotion {:?}", body);
        match body {
            PromotionAction::PromoteFeed(param) => {
                let res = ctrl.promote_feed(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_promotion_by_id(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(PromotionPath { id }): Path<PromotionPath>,
        Json(body): Json<PromotionByIdAction>,
    ) -> Result<Json<Promotion>> {
        tracing::debug!("act_promotion_by_id {:?} {:?}", id, body);
        match body {
            PromotionByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            PromotionByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
            PromotionByIdAction::Approve(_) => {
                let res = ctrl.approve(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_promotion_by_id(
        State(ctrl): State<PromotionController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(PromotionPath { id }): Path<PromotionPath>,
    ) -> Result<Json<Promotion>> {
        tracing::debug!("get_promotion {:?}", id);

        Ok(Json(
            Promotion::query_builder()
                .id_equals(id)
                .query()
                .map(Promotion::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_promotion(
        State(ctrl): State<PromotionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<PromotionParam>,
    ) -> Result<Json<PromotionGetResponse>> {
        tracing::debug!("list_promotion {:?}", q);

        match q {
            PromotionParam::Query(param) => Ok(Json(PromotionGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            PromotionParam::Read(param)
                if param.action == Some(PromotionReadActionType::HotPromotion) =>
            {
                let res = ctrl.hot_promotion(auth, param).await?;
                Ok(Json(PromotionGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}
