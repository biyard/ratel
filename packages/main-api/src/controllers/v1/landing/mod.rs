use crate::by_axum::axum::routing::get;
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
    },
};
use dto::*;

#[derive(Clone, Debug)]
pub struct LandingController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl LandingController {
    async fn find_one(&self, _auth: Option<Authorization>) -> Result<LandingData> {
        let _pool = self.pool.clone();
        Ok(LandingData {
            my_spaces: vec![],
            following_spaces: vec![],
            follower_list: vec![],
        })
    }
}

impl LandingController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_landing_data))
            .with_state(self.clone()))
    }

    pub async fn get_landing_data(
        State(ctrl): State<LandingController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<LandingDataParam>,
    ) -> Result<Json<LandingDataGetResponse>> {
        tracing::debug!("landing {:?}", q);

        match q {
            LandingDataParam::Read(param)
                if param.action == Some(LandingDataReadActionType::FindOne) =>
            {
                let res = ctrl.find_one(auth).await?;
                Ok(Json(LandingDataGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}
