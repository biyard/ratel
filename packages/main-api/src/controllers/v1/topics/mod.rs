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
// use by_types::QueryResponse;
use dto::*;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    repo: TopicRepository,
}

impl TopicControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool.clone());

        let ctrl = TopicControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_topic), // .post(Self::act_topic_by_id)
            )
            .with_state(ctrl.clone())
            .route("/", post(Self::act_topic).get(Self::list_topic))
            .with_state(ctrl.clone())
            .nest(
                "/comments",
                comments::CommentControllerV1::route(pool.clone())?,
            )
            .nest("/votes", votes::VoteControllerV1::route(pool)?))
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

        let topic = ctrl
            .repo
            .find_one(&TopicReadAction::new().find_by_id(id))
            .await?;
        Ok(Json(topic))
    }

    pub async fn list_topic(
        State(ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", params);
        // FIXME: why can't i use this method?
        match params {
            TopicParam::Query(req) => {
                Ok(Json(TopicGetResponse::Query(ctrl.repo.find(&req).await?)))
            }
            TopicParam::Read(req) => Ok(Json(TopicGetResponse::Read(
                ctrl.repo.find_one(&req).await?,
            ))),
        }
    }
}

impl TopicControllerV1 {
    pub async fn create_topic(&self, body: TopicCreateRequest) -> Result<Json<Topic>> {
        tracing::debug!("create_topic {:?}", body);

        let topic = self
            .repo
            .insert(
                body.ended_at,
                // body.user_id,
                body.title,
                body.content,
                body.legislation_link,
                body.solutions,
                body.discussions,
                body.additional_resources,
            )
            .await?;

        Ok(Json(topic))
    }
}
