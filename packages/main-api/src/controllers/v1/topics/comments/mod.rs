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
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: CommentRepository,
    user: UserRepository,
    like: CommentLikeRepository,
}

impl CommentControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Comment::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let like = CommentLike::get_repository(pool.clone());
        let ctrl = CommentControllerV1 {
            pool,
            repo,
            user,
            like,
        };

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
        State(ctrl): State<CommentControllerV1>,
        Path(topic_id): Path<i64>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<CommentAction>,
    ) -> Result<Json<Comment>> {
        tracing::debug!("act_comment {} {:?}", topic_id, body);

        match body {
            CommentAction::Comment(req) => ctrl.comment(topic_id, req).await,
        }
    }

    pub async fn act_comment_by_id(
        State(ctrl): State<CommentControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((topic_id, id)): Path<(i64, i64)>,
        Json(body): Json<CommentByIdAction>,
    ) -> Result<Json<Comment>> {
        tracing::debug!("act_comment_by_id {} {:?} {:?}", topic_id, id, body);

        match body {
            CommentByIdAction::Like(_) => ctrl.like(id).await,
        }
    }

    pub async fn get_comment(
        State(ctrl): State<CommentControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((topic_id, id)): Path<(i64, i64)>,
    ) -> Result<Json<Comment>> {
        tracing::debug!("get_comment {} {:?}", topic_id, id);

        let user = ctrl
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let comment = ctrl
            .repo
            .find_one(user.id, &CommentReadAction::new().find_by_id(id))
            .await?;

        if comment.topic_id != topic_id {
            return Err(ServiceError::BadRequest);
        }

        Ok(Json(comment))
    }

    pub async fn list_comment(
        State(ctrl): State<CommentControllerV1>,
        Path(parent_id): Path<String>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<CommentParam>,
    ) -> Result<Json<CommentGetResponse>> {
        tracing::debug!("list_comment {} {:?}", parent_id, param);

        match param {
            CommentParam::Query(q) => ctrl.list_by_topic_id(parent_id, q).await,
            _ => Err(ServiceError::BadRequest)?,
        }
    }
}

impl CommentControllerV1 {
    async fn comment(&self, parent_id: i64, content: String) -> Result<Json<Comment>> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let comment = self.repo.insert(parent_id, user.id, content).await?;

        Ok(Json(comment))
    }

    async fn list_by_topic_id(
        &self,
        parent_id: String,
        q: CommentQuery,
    ) -> Result<Json<CommentGetResponse>> {
        let topic_id = parent_id.parse::<i64>()?;

        let query = CommentSummary::base_sql_with("where topic_id = $1 limit $2 offset $3");
        tracing::debug!("list_by_topic_id query: {}", query);

        let mut total_count: i64 = 0;
        let items: Vec<CommentSummary> = sqlx::query(&query)
            .bind(topic_id)
            .bind(q.size as i64)
            .bind(
                q.size as i64
                    * (q.bookmark
                        .unwrap_or("1".to_string())
                        .parse::<i64>()
                        .unwrap()
                        - 1),
            )
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(CommentGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    async fn like(&self, id: i64) -> Result<Json<Comment>> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        match self
            .like
            .find_one(&CommentLikeReadAction::new().find_by_id(id))
            .await
        {
            Ok(like) => {
                self.like.delete(like.id).await?;
            }
            Err(_) => {
                self.like.insert(id, user.id).await?;
            }
        }

        let comment = self
            .repo
            .find_one(user.id, &CommentReadAction::new().find_by_id(id))
            .await?;

        Ok(Json(comment))
    }
}
