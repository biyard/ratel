use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use dto::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PatronPath {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct PatronControllerV1 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: PatronRepository,
    feature: FeatureRepository,
    user: UserRepository,
}

impl PatronControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Patron::get_repository(pool.clone());
        let feature = Feature::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = PatronControllerV1 {
            pool,
            repo,
            feature,
            user,
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_patron))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_patron).get(Self::list_patron))
            .with_state(ctrl.clone()))
    }

    pub async fn act_patron(
        State(ctrl): State<PatronControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<PatronAction>,
    ) -> Result<Json<Patron>> {
        tracing::debug!("act_patron {:?}", body);

        match body {
            PatronAction::Create(req) => Ok(Json(ctrl.create_patron(req).await?)),
        }
    }

    pub async fn get_patron(
        State(ctrl): State<PatronControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(PatronPath { id }): Path<PatronPath>,
    ) -> Result<Json<Patron>> {
        tracing::debug!("get_patron {:?}", id);

        let patron: Patron = Patron::query_builder()
            .id_equals(id)
            .query()
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&ctrl.pool)
            .await?;

        Ok(Json(patron))
    }

    pub async fn list_patron(
        State(ctrl): State<PatronControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<PatronParam>,
    ) -> Result<Json<PatronGetResponse>> {
        tracing::debug!("list_patron {:?}", param);
        match param {
            PatronParam::Query(q) => Ok(Json(PatronGetResponse::Query(ctrl.repo.find(&q).await?))),
        }
    }
}

impl PatronControllerV1 {
    async fn create_patron(&self, req: PatronCreateRequest) -> Result<Patron> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let mut tx = self.pool.begin().await?;

        let mut patron = match self
            .repo
            .insert_with_tx(&mut *tx, user.id, req.wallet_address, req.amount)
            .await
        {
            Ok(patron) => patron.unwrap(),
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        for feature in req.features.iter() {
            match self
                .feature
                .insert_with_tx(
                    &mut *tx,
                    patron.id,
                    feature.title.clone(),
                    feature.reference.clone(),
                    feature.description.clone(),
                    feature.attaches.clone(),
                    feature.status,
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            }
            patron.features.push(feature.clone());
        }

        tx.commit().await?;

        Ok(patron)
    }
}
