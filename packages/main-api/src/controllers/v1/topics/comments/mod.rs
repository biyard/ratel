#![allow(dead_code)]
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
pub struct CommentControllerV1 {
    repo: CommentRepository,
    user: UserRepository,
}

impl CommentControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Comment::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = CommentControllerV1 { repo, user };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:parent_id/:id",
                get(Self::get_comment), // .post(Self::act_comment_by_id)
            )
            .with_state(ctrl.clone())
            .route(
                "/:parent_id",
                post(Self::act_comment).get(Self::list_comment),
            )
            .with_state(ctrl.clone()))
    }

    pub async fn act_comment(
        State(ctrl): State<CommentControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<CommentAction>,
    ) -> Result<Json<Comment>> {
        tracing::debug!("act_comment {} {:?}", parent_id, body);

        let topic_id = parent_id.parse::<i64>()?;

        match body {
            CommentAction::Comment(req) => ctrl.comment(topic_id, req).await,
        }
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

        let _topic_id = parent_id.parse::<i64>()?;
        let _id = id.parse::<i64>()?;

        // FIXME: find_one method need unnecessary user_id parameter @hackartist

        // let comment = ctrl.repo.find_one(id).await?;

        Ok(Json(Comment::default()))
    }

    pub async fn list_comment(
        State(ctrl): State<CommentControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<CommentParam>,
    ) -> Result<Json<CommentGetResponse>> {
        tracing::debug!("list_comment {} {:?}", parent_id, param);

        let _topic_id = parent_id.parse::<i64>()?;

        match param {
            CommentParam::Query(q) => {
                Ok(Json(CommentGetResponse::Query(ctrl.repo.find(&q).await?)))
            }
        }
    }
}

impl CommentControllerV1 {
    async fn comment(&self, parent_id: i64, content: String) -> Result<Json<Comment>> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let comment = self
            .repo
            .insert(user.profile_url, user.nickname, content, parent_id)
            .await?;

        Ok(Json(comment))
    }
}
