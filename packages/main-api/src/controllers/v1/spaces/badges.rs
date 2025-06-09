use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, State},
        routing::post,
    },
};
use dto::*;

use crate::utils::users::extract_user_id;

#[derive(Clone, Debug)]
pub struct SpaceBadgeController {
    repo: SpaceBadgeRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceBadgeController {
    // async fn query(
    //     &self,
    //     space_id: i64,
    //     _auth: Option<Authorization>,
    //     param: SpaceBadgeQuery,
    // ) -> Result<QueryResponse<Badge>> {
    //     let mut total_count = 0;
    //     let items: Vec<SpaceBadgeSummary> = SpaceBadgeSummary::query_builder()
    //         .limit(param.size())
    //         .page(param.page())
    //         .space_id_equals(space_id)
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

    async fn create(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
        SpaceBadgeCreateRequest { badges }: SpaceBadgeCreateRequest,
    ) -> Result<SpaceBadge> {
        let mut tx = self.pool.begin().await?;
        let repo = Badge::get_repository(self.pool.clone());
        let creator_id = extract_user_id(&self.pool, auth).await?;

        for b in badges {
            let BadgeCreateRequest {
                name,
                image_url,
                contract,
                token_id,
            } = b;

            let badge = repo
                .insert_with_tx(
                    &mut *tx,
                    creator_id,
                    name,
                    Scope::Space,
                    image_url,
                    contract,
                    token_id,
                )
                .await?
                .ok_or(Error::BadgeCreationFailure)?;

            self.repo
                .insert_with_tx(&mut *tx, space_id, badge.id)
                .await?
                .map(SpaceBadge::from)
                .ok_or(Error::BadgeCreationFailure)?;
        }

        tx.commit().await?;

        Ok(SpaceBadge::default())
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     SpaceBadgeReadAction { action, .. }: SpaceBadgeReadAction,
    // ) -> Result<SpaceBadge> {
    //     todo!()
    // }
}

impl SpaceBadgeController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SpaceBadge::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            // .route(
            //     "/:id",
            //     get(Self::get_space_badge_by_id).post(Self::act_space_badge_by_id),
            // )
            // .with_state(self.clone())
            .route(
                "/",
                post(Self::act_space_badge), // .get(Self::get_space_badge)
            )
            .with_state(self.clone())
    }

    pub async fn act_space_badge(
        State(ctrl): State<SpaceBadgeController>,
        Path(SpaceBadgeParentPath { space_id }): Path<SpaceBadgeParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SpaceBadgeAction>,
    ) -> Result<Json<SpaceBadge>> {
        tracing::debug!("act_space_badge {} {:?}", space_id, body);
        match body {
            SpaceBadgeAction::Create(param) => {
                let res = ctrl.create(space_id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    // pub async fn act_space_badge_by_id(
    //     State(ctrl): State<SpaceBadgeController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Path(SpaceBadgePath { space_id, id }): Path<SpaceBadgePath>,
    //     Json(body): Json<SpaceBadgeByIdAction>,
    // ) -> Result<Json<SpaceBadge>> {
    //     tracing::debug!("act_space_badge_by_id {} {:?} {:?}", space_id, id, body);

    //     match body {
    //         SpaceBadgeByIdAction::Update(param) => {
    //             let res = ctrl.update(id, auth, param).await?;
    //             Ok(Json(res))
    //         }
    //         SpaceBadgeByIdAction::Delete(_) => {
    //             let res = ctrl.delete(id, auth).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    // pub async fn get_space_badge_by_id(
    //     State(ctrl): State<SpaceBadgeController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(SpaceBadgePath { space_id, id }): Path<SpaceBadgePath>,
    // ) -> Result<Json<SpaceBadge>> {
    //     tracing::debug!("get_space_badge {} {:?}", space_id, id);
    //     Ok(Json(
    //         SpaceBadge::query_builder()
    //             .id_equals(id)
    //             .space_id_equals(space_id)
    //             .query()
    //             .map(SpaceBadge::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    // pub async fn get_space_badge(
    //     State(ctrl): State<SpaceBadgeController>,
    //     Path(SpaceBadgeParentPath { space_id }): Path<SpaceBadgeParentPath>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Query(q): Query<SpaceBadgeParam>,
    // ) -> Result<Json<SpaceBadgeGetResponse>> {
    //     tracing::debug!("list_space_badge {} {:?}", space_id, q);

    //     match q {
    //         SpaceBadgeParam::Query(param) => Ok(Json(SpaceBadgeGetResponse::Query(
    //             ctrl.query(space_id, auth, param).await?,
    //         ))),
    //         // SpaceBadgeParam::Read(param)
    //         //     if param.action == Some(SpaceBadgeReadActionType::ActionType) =>
    //         // {
    //         //     let res = ctrl.run_read_action(auth, param).await?;
    //         //     Ok(Json(SpaceBadgeGetResponse::Read(res)))
    //         // }
    //     }
    // }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceBadgePath {
    pub space_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceBadgeParentPath {
    pub space_id: i64,
}
