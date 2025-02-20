#![allow(dead_code)]
pub mod comments;
pub mod votes;

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use dto::*;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    repo: TopicRepository,
    user: UserRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TopicControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = TopicControllerV1 {
            repo,
            user,
            pool: pool.clone(),
        };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_topic).get(Self::list_topic))
            .with_state(ctrl.clone())
            .route(
                "/:id",
                get(Self::get_topic), // .post(Self::act_topic_by_id)
            )
            .with_state(ctrl.clone())
            .nest(
                "/:id/comments",
                comments::CommentControllerV1::route(pool.clone())?,
            )
            .nest("/:id/votes", votes::VoteControllerV1::route(pool)?))
    }

    pub async fn act_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("act_topic {:?}", body);
        match body {
            TopicAction::Create(req) => ctrl.create_topic(req).await,
        }
    }

    // pub async fn act_topic_by_id(
    //     State(_ctrl): State<TopicControllerV1>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(id): Path<String>,
    //     Json(body): Json<TopicByIdAction>,
    // ) -> Result<Json<Topic>> {
    //     tracing::debug!("act_topic_by_id {:?} {:?}", id, body);
    //     Ok(Json(Topic::default()))
    // }

    pub async fn get_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);

        let id = id.parse::<i64>()?;

        let user = ctrl
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        // FIXME: error returned from database: column \"volume.value\" must appear in the GROUP BY clause or be used in an aggregate function
        let topic = ctrl
            .repo
            .find_one(user.id, &TopicReadAction::new().find_by_id(id))
            .await?;

        // let query = TopicSummary::base_sql_with("where p.id = $1");
        // tracing::debug!("get_topic query {:?}", query);

        // let topic = Topic = sqlx::query(&query)
        //     .map(|r: sqlx::postgres::PgRow| {
        //         tracing::debug!("get_topic row {:?}", r);
        //         Topic::default()
        //     })
        //     .fetch_one(&ctrl.pool)
        //     .await?;

        // let topic = Topic::default();
        Ok(Json(topic))
    }

    pub async fn list_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", params);

        match params {
            TopicParam::Query(q) => Ok(Json(TopicGetResponse::Query(ctrl.repo.find(&q).await?))),
            _ => Err(ServiceError::BadRequest),
        }
    }
}

impl TopicControllerV1 {
    pub async fn create_topic(&self, body: TopicCreateRequest) -> Result<Json<Topic>> {
        tracing::debug!("create_topic {:?}", body);

        match body.status {
            TopicStatus::Ongoing | TopicStatus::Finished | TopicStatus::Cancelled => {
                return Err(ServiceError::BadRequest);
            }
            _ => {}
        }

        let topic = self
            .repo
            .insert(
                body.ended_at,
                // body.user_id,
                body.title,
                body.content,
                None,
                TopicResult::default(),
                body.status,
                body.legislation_link,
                body.solutions,
                body.discussions,
                body.additional_resources,
            )
            .await?;

        Ok(Json(topic))
    }
}
