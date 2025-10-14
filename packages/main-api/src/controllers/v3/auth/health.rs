use crate::{
    AppState, Error2, models::user::User, types::UserType, utils::dynamo_extractor::extract_user,
};
use bdk::prelude::*;
use by_axum::{
    auth::{Authorization, DYNAMO_USER_SESSION_KEY, DynamoUserSession},
    axum::{Extension, extract::State},
};

use tower_sessions::Session;

pub async fn health_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Extension(session): Extension<Session>,
) -> Result<(), Error2> {
    let user = extract_user(&dynamo.client, auth).await;

    if let Ok(user) = user {
        // Revalidate user
        let user_session = DynamoUserSession {
            pk: user.pk.to_string(),
            typ: user.user_type as i64,
        };
        session
            .insert(DYNAMO_USER_SESSION_KEY, user_session)
            .await?;
        return Ok(());
    } else {
        let user = User::new(
            "".to_string(),
            format!("{}@invalid.email", uuid::Uuid::new_v4()),
            "".to_string(),
            false,
            false,
            UserType::Anonymous,
            "".to_string(),
            None,
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
