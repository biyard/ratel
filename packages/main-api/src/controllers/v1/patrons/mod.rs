use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use dto::*;

#[derive(Clone, Debug)]
pub struct PatronControllerV1 {
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
        Path(id): Path<i64>,
    ) -> Result<Json<Patron>> {
        tracing::debug!("get_patron {:?}", id);

        let patron = ctrl
            .repo
            .find_one(&PatronReadAction::new().find_by_id(id))
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
            _ => Err(ServiceError::BadRequest),
        }
    }
}

impl PatronControllerV1 {
    async fn create_patron(&self, req: PatronCreateRequest) -> Result<Patron> {
        let user = self
            .user
            .find_one(&UserReadAction::new().user_info())
            .await?;

        let mut patron = self
            .repo
            .insert(user.id, req.wallet_address, req.amount)
            .await?;
        for feature in req.features.iter() {
            self.feature
                .insert(
                    patron.id,
                    feature.title.clone(),
                    feature.reference.clone(),
                    feature.description.clone(),
                    feature.attaches.clone(),
                    feature.status,
                )
                .await?;
            patron.features.push(feature.clone());
        }
        Ok(patron)
    }
}
