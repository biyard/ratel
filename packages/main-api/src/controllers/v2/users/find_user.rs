use dto::{
    Error, Result, UserV2,
    by_axum::axum::{
        Json,
        extract::{Query, State},
    },
    sqlx::PgPool,
};
use serde::Deserialize;
use tracing::{debug, error};

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub username: Option<String>,
    #[serde(rename = "phone-number")]
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

pub async fn find_user_handler(
    State(pool): State<PgPool>,
    Query(UserQuery {
        username,
        phone_number,
        email,
    }): Query<UserQuery>,
) -> Result<Json<UserV2>> {
    if let Some(username) = username {
        debug!("Finding user by username: {:?}", username);

        let user: UserV2 = UserV2::query_builder()
            .username_equals(username)
            .query()
            .map(UserV2::from)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by username: {:?}", e);
                Error::NotFound
            })?;

        return Ok(Json(user));
    }

    if let Some(phone_number) = phone_number {
        debug!("Finding user by phone number: {:?}", phone_number);

        let user = UserV2::query_builder()
            .phone_number_equals(phone_number)
            .query()
            .map(UserV2::from)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by phone number: {:?}", e);
                Error::NotFound
            })?;

        return Ok(Json(user));
    }

    if let Some(email) = email {
        debug!("Finding user by email: {:?}", email);

        let cleaned_email = email.replace(' ', "+");

        let user = UserV2::query_builder()
            .email_equals(cleaned_email)
            .query()
            .map(UserV2::from)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                error!("Failed to find user by email: {:?}", e);
                Error::NotFound
            })?;

        return Ok(Json(user));
    }

    Err(Error::InvalidUserQuery(
        "username, phone-number, or email query is required".to_string(),
    ))
}
