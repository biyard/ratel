use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Json,
    },
    log::root,
};
use dto::{common_query_response::CommonQueryResponse, error::ServiceError};
use slog::o;

#[derive(Clone, Debug)]
pub struct TopicControllerV1 {
    log: slog::Logger,
}

// NOTE: if already have other pagination, please remove this and use defined one.
#[derive(serde::Deserialize)]
pub struct Pagination {
    page: usize,
    size: usize,
    bookmark: String,
}

#[derive(serde::Deserialize)]
pub struct CreateTopicRequest {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateTopicRequest {
    name: Option<String>,
}

// NOTE: This is a real model and recommended to be moved to shared_models
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Topic {
    id: String,
    r#type: String,
    created_at: u64,
    updated_at: u64,
    deleted_at: Option<u64>,

    name: Option<String>,

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    gsi1: String,
    gsi2: String,
}

impl TopicControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router, Box<dyn std::error::Error>> {
        let log = root().new(o!("api-controller" => "TopicControllerV1"));
        let ctrl = TopicControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                post(Self::create_topic)
                    .get(Self::get_topic)
                    .delete(Self::delete_topic)
                    .put(Self::update_topic),
            )
            .with_state(ctrl.clone())
            .route("/", get(Self::list_topic))
            .with_state(ctrl))
    }

    pub async fn create_topic(
        State(_ctrl): State<TopicControllerV1>,

        Path(_id): Path<String>,
        Json(_body): Json<CreateTopicRequest>,
    ) -> Result<Json<Topic>, ServiceError> {
        Ok(Json(Topic::default()))
    }

    pub async fn update_topic(
        State(_ctrl): State<TopicControllerV1>,

        Path(_id): Path<String>,
        Json(_body): Json<UpdateTopicRequest>,
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn get_topic(
        State(_ctrl): State<TopicControllerV1>,

        Path(_id): Path<String>,
    ) -> Result<Json<Topic>, ServiceError> {
        Ok(Json(Topic::default()))
    }

    pub async fn delete_topic(
        State(_ctrl): State<TopicControllerV1>,

        Path(_id): Path<String>,
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn list_topic(
        State(_ctrl): State<TopicControllerV1>,

        Query(_pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<Topic>>, ServiceError> {
        Ok(Json(CommonQueryResponse::default()))
    }
}
