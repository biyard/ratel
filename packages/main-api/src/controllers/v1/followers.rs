use by_axum::auth::Authorization;
use by_axum::axum::{
    Extension, Json,
    extract::{Path, Query, State},
    routing::{get}
};
use by_types::QueryResponse;

use sqlx::postgres::PgRow;

use dto::*;

#[derive(Clone, Debug)]
pub struct FollowerController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct FollowerPath {
    pub id: i64,
}

impl FollowerController {
    async fn query(
        &self,
        user_id: i64,
        _auth: Option<Authorization>,
        param: FollowerQuery,
    ) -> Result<QueryResponse<FollowerSummary>> {
        let mut total_count = 0;
        let items: Vec<FollowerSummary> = FollowerSummary::query_builder()
            .follower_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch suggested users for user {}: {:?}", user_id, e);
                Error::DatabaseException(e.to_string())
            })?;

        Ok(
            QueryResponse {
                items,
                total_count,
            }
        )
    }
}

impl FollowerController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        FollowerController {
            pool,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        let router = by_axum::axum::Router::new()
            .route("/:id", get(Self::get_followers))
            .with_state(self.clone());

        Ok(router)
    }

    pub async fn get_followers(
        State(ctrl): State<FollowerController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(FollowerPath { id }): Path<FollowerPath>,
        Query(q): Query<FollowerParam>,
    ) -> Result<Json<FollowerGetResponse>> {
        match q {
            FollowerParam::Query(param) => {
                Ok(Json(FollowerGetResponse::Query(ctrl.query(id, auth, param).await?)))

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_query_followers() {
        let TestContext { 
            pool, 
            claims,
            user,
            ..
        } = setup().await.unwrap();

        let cli = FollowerController::new(pool.clone());
        let auth = Some(Authorization::Bearer { claims });


        // Test basic query
        let param = FollowerQuery {
            size: 10,
            bookmark: Some("1".to_string()),
            ..Default::default()
        };

        let res = cli.query(user.id, auth, param).await;
        
        assert!(res.is_ok());
    }

}