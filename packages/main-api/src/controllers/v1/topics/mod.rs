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
use by_types::QueryResponse;
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
        State(_ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<TopicAction>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("act_topic {:?}", body);
        Ok(Json(Topic::default()))
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
        State(_ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);
        Ok(Json(Topic::default()))
    }

    pub async fn list_topic(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", q);

        Ok(Json(TopicGetResponse::Query(QueryResponse::default())))
    }
}
