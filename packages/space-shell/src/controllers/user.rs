#[cfg(feature = "server")]
use common::axum::extract::{Extension, FromRef, FromRequest, Request, State};
#[cfg(feature = "server")]
use common::{models::session::TowerSession, utils::aws::dynamo::Client};
use common::{models::user::User, Partition};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::AppState;

struct Extractor {
    user: Option<User>,
}
#[cfg(feature = "server")]
impl<S> FromRequest<S> for Extractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request(req: Request, state: &S) -> std::result::Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let user = User::from_request(req, &state).await.ok();
        Ok(Extractor { user })
    }
}
/*
AS-IS
#[get("/api/user", state: State<AppState>, ex: Extractor) ]
TO-DO
#[get("/api/user", state: State<AppState>, user: Option<User>, ext1 ...) ]
 */

#[get("/api/user", state: State<AppState>, ex: Extractor) ]
pub async fn get_user() -> std::result::Result<Option<User>, ServerFnError> {
    Ok(ex.user)
}

#[post("/api/login", state: State<AppState>, dynamo: Extension<Client>, session : Extension<TowerSession>)]
pub async fn login() -> std::result::Result<(), ServerFnError> {
    use common::{models::user::SESSION_KEY_USER_ID, EntityType};

    let user = User::get(
        &dynamo,
        Partition::User("00000000-0000-0000-0000-000000000006".to_string()),
        Some(EntityType::User),
    )
    .await
    .map_err(|_| ServerFnError::new("Wrong user".to_string()))?
    .ok_or(ServerFnError::new("Wrong User".to_string()))?;

    let _res = session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await;

    Ok(())
}
