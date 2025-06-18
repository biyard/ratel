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
pub struct NetworkController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl NetworkController {
    async fn find_one(&self, _auth: Option<Authorization>) -> Result<NetworkData> {
        let _pool = self.pool.clone();
        Ok(NetworkData {
            industries: vec![],
            suggested_teams: vec![],
            suggested_users: vec![],
        })
    }
}

impl NetworkController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_network_data))
            .with_state(self.clone()))
    }

    pub async fn get_network_data(
        State(ctrl): State<NetworkController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<NetworkDataParam>,
    ) -> Result<Json<NetworkDataGetResponse>> {
        tracing::debug!("network {:?}", q);

        match q {
            NetworkDataParam::Read(param)
                if param.action == Some(NetworkDataReadActionType::FindOne) =>
            {
                let res = ctrl.find_one(auth).await?;
                Ok(Json(NetworkDataGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}
