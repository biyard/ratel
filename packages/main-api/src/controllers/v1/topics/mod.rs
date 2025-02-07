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
use by_types::QueryResponse;
// use by_types::QueryResponse;
use dto::*;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    repo: TopicRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TopicControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Topic::get_repository(pool.clone());

        let ctrl = TopicControllerV1 {
            repo,
            pool: pool.clone(),
        };

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
        State(_ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<Topic>> {
        tracing::debug!("get_topic {:?}", id);

        // let topic = ctrl
        //     .repo
        //     .find_one(&TopicReadAction::new().find_by_id(id))
        //     .await?;

        // let query = r#"
        //     SELECT * FROM topics WHERE id = $1
        // "#;

        // let topic = sqlx::query_as::<_, Topic>("SELECT * FROM topics WHERE id = $1")
        //     .bind(id)
        //     .fetch_one(&ctrl.pool)
        //     .await?;
        let topic = Topic::default();
        Ok(Json(topic))
    }

    pub async fn list_topic(
        State(_ctrl): State<TopicControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<TopicParam>,
    ) -> Result<Json<TopicGetResponse>> {
        tracing::debug!("list_topic {:?}", params);

        // match params {
        //     TopicParam::Query(req) => {
        //         Ok(Json(TopicGetResponse::Query(ctrl.repo.find(&req).await?)))
        //     }
        //     TopicParam::Read(req) => Ok(Json(TopicGetResponse::Read(
        //         ctrl.repo.find_one(&req).await?,
        //     ))),
        // }

        // let resp: Vec<TopicSummary> = sqlx::query_as::<_, TopicSummary>("SELECT * FROM topics")
        //     .fetch_all(&ctrl.pool)
        //     .await?;

        // let topics = QueryResponse {
        //     items: resp.clone(),
        //     total_count: resp.len() as i64,
        // };
        let topics: QueryResponse<TopicSummary> = QueryResponse {
            items: vec![],
            total_count: 0,
        };
        Ok(Json(TopicGetResponse::Query(topics)))
    }
}

impl TopicControllerV1 {
    pub async fn create_topic(&self, body: TopicCreateRequest) -> Result<Json<Topic>> {
        tracing::debug!("create_topic {:?}", body);

        // let topic = self
        //     .repo
        //     .insert(
        //         body.ended_at,
        //         // body.user_id,
        //         body.title,
        //         body.content,
        //         body.legislation_link,
        //         body.solutions,
        //         body.discussions,
        //         body.additional_resources,
        //     )
        //     .await?;

        // let query = r#"
        //     INSERT INTO topics (ended_at, title, content, legislation_link, solutions, discussions, additional_resources)
        //     VALUES ($1, $2, $3, $4, $5, $6, $7)
        //     RETURNING *
        // "#;

        // let resp = sqlx::query_as(
        //     r#"
        //     INSERT INTO topics (ended_at, title, content, legislation_link, solutions, discussions, additional_resources)
        //     VALUES ($1, $2, $3, $4, $5, $6, $7)
        //     RETURNING *
        // "#,
        // )
        // .bind(body.ended_at)
        // .bind(body.title)
        // .bind(body.content)
        // .bind(body.legislation_link)
        // .bind(body.solutions)
        // .bind(body.discussions)
        // .bind(body.additional_resources)
        // .fetch_one(&self.pool)
        // .await?;

        let topic = Topic::default();
        Ok(Json(topic))
    }
}
