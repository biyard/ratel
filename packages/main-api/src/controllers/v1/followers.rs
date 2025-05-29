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

// use crate::security::check_perm;
use crate::utils::users::extract_user_id;

// #[derive(
//     Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
// )]
// pub struct NewsPath {
//     pub id: i64,
// }

#[derive(Clone, Debug)]
pub struct FollowerController {
    repo: FollowerRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl FollowerController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;
        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
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
        auth: Option<Authorization>,
        req: FollowerFollowRequest
    ) -> Result<Follower> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        if let Ok(_already_following) = Follower::query_builder()
            .follower_id_equals(user_id)
            .followed_id_equals(req.followed_id)
            .query()
            .map(Follower::from)
            .fetch_one(&self.pool)
            .await
        {
            return Err(Error::BadRequest);
        }
        
        let follower = self.repo.insert(user_id, req.followed_id).await?;
        Ok(follower)
    }

    // async fn update(
    //     &self,
    //     id: i64,
    //     auth: Option<Authorization>,
    //     param: NewsUpdateRequest,
    // ) -> Result<News> {
    //     let user = check_perm(
    //         &self.pool,
    //         auth,
    //         RatelResource::News,
    //         GroupPermission::UpdateNews,
    //     )
    //     .await?;

    //     btracing::notify!(
    //         crate::config::get().slack_channel_monitor,
    //         &format!(
    //             "admin user({:?}) will update news {:?} with {:?}",
    //             user.email, id, param
    //         )
    //     );
    //     let res = self.repo.update(id, param.into()).await?;

    //     Ok(res)
    // }

    // async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<News> {
    //     let user = check_perm(
    //         &self.pool,
    //         auth,
    //         RatelResource::News,
    //         GroupPermission::DeleteNews,
    //     )
    //     .await?;

    //     let res = self.repo.delete(id).await?;
    //     btracing::notify!(
    //         crate::config::get().slack_channel_monitor,
    //         &format!("admin user({:?}) deleted news({:?})", user.email, res)
    //     );

    //     Ok(res)
    // }
}

impl FollowerController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Follower::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            // .route("/:id", get(Self::get_news_by_id).post(Self::act_news_by_id))
            // .with_state(self.clone())
            .route("/", post(Self::act_follower)
            .get(Self::get_followers))
            .with_state(self.clone()))
    }

    pub async fn act_follower(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<FollowerAction>,
    ) -> Result<Json<Follower>> {
        tracing::debug!("act_follower {:?}", body);
        match body {
            FollowerAction::Follow(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            },
            FollowerAction::Unfollow(_) => todo!(),
        }
    }

    // pub async fn act_news_by_id(
    //     State(ctrl): State<FollowerController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Path(NewsPath { id }): Path<NewsPath>,
    //     Json(body): Json<NewsByIdAction>,
    // ) -> Result<Json<News>> {
    //     tracing::debug!("act_news_by_id {:?} {:?}", id, body);
    //     match body {
    //         NewsByIdAction::Update(param) => {
    //             let res = ctrl.update(id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //         NewsByIdAction::Delete(_) => {
    //             let res = ctrl.delete(id, auth).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // pub async fn get_news_by_id(
    //     State(ctrl): State<FollowerController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(NewsPath { id }): Path<NewsPath>,
    // ) -> Result<Json<News>> {
    //     tracing::debug!("get_news {:?}", id);

    //     Ok(Json(
    //         News::query_builder()
    //             .id_equals(id)
    //             .query()
    //             .map(News::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    pub async fn get_followers(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<FollowerParam>,
    ) -> Result<Json<FollowerGetResponse>> {
        tracing::debug!("list_followers {:?}", q);

        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query(auth, param).await?)))
            }
        }
    }
}
