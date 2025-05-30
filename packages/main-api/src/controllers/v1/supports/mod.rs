use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use dto::*;

#[derive(Clone, Debug)]
pub struct SupportController {
    repo: SupportRepository,
}

impl SupportController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Support::get_repository(pool.clone());
        let ctrl = SupportController { repo };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_support))
            .with_state(ctrl.clone()))
    }

    pub async fn act_support(
        State(ctrl): State<SupportController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<SupportAction>,
    ) -> Result<Json<Support>> {
        tracing::debug!("act_support {:?}", body);

        match body {
            SupportAction::Submit(req) => Ok(Json(ctrl.create_support(req).await?)),
        }
    }
}

impl SupportController {
    async fn create_support(&self, req: SupportSubmitRequest) -> Result<Support> {
        let support = self
            .repo
            .insert(
                req.first_name,
                req.last_name,
                req.email,
                req.company_name,
                req.needs,
                req.help,
            )
            .await?;

        Ok(support)
    }
}
