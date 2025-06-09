use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct SpaceContractController {
    repo: SpaceContractRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceContractController {
    async fn query(
        &self,
        space_id: i64,
        _auth: Option<Authorization>,
        param: SpaceContractQuery,
    ) -> Result<QueryResponse<SpaceContractSummary>> {
        let mut total_count = 0;
        let items: Vec<SpaceContractSummary> = SpaceContractSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .space_id_equals(space_id)
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
    //     _param: SpaceContractCreateRequest,
    // ) -> Result<SpaceContract> {
    //     todo!()
    // }

    // async fn update(
    //     &self,
    //     id: i64,
    //     auth: Option<Authorization>,
    //     param: SpaceContractUpdateRequest,
    // ) -> Result<SpaceContract> {
    //     if auth.is_none() {
    //         return Err(Error::Unauthorized);
    //     }

    //     let res = self.repo.update(id, param.into()).await?;

    //     Ok(res)
    // }

    // async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<SpaceContract> {
    //     if auth.is_none() {
    //         return Err(Error::Unauthorized);
    //     }

    //     let res = self.repo.delete(id).await?;

    //     Ok(res)
    // }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     SpaceContractReadAction { action, .. }: SpaceContractReadAction,
    // ) -> Result<SpaceContract> {
    //     todo!()
    // }
}

impl SpaceContractController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SpaceContract::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_space_contract_by_id).post(Self::act_space_contract_by_id),
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_space_contract).get(Self::get_space_contract),
            )
            .with_state(self.clone())
    }

    // pub async fn act_space_contract(
    //     State(ctrl): State<SpaceContractController>,
    //     Path(SpaceContractParentPath { space_id }): Path<SpaceContractParentPath>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Json(body): Json<SpaceContractAction>,
    // ) -> Result<Json<SpaceContract>> {
    //     tracing::debug!("act_space_contract {} {:?}", space_id, body);
    //     match body {
    //         SpaceContractAction::Create(param) => {
    //             let res = ctrl.create(space_id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    pub async fn act_space_contract_by_id(
        State(ctrl): State<SpaceContractController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SpaceContractPath { space_id, id }): Path<SpaceContractPath>,
        Json(body): Json<SpaceContractByIdAction>,
    ) -> Result<Json<SpaceContract>> {
        tracing::debug!("act_space_contract_by_id {} {:?} {:?}", space_id, id, body);

        match body {
            SpaceContractByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            SpaceContractByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    // pub async fn get_space_contract_by_id(
    //     State(ctrl): State<SpaceContractController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(SpaceContractPath { space_id, id }): Path<SpaceContractPath>,
    // ) -> Result<Json<SpaceContract>> {
    //     tracing::debug!("get_space_contract {} {:?}", space_id, id);
    //     Ok(Json(
    //         SpaceContract::query_builder()
    //             .id_equals(id)
    //             .space_id_equals(space_id)
    //             .query()
    //             .map(SpaceContract::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    // pub async fn get_space_contract(
    //     State(ctrl): State<SpaceContractController>,
    //     Path(SpaceContractParentPath { space_id }): Path<SpaceContractParentPath>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Query(q): Query<SpaceContractParam>,
    // ) -> Result<Json<SpaceContractGetResponse>> {
    //     tracing::debug!("list_space_contract {} {:?}", space_id, q);

    //     match q {
    //         SpaceContractParam::Query(param) => Ok(Json(SpaceContractGetResponse::Query(
    //             ctrl.query(space_id, auth, param).await?,
    //         ))),
    //         // SpaceContractParam::Read(param)
    //         //     if param.action == Some(SpaceContractReadActionType::ActionType) =>
    //         // {
    //         //     let res = ctrl.run_read_action(auth, param).await?;
    //         //     Ok(Json(SpaceContractGetResponse::Read(res)))
    //         // }
    //     }
    // }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceContractPath {
    pub space_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceContractParentPath {
    pub space_id: i64,
}
