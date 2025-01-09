use by_axum::{
    axum::{
        extract::{Query, State},
        middleware,
        routing::get,
        Extension, Json,
    },
    log::root,
};
use dto::*;
use slog::o;

use crate::{models, utils::middlewares::authorization_middleware};

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    log: slog::Logger,
}

impl UserControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "UserControllerV1"));
        let ctrl = UserControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::read_user).post(Self::act_user))
            .with_state(ctrl.clone())
            .layer(middleware::from_fn(authorization_middleware)))
    }

    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Signature>,
        Json(body): Json<UserActionRequest>,
    ) -> Result<Json<User>> {
        let log = ctrl.log.new(o!("api" => "act_user"));
        slog::debug!(log, "act_user: sig={:?} {:?}", sig, body);

        match body {
            UserActionRequest::Signup(req) => {
                let principal = sig.principal()?;

                if let Some(user) = models::User::get(&log, &principal).await? {
                    if &user.email == &req.email {
                        return Err(ServiceError::UserAlreadyExists);
                    } else {
                        return Err(ServiceError::Unauthorized);
                    }
                }

                let user = ctrl.signup(&sig.principal()?, req).await?;

                Ok(Json(user))
            }
        }
    }

    pub async fn read_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Signature>,

        Query(req): Query<UserReadActionRequest>,
    ) -> Result<Json<User>> {
        let log = ctrl.log.new(o!("api" => "read_user"));
        slog::debug!(log, "read_user: sig={:?}", sig);

        let principal = sig.principal()?;

        let user = models::User::get(&log, &principal).await?;

        if user.is_none() {
            return Err(ServiceError::NotFound);
        }

        let user = user.unwrap();

        match req.action {
            ReadActionType::CheckEmail => {
                if user.email == req.email.unwrap_or_default() {
                    Ok(Json(user.into()))
                } else {
                    Err(ServiceError::Unauthorized)
                }
            }
            ReadActionType::UserInfo => Ok(Json(user.into())),
        }
    }
}

impl UserControllerV1 {
    pub async fn signup(&self, wallet_address: &str, req: SignupRequest) -> Result<User> {
        let log = self.log.new(o!("api" => "signup"));

        let user = crate::models::user::User::new(
            wallet_address.to_string(),
            req.nickname,
            req.email,
            req.profile_url,
        );
        user.create(&log).await.map_err(|e| ServiceError::from(e))?;

        Ok(user.into())
    }
}
