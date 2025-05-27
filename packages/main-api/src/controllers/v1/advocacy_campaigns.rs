use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::get,
    },
};
use dto::*;

use crate::utils::users::{extract_user_id, extract_user_with_allowing_anonymous};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct AdvocacyCampaignPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct AdvocacyCampaignController {
    #[allow(dead_code)]
    repo: AdvocacyCampaignRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl AdvocacyCampaignController {
    // async fn query(
    //     &self,
    //     _auth: Option<Authorization>,
    //     param: AdvocacyCampaignQuery,
    // ) -> Result<QueryResponse<AdvocacyCampaignSummary>> {
    //     let mut total_count = 0;
    //     let items: Vec<AdvocacyCampaignSummary> = AdvocacyCampaignSummary::query_builder()
    //         .limit(param.size())
    //         .page(param.page())
    //         .query()
    //         .map(|row: PgRow| {
    //             use sqlx::Row;

    //             total_count = row.try_get("total_count").unwrap_or_default();
    //             row.into()
    //         })
    //         .fetch_all(&self.pool)
    //         .await?;

    //     Ok(QueryResponse { total_count, items })
    // }

    async fn agree(
        &self,
        auth: Option<Authorization>,
        id: i64,
        _param: AdvocacyCampaignAgreeRequest,
    ) -> Result<AdvocacyCampaign> {
        let user = extract_user_with_allowing_anonymous(&self.pool, auth)
            .await
            .unwrap_or_default();

        AdvocacyCampaignVoter::get_repository(self.pool.clone())
            .insert(user.id, id)
            .await
            .map_err(|e| {
                tracing::error!("failed to insert advocacy campaign voter: {:?}", e);
                Error::AlreadyVoted
            })?;

        let res = AdvocacyCampaign::query_builder(user.id)
            .id_equals(id)
            .query()
            .map(AdvocacyCampaign::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(res)
    }
}

impl AdvocacyCampaignController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = AdvocacyCampaign::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_advocacy_campaign_by_id).post(Self::post_advocacy_campaign_by_id),
            )
            .with_state(self.clone()))
    }

    pub async fn post_advocacy_campaign_by_id(
        State(ctrl): State<AdvocacyCampaignController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(AdvocacyCampaignPath { id }): Path<AdvocacyCampaignPath>,
        Json(body): Json<AdvocacyCampaignByIdAction>,
    ) -> Result<Json<AdvocacyCampaign>> {
        tracing::debug!("get_advocacy_campaign {:?}", id);

        let res = match body {
            AdvocacyCampaignByIdAction::Agree(param) => ctrl.agree(auth, id, param).await?,
            _ => return Err(Error::BadRequest),
        };

        Ok(Json(res))
    }

    pub async fn get_advocacy_campaign_by_id(
        State(ctrl): State<AdvocacyCampaignController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(AdvocacyCampaignPath { id }): Path<AdvocacyCampaignPath>,
    ) -> Result<Json<AdvocacyCampaign>> {
        tracing::debug!("get_advocacy_campaign {:?}", id);
        let user_id = extract_user_id(&ctrl.pool, auth).await.unwrap_or_default();

        Ok(Json(
            AdvocacyCampaign::query_builder(user_id)
                .id_equals(id)
                .query()
                .map(AdvocacyCampaign::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    // pub async fn get_advocacy_campaign(
    //     State(ctrl): State<AdvocacyCampaignController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Query(q): Query<AdvocacyCampaignParam>,
    // ) -> Result<Json<AdvocacyCampaignGetResponse>> {
    //     tracing::debug!("list_advocacy_campaign {:?}", q);

    //     match q {
    //         AdvocacyCampaignParam::Query(param) => Ok(Json(AdvocacyCampaignGetResponse::Query(
    //             ctrl.query(auth, param).await?,
    //         ))),
    //         // AdvocacyCampaignParam::Read(param)
    //         //     if param.action == Some(AdvocacyCampaignReadActionType::ActionType) =>
    //         // {
    //         //     let res = ctrl.run_read_action(auth, param).await?;
    //         //     Ok(Json(AdvocacyCampaignGetResponse::Read(res)))
    //         // }
    //         _ => Err(Error::BadRequest),
    //     }
    // }
}
