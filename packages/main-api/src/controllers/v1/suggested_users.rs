use crate::by_axum::axum::routing::get;
use crate::utils::users::extract_user_with_options;
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
    },
};
use by_types::QueryResponse;
use dto::*;


#[derive(Clone, Debug)]
pub struct SuggestedUsersController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SuggestedUsersController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::query_suggested_users))
            .with_state(self.clone()))
    }

    pub async fn query_suggested_users(
        State(ctrl): State<SuggestedUsersController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(param): Query<UserQuery>,
    ) -> Result<Json<QueryResponse<SuggestedUser>>> {
        tracing::debug!("suggested users query: {:?}", param);
        // Get the current user to exclude from suggestions
        let current_user = extract_user_with_options(&ctrl.pool, auth, false).await?;

        let users = User::query_builder()
            .id_not_equals(current_user.id.clone())
            .term_agreed_is_true()
            .informed_agreed_is_true()
            .user_type_equals(UserType::Individual)
            .order_by_random()
            .limit(param.size())
            .page(param.page())
            .query()
            .map(SuggestedUser::from)
            .fetch_all(&ctrl.pool)
            .await?;

        let total_count = users.len() as i64;
        Ok(Json(QueryResponse { 
            items: users,
            total_count,
        }))
    }
}
