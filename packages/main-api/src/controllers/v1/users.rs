use by_axum::{
    axum::{extract::State, middleware, routing::post, Extension, Json},
    log::root,
};
use dto::*;
use slog::o;

use crate::utils::middlewares::authorization_middleware;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    log: slog::Logger,
}

impl UserControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "UserControllerV1"));
        let ctrl = UserControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_user))
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
                let user = ctrl.signup(&sig.principal()?, req).await?;

                return Ok(Json(user));
            }
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
