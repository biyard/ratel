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
        let now = chrono::Utc::now().timestamp();

        let mut total_count = 0;
        let items: Vec<PromotionSummary> = PromotionSummary::query_builder()
            .start_at_less_than_equals(now)
            .end_at_greater_than_equals(now)
            .limit(param.size())
            .page(param.page())
            .order_by_priority_desc()
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
            image_url,
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
                image_url,
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

    async fn approve(
        &self,
        id: i64,
        auth: Option<Authorization>,
        PromotionApproveRequest { priority }: PromotionApproveRequest,
    ) -> Result<Promotion> {
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
                PromotionRepositoryUpdateRequest::new()
                    .with_status(PromotionStatus::Approved)
                    .with_priority(priority),
            )
            .await?;

        Ok(res)
    }

    async fn hot_promotion(
        &self,
        _auth: Option<Authorization>,
        _param: PromotionReadAction,
    ) -> Result<Promotion> {
        let now = chrono::Utc::now().timestamp();

        let promotion = Promotion::query_builder()
            .start_at_less_than_equals(now)
            .end_at_greater_than_equals(now)
            .order_by_priority_desc()
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
            PromotionByIdAction::Approve(param) => {
                let res = ctrl.approve(id, auth, param).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    async fn test_setup(user: &User, now: i64, pool: &sqlx::Pool<sqlx::Postgres>) -> Feed {
        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        let post = Feed::get_repository(pool.clone())
            .insert(
                html_contents.clone(),
                FeedType::Post,
                user.id,
                industry_id,
                Some(user.nickname.clone()),
                Some(user.profile_url.clone()),
                None,
                title,
                None,
                None,
                vec![],
                0,
                0,
            )
            .await
            .unwrap();

        post
    }

    #[tokio::test]
    async fn test_promotions() {
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            admin_token,
            user_token,
            ..
        } = setup().await.unwrap();

        let feed = test_setup(&user, now, &pool).await;

        let cli = Promotion::get_client(&endpoint);
        let start_at = chrono::Utc::now().timestamp() % 3600;
        let end_at = chrono::Utc::now().timestamp() + (3 * 3600);

        let title = format!("Test Title {now}");
        let description = format!("<p>Test {now}</p>");
        let image_url = format!("https://test.com/{now}");

        let promote = cli
            .promote_feed(
                title.clone(),
                description.clone(),
                image_url.clone(),
                start_at,
                end_at,
                feed.id,
            )
            .await
            .expect("failed to promote feed");

        assert_eq!(promote.name, title);
        assert_eq!(promote.description, description);
        assert_eq!(promote.image_url, image_url);
        assert_eq!(promote.feed_id, Some(feed.id));
        assert_eq!(promote.requester_id, user.id);
        assert_eq!(promote.start_at, start_at);
        assert_eq!(promote.end_at, end_at);
        assert_eq!(promote.status, PromotionStatus::Requested);
        assert_eq!(promote.promotion_type, PromotionType::Feed);
        assert_eq!(promote.priority, 0);

        rest_api::add_authorization(&format!("Bearer {}", admin_token));

        let promote = cli
            .approve(promote.id, now)
            .await
            .expect("failed to approve promotion");

        assert_eq!(promote.status, PromotionStatus::Approved);
        assert_eq!(promote.priority, now);

        rest_api::add_authorization(&format!("Bearer {}", user_token));

        let promote = cli
            .hot_promotion()
            .await
            .expect("failed to get hot promotion");

        assert_eq!(promote.name, title);
        assert_eq!(promote.description, description);
        assert_eq!(promote.image_url, image_url);
        assert_eq!(promote.feed_id, Some(feed.id));
        assert_eq!(promote.requester_id, user.id);
        assert_eq!(promote.start_at, start_at);
        assert_eq!(promote.end_at, end_at);
        assert_eq!(promote.status, PromotionStatus::Approved);
        assert_eq!(promote.promotion_type, PromotionType::Feed);
        assert_eq!(promote.priority, now);

        let promotes = cli.query(PromotionQuery::new(10)).await.unwrap();
        assert!(promotes.items.len() >= 1);
        assert_eq!(promotes.items[0].id, promote.id);
    }
}
