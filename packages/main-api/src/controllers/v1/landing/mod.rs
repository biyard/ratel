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

use crate::utils::users::extract_user_id;

#[derive(Clone, Debug)]
pub struct LandingController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl LandingController {
    async fn find_one(&self, auth: Option<Authorization>) -> Result<LandingData> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let profile_data = MyInfo::query_builder()
            .id_equals(user_id)
            .query()
            .map(MyInfo::from)
            .fetch_one(&self.pool)
            .await?;

        tracing::debug!("profile data: {:?}", profile_data);

        Ok(LandingData {
            my_spaces: vec![],
            following_spaces: vec![],
            follower_list: vec![],
            profile_data,
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
                tracing::debug!("hello");
                let res = ctrl.find_one(auth).await?;
                Ok(Json(LandingDataGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}
