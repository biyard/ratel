use by_axum::{
    axum::{extract::State, routing::post, Extension, Json},
    log::root,
};
use dto::*;
use rest_api::Signature;
use slog::o;

#[derive(Clone, Debug)]
pub struct AssetControllerV1 {
    log: slog::Logger,
}

impl AssetControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "AssetControllerV1"));
        let ctrl = AssetControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_asset))
            .with_state(ctrl.clone()))
    }

    pub async fn act_asset(
        State(ctrl): State<AssetControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Json(body): Json<AssetActionRequest>,
    ) -> Result<Json<AssetActionResponse>> {
        let log = ctrl.log.new(o!("api" => "create_asset"));
        slog::debug!(log, "list_asset {:?}", body);
        Ok(Json(AssetActionResponse::GetSignedUrl(String::default())))
    }
}
