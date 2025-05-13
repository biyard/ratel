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
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: NewsQuery,
    ) -> Result<QueryResponse<NewsSummary>> {
        let mut total_count = 0;
        let items: Vec<NewsSummary> = NewsSummary::query_builder()
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

    async fn create(
        &self,
        _auth: Option<Authorization>,
        _param: NewsCreateRequest,
    ) -> Result<News> {
        todo!()
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: NewsUpdateRequest,
    ) -> Result<News> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<News> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     NewsReadAction { action, .. }: NewsReadAction,
    // ) -> Result<News> {
    //     todo!()
    // }
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
            } // NewsParam::Read(param)
              //     if param.action == Some(NewsReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(NewsGetResponse::Read(res)))
              // }
        }
    }
}
