use crate::{AppState, models::user::User, types::UserType, utils::dynamo_extractor::extract_user};
use dto::{
    Error, Result,
    by_axum::{
        auth::{Authorization, DYNAMO_USER_SESSION_KEY, DynamoUserSession},
        axum::{Extension, extract::State},
    },
};

use tower_sessions::Session;

pub async fn health_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Extension(session): Extension<Session>,
) -> Result<()> {
    let user = extract_user(&dynamo.client, auth).await;

    if let Ok(user) = user {
        // Revalidate user
        let user_session = DynamoUserSession {
            pk: user.pk.to_string(),
            typ: user.user_type as i64,
        };
        session
            .insert(DYNAMO_USER_SESSION_KEY, user_session)
            .await
            .map_err(|e| Error::DatabaseException(e.to_string()))?;
        return Ok(());
    } else {
        let user = User::new(
            "".to_string(),
            "anonymous@invalid.email".to_string(),
            "".to_string(),
            false,
            false,
            UserType::Anonymous,
            None,
            "".to_string(),
            "".to_string(),
        );

        user.create(&dynamo.client).await?;

        session
            .insert(
                DYNAMO_USER_SESSION_KEY,
                DynamoUserSession {
                    pk: user.pk.to_string(),
                    typ: user.user_type as i64,
                },
            )
            .await?;
    }

    Ok(())
}
