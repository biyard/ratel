#![allow(dead_code)]
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
pub struct CommentControllerV1 {
    repo: CommentRepository,
}

impl CommentControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Comment::get_repository(pool);

        let ctrl = CommentControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_comment), // .post(Self::act_comment_by_id)
            )
            .with_state(ctrl.clone())
            .route("/", post(Self::act_comment).get(Self::list_comment))
            .with_state(ctrl.clone()))
    }

    pub async fn act_comment(
        State(_ctrl): State<CommentControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<CommentAction>,
    ) -> Result<Json<Comment>> {
        tracing::debug!("act_comment {} {:?}", parent_id, body);
        Ok(Json(Comment::default()))
    }

    // pub async fn act_comment_by_id(
    //     State(_ctrl): State<CommentControllerV1>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path((parent_id, id)): Path<(String, String)>,
    //     Json(body): Json<CommentByIdAction>,
    // ) -> Result<Json<Comment>> {
    //     tracing::debug!("act_comment_by_id {} {:?} {:?}", parent_id, id, body);
    //     Ok(Json(Comment::default()))
    // }

    pub async fn get_comment(
        State(_ctrl): State<CommentControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((parent_id, id)): Path<(String, String)>,
    ) -> Result<Json<Comment>> {
        tracing::debug!("get_comment {} {:?}", parent_id, id);
        Ok(Json(Comment::default()))
    }

    pub async fn list_comment(
        State(_ctrl): State<CommentControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<CommentParam>,
    ) -> Result<Json<CommentGetResponse>> {
        tracing::debug!("list_comment {} {:?}", parent_id, q);

        Ok(Json(CommentGetResponse::Query(QueryResponse::default())))
    }
}
