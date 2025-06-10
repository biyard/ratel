use crate::by_axum::axum::extract::Path;
use crate::by_axum::axum::routing::post;
use crate::utils::middlewares::authorization_middleware;
use bdk::prelude::*;
use by_axum::auth::Authorization;
use by_axum::axum::{
    Extension, Json,
    extract::{Query, State},
    middleware,
    routing::get,
};
use dto::*;
use rest_api::Signature;
use sqlx::{Pool, Postgres};
use tracing::instrument;
use validator::Validate;

use crate::utils::users::extract_user_id;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    users: UserRepository,
    pool: Pool<Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct UserByIdPath {
    pub id: i64,
}

impl UserControllerV1 {
    pub fn route(pool: Pool<Postgres>) -> Result<by_axum::axum::Router> {
        let users = User::get_repository(pool.clone());

        let ctrl = UserControllerV1 { users, pool };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::read_user).post(Self::act_user))
            .route("/:id", post(Self::act_user_by_id))
            .with_state(ctrl.clone())
            .layer(middleware::from_fn(authorization_middleware)))
    }

    pub async fn act_user_by_id(
        State(ctrl): State<UserControllerV1>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(UserByIdPath { id }): Path<UserByIdPath>,
        Json(body): Json<UserByIdAction>,
    ) -> Result<Json<User>> {
        let user_id = extract_user_id(&ctrl.pool, auth).await?;

        if user_id != id {
            return Err(Error::Unauthorized);
        }

        match body {
            UserByIdAction::EditProfile(req) => ctrl.edit_profile(id, req).await,
        }
    }

    #[instrument]
    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,
        Json(body): Json<UserAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("act_user: sig={:?} {:?}", sig, body);
        let sig = sig.ok_or(Error::Unauthorized)?;
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
        let principal = sig.ok_or(Error::Unauthorized)?.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            Error::Unknown(s.to_string())
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
            None | Some(UserReadActionType::ByPrincipal) => Err(Error::BadRequest)?,
        }
    }
}

impl UserControllerV1 {
    pub async fn edit_profile(&self, id: i64, req: UserEditProfileRequest) -> Result<Json<User>> {
        let user = self
            .users
            .update(
                id,
                UserRepositoryUpdateRequest {
                    nickname: Some(req.nickname),
                    profile_url: Some(req.profile_url),
                    html_contents: Some(req.html_contents),
                    ..Default::default()
                },
            )
            .await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn login(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn signup(&self, req: UserSignupRequest, sig: Signature) -> Result<Json<User>> {
        let principal = sig.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            Error::Unauthorized
        })?;

        if req.term_agreed == false {
            return Err(Error::BadRequest);
        }

        if let Ok(user) = User::query_builder()
            .principal_equals(principal.clone())
            .user_type_equals(UserType::Anonymous)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
        {
            let user = self
                .users
                .update(
                    user.id,
                    UserRepositoryUpdateRequest::new()
                        .with_email(req.email)
                        .with_nickname(req.nickname)
                        .with_profile_url(req.profile_url)
                        .with_term_agreed(req.term_agreed)
                        .with_informed_agreed(req.informed_agreed)
                        .with_username(req.username)
                        .with_user_type(UserType::Individual),
                )
                .await?;

            return Ok(Json(user));
        }

        let user = self
            .users
            .insert(
                req.nickname,
                principal,
                req.email,
                req.profile_url,
                req.term_agreed,
                req.informed_agreed,
                UserType::Individual,
                None,
                req.username,
                "".to_string(),
            )
            .await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn check_email(
        &self,
        UserReadAction { email, .. }: UserReadAction,
    ) -> Result<Json<User>> {
        let user = User::query_builder()
            .email_equals(email.ok_or(Error::InvalidEmail)?)
            .user_type_equals(UserType::Individual)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn user_info(
        &self,
        UserReadAction { principal, .. }: UserReadAction,
    ) -> Result<Json<User>> {
        tracing::debug!("principal 111: {:?}", principal);
        let user = User::query_builder()
            .principal_equals(principal.ok_or(Error::InvalidUser)?)
            .user_type_equals(UserType::Individual)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)?;

        Ok(Json(user))
    }
}
