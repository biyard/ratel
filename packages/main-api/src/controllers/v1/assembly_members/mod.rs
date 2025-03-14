use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::get,
    },
};
use dto::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct AssemblyMemberPath {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerV1 {
    repo: AssemblyMemberRepository,
}

impl AssemblyMemberControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = AssemblyMember::get_repository(pool);

        let ctrl = AssemblyMemberControllerV1 { repo };

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_assembly_member).post(Self::act_assembly_member_by_id),
            )
            .with_state(ctrl.clone())
            .route("/", get(Self::list_assembly_member))
            .with_state(ctrl.clone()))
    }

    pub async fn act_assembly_member_by_id(
        State(_ctrl): State<AssemblyMemberControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(AssemblyMemberPath { id }): Path<AssemblyMemberPath>,
        Json(body): Json<AssemblyMemberByIdAction>,
    ) -> Result<Json<AssemblyMember>> {
        tracing::debug!("act_assembly_member_by_id {:?} {:?}", id, body);
        match body {
            AssemblyMemberByIdAction::ChangeStance(_params) => {
                // TODO: implement change stance
            }
            AssemblyMemberByIdAction::SendVerifyEmail(_) => {
                // TODO: implement send verify email
            }
        }
        Ok(Json(AssemblyMember::default()))
    }

    pub async fn get_assembly_member(
        State(_ctrl): State<AssemblyMemberControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(AssemblyMemberPath { id }): Path<AssemblyMemberPath>,
    ) -> Result<Json<AssemblyMember>> {
        tracing::debug!("get_assembly_member {:?}", id);
        Ok(Json(AssemblyMember::default()))
    }

    pub async fn list_assembly_member(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<AssemblyMemberParam>,
    ) -> Result<Json<AssemblyMemberGetResponse>> {
        tracing::debug!("list_assembly_member {:?}", q);

        match q {
            AssemblyMemberParam::Query(q) => {
                let docs = ctrl.repo.find(&q).await?;
                Ok(Json(AssemblyMemberGetResponse::Query(docs)))
            }
        }
    }
}
