use by_axum::axum::{
    extract::{Query, State},
    routing::get,
    Extension, Json,
};
use dto::*;
use rest_api::Signature;
use sqlx::{Pool, Postgres};
use tracing::instrument;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    users: UserRepository,
}

impl UserControllerV1 {
    pub async fn route(pool: Pool<Postgres>) -> Result<by_axum::axum::Router> {
        let users = User::get_repository(pool);

        users.create_table().await?;

        let ctrl = UserControllerV1 { users };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::read_user).post(Self::act_user))
            .with_state(ctrl.clone()))
    }

    #[instrument]
    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,
        Json(body): Json<UserAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("act_user: sig={:?} {:?}", sig, body);
        let sig = sig.ok_or(ServiceError::Unauthorized)?;
        body.validate()?;

        match body {
            UserAction::Signup(req) => ctrl.signup(req, sig).await,
        }
    }

    #[instrument]
    pub async fn read_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,

        Query(mut req): Query<UserReadAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("read_user: sig={:?}", sig);
        let principal = sig
            .ok_or(ServiceError::Unauthorized)?
            .principal()
            .map_err(|s| {
                tracing::error!("failed to get principal: {:?}", s);
                ServiceError::Unknown(s.to_string())
            })?;
        req.validate()?;

        match req.action {
            Some(UserReadActionType::CheckEmail) => ctrl.check_email(req).await,
            Some(UserReadActionType::UserInfo) => {
                req.principal = Some(principal);
                ctrl.user_info(req).await
            }
            Some(UserReadActionType::Login) => {
                req.principal = Some(principal);
                ctrl.login(req).await
            }
            None | Some(UserReadActionType::ByPrincipal) => Err(ServiceError::BadRequest)?,
        }
    }
}

impl UserControllerV1 {
    #[instrument]
    pub async fn login(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn signup(&self, req: UserSignupRequest, sig: Signature) -> Result<Json<User>> {
        let principal = sig.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            ServiceError::Unauthorized
        })?;

        let user = self
            .users
            .insert(req.nickname, principal, req.email, req.profile_url)
            .await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn check_email(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self
            .users
            .find_one(&req)
            .await
            .map_err(|_| ServiceError::NotFound)?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn user_info(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }
}
