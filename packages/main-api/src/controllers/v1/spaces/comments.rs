use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct SpaceCommentController {
    _repo: SpaceCommentRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceCommentController {
    async fn query(
        &self,
        space_id: i64,
        _auth: Option<Authorization>,
        param: SpaceCommentQuery,
    ) -> Result<QueryResponse<SpaceComment>> {
        let mut total_count = 0;
        let items: Vec<SpaceComment> = SpaceComment::query_builder()
            .parent_id_equals(space_id)
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

    // async fn create(
    //     &self,
    //     _space_id: i64,
    //     _auth: Option<Authorization>,
    //     _param: SpaceCommentCreateRequest,
    // ) -> Result<SpaceComment> {
    //     todo!()
    // }

    // async fn update(
    //     &self,
    //     id: i64,
    //     auth: Option<Authorization>,
    //     param: SpaceCommentUpdateRequest,
    // ) -> Result<SpaceComment> {
    //     if auth.is_none() {
    //         return Err(Error::Unauthorized);
    //     }

    //     let res = self.repo.update(id, param.into()).await?;

    //     Ok(res)
    // }

    // async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<SpaceComment> {
    //     if auth.is_none() {
    //         return Err(Error::Unauthorized);
    //     }

    //     let res = self.repo.delete(id).await?;

    //     Ok(res)
    // }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     SpaceCommentReadAction { action, .. }: SpaceCommentReadAction,
    // ) -> Result<SpaceComment> {
    //     todo!()
    // }
}

impl SpaceCommentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SpaceComment::get_repository(pool.clone());

        Self { _repo: repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            // .route(
            //     "/:id",
            //     get(Self::get_space_comment_by_id).post(Self::act_space_comment_by_id),
            // )
            // .with_state(self.clone())
            .route(
                "/",
                // post(Self::act_space_comment).
                get(Self::get_space_comment),
            )
            .with_state(self.clone())
    }

    // pub async fn act_space_comment(
    //     State(ctrl): State<SpaceCommentController>,
    //     Path(SpaceCommentParentPath { space_id }): Path<SpaceCommentParentPath>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Json(body): Json<SpaceCommentAction>,
    // ) -> Result<Json<SpaceComment>> {
    //     tracing::debug!("act_space_comment {} {:?}", space_id, body);
    //     match body {
    //         SpaceCommentAction::Create(param) => {
    //             let res = ctrl.create(space_id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // pub async fn act_space_comment_by_id(
    //     State(ctrl): State<SpaceCommentController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Path(SpaceCommentPath { space_id, id }): Path<SpaceCommentPath>,
    //     Json(body): Json<SpaceCommentByIdAction>,
    // ) -> Result<Json<SpaceComment>> {
    //     tracing::debug!("act_space_comment_by_id {} {:?} {:?}", space_id, id, body);

    //     match body {
    //         SpaceCommentByIdAction::Update(param) => {
    //             let res = ctrl.update(id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //         SpaceCommentByIdAction::Delete(_) => {
    //             let res = ctrl.delete(id, auth).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // pub async fn get_space_comment_by_id(
    //     State(ctrl): State<SpaceCommentController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(SpaceCommentPath { space_id, id }): Path<SpaceCommentPath>,
    // ) -> Result<Json<SpaceComment>> {
    //     tracing::debug!("get_space_comment {} {:?}", space_id, id);
    //     Ok(Json(
    //         SpaceComment::query_builder()
    //             .id_equals(id)
    //             .space_id_equals(space_id)
    //             .query()
    //             .map(SpaceComment::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    pub async fn get_space_comment(
        State(ctrl): State<SpaceCommentController>,
        Path(SpaceCommentParentPath { space_id }): Path<SpaceCommentParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<SpaceCommentParam>,
    ) -> Result<Json<SpaceCommentGetResponse>> {
        tracing::debug!("list_space_comment {} {:?}", space_id, q);

        match q {
            SpaceCommentParam::Query(param) => Ok(Json(SpaceCommentGetResponse::Query(
                ctrl.query(space_id, auth, param).await?,
            ))),
            // SpaceCommentParam::Read(param)
            //     if param.action == Some(SpaceCommentReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(SpaceCommentGetResponse::Read(res)))
            // }
        }
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceCommentPath {
    pub space_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceCommentParentPath {
    pub space_id: i64,
}
