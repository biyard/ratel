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

use crate::security::check_perm;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct NewsPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct NewsController {
    repo: NewsRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl NewsController {
    pub async fn query(
        &self,
        _auth: Option<Authorization>,
        param: NewsQuery,
    ) -> Result<QueryResponse<NewsSummary>> {
        let mut total_count = 0;
        let items: Vec<NewsSummary> = NewsSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
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

    async fn create(
        &self,
        auth: Option<Authorization>,
        NewsCreateRequest {
            title,
            html_content,
        }: NewsCreateRequest,
    ) -> Result<News> {
        let user = check_perm(
            &self.pool,
            auth,
            RatelResource::News,
            GroupPermission::ManageNews,
        )
        .await?;

        let news = self.repo.insert(title, html_content, user.id).await?;

        Ok(news)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: NewsUpdateRequest,
    ) -> Result<News> {
        let user = check_perm(
            &self.pool,
            auth,
            RatelResource::News,
            GroupPermission::ManageNews,
        )
        .await?;

        btracing::notify!(
            crate::config::get().slack_channel_monitor,
            &format!(
                "admin user({:?}) will update news {:?} with {:?}",
                user.email, id, param
            )
        );
        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<News> {
        let user = check_perm(
            &self.pool,
            auth,
            RatelResource::News,
            GroupPermission::ManageNews,
        )
        .await?;

        let res = self.repo.delete(id).await?;
        btracing::notify!(
            crate::config::get().slack_channel_monitor,
            &format!("admin user({:?}) deleted news({:?})", user.email, res)
        );

        Ok(res)
    }
}

impl NewsController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = News::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_news_by_id).post(Self::act_news_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_news).get(Self::get_news))
            .with_state(self.clone()))
    }

    pub async fn act_news(
        State(ctrl): State<NewsController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<NewsAction>,
    ) -> Result<Json<News>> {
        tracing::debug!("act_news {:?}", body);
        match body {
            NewsAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_news_by_id(
        State(ctrl): State<NewsController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(NewsPath { id }): Path<NewsPath>,
        Json(body): Json<NewsByIdAction>,
    ) -> Result<Json<News>> {
        tracing::debug!("act_news_by_id {:?} {:?}", id, body);
        match body {
            NewsByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            NewsByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_news_by_id(
        State(ctrl): State<NewsController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(NewsPath { id }): Path<NewsPath>,
    ) -> Result<Json<News>> {
        tracing::debug!("get_news {:?}", id);

        Ok(Json(
            News::query_builder()
                .id_equals(id)
                .query()
                .map(News::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_news(
        State(ctrl): State<NewsController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<NewsParam>,
    ) -> Result<Json<NewsGetResponse>> {
        tracing::debug!("list_news {:?}", q);

        match q {
            NewsParam::Query(param) => {
                Ok(Json(NewsGetResponse::Query(ctrl.query(auth, param).await?)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_create_news_without_permission() {
        let TestContext { now, endpoint, .. } = setup().await.unwrap();

        let title = format!("Test News {}", now);
        let html_content = format!("<p>This is a test news content.</p> {now}");

        let news = News::get_client(&endpoint)
            .create(title.clone(), html_content.clone())
            .await;

        assert_eq!(news, Err(Error::Unauthorized));
    }

    #[tokio::test]
    async fn test_create_news() {
        let TestContext {
            admin,
            now,
            endpoint,
            admin_token,
            ..
        } = setup().await.unwrap();
        rest_api::add_authorization(&format!("Bearer {}", admin_token));

        let title = format!("Test News {}", now);
        let html_content = format!("<p>This is a test news content.</p> {now}");

        let news = News::get_client(&endpoint)
            .create(title.clone(), html_content.clone())
            .await
            .unwrap();

        assert_eq!(news.title, title);
        assert_eq!(news.html_content, html_content);
        assert_eq!(news.user_id, admin.id);
    }

    #[tokio::test]
    async fn test_update_news() {
        let TestContext {
            admin,
            now,
            endpoint,
            admin_token,
            ..
        } = setup().await.unwrap();
        rest_api::add_authorization(&format!("Bearer {}", admin_token));

        let title = format!("Test News {}", now);
        let html_content = format!("<p>This is a test news content.</p> {now}");

        let news = News::get_client(&endpoint)
            .create(title.clone(), html_content.clone())
            .await
            .unwrap();

        let new_title = format!("Updated News {}", now);
        let new_html_content = format!("<p>This is an updated news content.</p> {now}");

        let updated_news = News::get_client(&endpoint)
            .update(news.id, new_title.clone(), new_html_content.clone())
            .await
            .unwrap();

        assert_eq!(updated_news.title, new_title);
        assert_eq!(updated_news.html_content, new_html_content);
        assert_eq!(updated_news.user_id, admin.id);
    }
}
