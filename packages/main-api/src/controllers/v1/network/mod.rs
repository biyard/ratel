use crate::by_axum::axum::routing::get;
use crate::utils::users::extract_user_id;
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
    async fn find_one(&self, auth: Option<Authorization>) -> Result<NetworkData> {
        let industries = Industry::query_builder()
            .query()
            .map(Industry::from)
            .fetch_all(&self.pool)
            .await?;

        let current_user_id = extract_user_id(&self.pool, auth.clone()).await?;

        let suggested_teams_sql = r#"
            SELECT *
            FROM users u
            WHERE u.id != $1
            AND u.user_type = $2
            AND u.id NOT IN (
                SELECT following_id FROM my_networks WHERE follower_id = $1
            )
            ORDER BY RANDOM()
            LIMIT 3
        "#;

        let suggested_users_sql = r#"
            SELECT *
            FROM users u
            WHERE u.id != $1
            AND u.user_type = $2
            AND u.id NOT IN (
                SELECT following_id FROM my_networks WHERE follower_id = $1
            )
            ORDER BY RANDOM()
            LIMIT 5
        "#;

        let suggested_teams = sqlx::query(suggested_teams_sql)
            .bind(current_user_id)
            .bind(UserType::Team as i32)
            .map(Follower::from)
            .fetch_all(&self.pool)
            .await?;

        let suggested_users = sqlx::query(suggested_users_sql)
            .bind(current_user_id)
            .bind(UserType::Individual as i32)
            .map(Follower::from)
            .fetch_all(&self.pool)
            .await?;

        Ok(NetworkData {
            industries,
            suggested_teams,
            suggested_users,
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
