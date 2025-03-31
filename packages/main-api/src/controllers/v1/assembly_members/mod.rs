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
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct AssemblyMemberPath {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerV1 {
    _repo: AssemblyMemberRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl AssemblyMemberControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let _repo = AssemblyMember::get_repository(pool.clone());

        let ctrl = AssemblyMemberControllerV1 { _repo, pool };

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
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(AssemblyMemberPath { id }): Path<AssemblyMemberPath>,
    ) -> Result<Json<AssemblyMember>> {
        tracing::debug!("get_assembly_member {:?}", id);
        //FIXME: if neeeded, use user.id not '0'
        let doc = AssemblyMember::query_builder()
            .bills_builder(Bill::query_builder(0))
            .id_equals(id)
            .query()
            .map(AssemblyMember::from)
            .fetch_one(&ctrl.pool)
            .await?;

        Ok(Json(doc))
    }

    pub async fn list_assembly_member(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<AssemblyMemberParam>,
    ) -> Result<Json<AssemblyMemberGetResponse>> {
        tracing::debug!("list_assembly_member {:?}", param);

        match param {
            AssemblyMemberParam::Query(q) => {
                let mut query_builder = AssemblyMemberSummary::query_builder().limit(q.size());
                let mut total_count = 0;
                if let Some(party) = q.party {
                    query_builder = query_builder.party_equals(party);
                }

                if let Some(stance) = q.stance {
                    query_builder = query_builder.stance_equals(stance);
                }

                query_builder = match (q.sort, q.order) {
                    (Some(AssemblyMemberSorter::Name), Some(SortOrder::Ascending)) => {
                        query_builder.order_by_name_asc()
                    }
                    (Some(AssemblyMemberSorter::Name), Some(SortOrder::Descending)) => {
                        query_builder.order_by_name_desc()
                    }
                    (Some(AssemblyMemberSorter::Stance), Some(SortOrder::Ascending)) => {
                        query_builder.order_by_district_asc()
                    }
                    (Some(AssemblyMemberSorter::Stance), Some(SortOrder::Descending)) => {
                        query_builder.order_by_district_desc()
                    }
                    (Some(AssemblyMemberSorter::Party), Some(SortOrder::Ascending)) => {
                        query_builder.order_by_party_asc()
                    }
                    (Some(AssemblyMemberSorter::Party), Some(SortOrder::Descending)) => {
                        query_builder.order_by_party_desc()
                    }
                    (Some(AssemblyMemberSorter::Bills), Some(SortOrder::Ascending)) => {
                        query_builder.order_by_no_of_bills_asc()
                    }
                    (Some(AssemblyMemberSorter::Bills), Some(SortOrder::Descending)) => {
                        query_builder.order_by_no_of_bills_desc()
                    }
                    _ => query_builder.order_by_random(),
                };

                let items: Vec<AssemblyMemberSummary> = query_builder
                    .query()
                    .map(|row: PgRow| {
                        use sqlx::Row;
                        total_count = row.get("total_count");
                        row.into()
                    })
                    .fetch_all(&ctrl.pool)
                    .await?;

                Ok(Json(AssemblyMemberGetResponse::Query(QueryResponse {
                    total_count,
                    items,
                })))
            }
        }
    }
}
