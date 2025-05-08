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

use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct FeedPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct FeedController {
    repo: FeedRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl FeedController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: FeedQuery,
    ) -> Result<QueryResponse<FeedSummary>> {
        let mut total_count = 0;
        let items: Vec<FeedSummary> = FeedSummary::query_builder()
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

    async fn write_post(
        &self,
        auth: Option<Authorization>,
        FeedWritePostRequest {
            html_contents,
            industry_id,
            title,
        }: FeedWritePostRequest,
    ) -> Result<Feed> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let res = self
            .repo
            .insert(
                html_contents,
                FeedType::Post,
                user_id,
                industry_id,
                None,
                title,
                None,
                None,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert post feed: {:?}", e);
                ServiceError::FeedWritePostError
            })?;

        Ok(res)
    }

    async fn comment(
        &self,
        auth: Option<Authorization>,
        FeedCommentRequest {
            html_contents,
            parent_id,
        }: FeedCommentRequest,
    ) -> Result<Feed> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let parent_id = parent_id.ok_or_else(|| {
            tracing::error!("parent id is missing: {user_id}");
            ServiceError::FeedInvalidParentId
        })?;

        let feed = Feed::query_builder()
            .parent_id_equals(parent_id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await?;

        let res = self
            .repo
            .insert(
                html_contents,
                FeedType::Reply,
                user_id,
                feed.industry_id,
                Some(parent_id),
                None,
                None,
                None,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert comment feed: {:?}", e);
                ServiceError::FeedWriteCommentError
            })?;

        Ok(res)
    }

    async fn review_doc(
        &self,
        auth: Option<Authorization>,
        FeedReviewDocRequest {
            html_contents,
            parent_id,
            part_id: _,
        }: FeedReviewDocRequest,
    ) -> Result<Feed> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let parent_id = parent_id.ok_or_else(|| {
            tracing::error!("parent id is missing: {user_id}");
            ServiceError::FeedInvalidParentId
        })?;

        let feed = Feed::query_builder()
            .parent_id_equals(parent_id)
            .query()
            .map(Feed::from)
            .fetch_one(&self.pool)
            .await?;

        let res = self
            .repo
            .insert(
                html_contents,
                FeedType::DocReview,
                user_id,
                feed.industry_id,
                Some(parent_id),
                None,
                None,
                None,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert comment feed: {:?}", e);
                ServiceError::FeedWriteCommentError
            })?;

        Ok(res)
    }

    async fn repost(
        &self,
        auth: Option<Authorization>,
        FeedRepostRequest {
            html_contents,
            quote_feed_id,
            parent_id,
            industry_id,
        }: FeedRepostRequest,
    ) -> Result<Feed> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        if industry_id.is_none() && parent_id.is_none() {
            return Err(ServiceError::FeedExclusiveParentOrIndustry);
        }

        let quote_feed_id = quote_feed_id.ok_or_else(|| {
            tracing::error!("quote feed id is missing: {user_id}");
            tokio::spawn(async move {
                btracing::notify!(
                    crate::config::get().slack_channel_abusing,
                    "invalid quote feed id:{user_id}"
                );
            });
            ServiceError::FeedInvalidQuoteId
        })?;

        let industry_id = match industry_id {
            Some(industry_id) => industry_id,
            None => {
                let feed = Feed::query_builder()
                    .id_equals(quote_feed_id)
                    .query()
                    .map(Feed::from)
                    .fetch_one(&self.pool)
                    .await?;

                feed.industry_id
            }
        };

        let res = self
            .repo
            .insert(
                html_contents,
                FeedType::Repost,
                user_id,
                industry_id,
                parent_id,
                None,
                None,
                None,
            )
            .await
            .map_err(|e| {
                tracing::error!("failed to insert comment feed: {:?}", e);
                ServiceError::FeedWriteCommentError
            })?;

        Ok(res)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: FeedUpdateRequest,
    ) -> Result<Feed> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Feed> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     FeedReadAction { action, .. }: FeedReadAction,
    // ) -> Result<Feed> {
    //     todo!()
    // }
}

impl FeedController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Feed::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_feed_by_id).post(Self::act_feed_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_feed).get(Self::get_feed))
            .with_state(self.clone()))
    }

    pub async fn act_feed(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<FeedAction>,
    ) -> Result<Json<Feed>> {
        tracing::debug!("act_feed {:?}", body);
        let feed = match body {
            FeedAction::WritePost(param) => ctrl.write_post(auth, param).await?,
            FeedAction::Comment(param) => ctrl.comment(auth, param).await?,
            FeedAction::ReviewDoc(param) => ctrl.review_doc(auth, param).await?,
            FeedAction::Repost(param) => ctrl.repost(auth, param).await?,
        };

        Ok(Json(feed))
    }

    pub async fn act_feed_by_id(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FeedPath { id }): Path<FeedPath>,
        Json(body): Json<FeedByIdAction>,
    ) -> Result<Json<Feed>> {
        tracing::debug!("act_feed_by_id {:?} {:?}", id, body);
        match body {
            FeedByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            FeedByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_feed_by_id(
        State(ctrl): State<FeedController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(FeedPath { id }): Path<FeedPath>,
    ) -> Result<Json<Feed>> {
        tracing::debug!("get_feed {:?}", id);

        Ok(Json(
            Feed::query_builder()
                .id_equals(id)
                .query()
                .map(Feed::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_feed(
        State(ctrl): State<FeedController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FeedParam>,
    ) -> Result<Json<FeedGetResponse>> {
        tracing::debug!("list_feed {:?}", q);

        match q {
            FeedParam::Query(param) => {
                Ok(Json(FeedGetResponse::Query(ctrl.query(auth, param).await?)))
            } // FeedParam::Read(param)
              //     if param.action == Some(FeedReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(FeedGetResponse::Read(res)))
              // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_write_post() {
        let TestContext {
            user: _,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let html_contents = format!("<p>Test {now}</p>");
        let title = Some(format!("Test Title {now}"));
        // predefined industry: Crypto
        let industry_id = 1;

        let res = Feed::get_client(&endpoint)
            .write_post(html_contents.clone(), industry_id, title.clone())
            .await;

        assert!(res.is_ok());

        let feed = res.unwrap();
        assert_eq!(feed.html_contents, html_contents);
        assert_eq!(feed.industry_id, industry_id);
        assert_eq!(feed.title, title);
        assert_eq!(feed.feed_type, FeedType::Post);
    }
}
