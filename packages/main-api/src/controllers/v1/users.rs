use by_axum::{
    axum::{extract::State, routing::post, Json},
    log::root,
};
use dto::*;
use slog::o;

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
            .with_state(ctrl.clone()))
    }

    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Json(body): Json<UserActionRequest>,
    ) -> Result<Json<User>> {
        let log = ctrl.log.new(o!("api" => "create_user"));
        slog::debug!(log, "list_user {:?}", body);
        Ok(Json(User::default()))
    }
}
