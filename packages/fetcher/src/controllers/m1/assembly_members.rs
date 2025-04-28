use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct AssemblyMemberPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct AssemblyMemberController {
    repo: AssemblyMemberRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl AssemblyMemberController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: AssemblyMemberQuery,
    ) -> Result<QueryResponse<AssemblyMemberSummary>> {
        let mut total_count = 0;
        let items: Vec<AssemblyMemberSummary> = AssemblyMemberSummary::query_builder()
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
}

impl AssemblyMemberController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = AssemblyMember::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_assembly_member))
            .with_state(self.clone()))
    }

    // pub async fn act_assembly_member(
    //     State(ctrl): State<AssemblyMemberController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Json(body): Json<AssemblyMemberAction>,
    // ) -> Result<Json<AssemblyMember>> {
    //     tracing::debug!("act_assembly_member {:?}", body);
    //     match body {
    //         AssemblyMemberAction::Create(param) => {
    //             let res = ctrl.create(auth, param).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // pub async fn act_assembly_member_by_id(
    //     State(ctrl): State<AssemblyMemberController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Path(AssemblyMemberPath { id }): Path<AssemblyMemberPath>,
    //     Json(body): Json<AssemblyMemberByIdAction>,
    // ) -> Result<Json<AssemblyMember>> {
    //     tracing::debug!("act_assembly_member_by_id {:?} {:?}", id, body);
    //     match body {
    //         AssemblyMemberByIdAction::Update(param) => {
    //             let res = ctrl.update(id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //         AssemblyMemberByIdAction::Delete(_) => {
    //             let res = ctrl.delete(id, auth).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // pub async fn get_assembly_member_by_id(
    //     State(ctrl): State<AssemblyMemberController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(AssemblyMemberPath { id }): Path<AssemblyMemberPath>,
    // ) -> Result<Json<AssemblyMember>> {
    //     tracing::debug!("get_assembly_member {:?}", id);

    //     Ok(Json(
    //         AssemblyMember::query_builder()
    //             .id_equals(id)
    //             .query()
    //             .map(AssemblyMember::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    pub async fn get_assembly_member(
        State(ctrl): State<AssemblyMemberController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<AssemblyMemberParam>,
    ) -> Result<Json<AssemblyMemberGetResponse>> {
        tracing::debug!("list_assembly_member {:?}", q);

        match q {
            AssemblyMemberParam::Query(param) => Ok(Json(AssemblyMemberGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            // AssemblyMemberParam::Read(param)
            //     if param.action == Some(AssemblyMemberReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(AssemblyMemberGetResponse::Read(res)))
            // }
        }
    }
}
