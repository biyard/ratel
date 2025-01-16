use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Json,
    },
    log::root,
};
use common_query_response::CommonQueryResponse;
use dto::*;
use slog::o;

#[derive(Clone, Debug)]
pub struct PatronControllerV1 {
    log: slog::Logger,
}

impl PatronControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "PatronControllerV1"));
        let ctrl = PatronControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_patron))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_patron).get(Self::list_patron))
            .with_state(ctrl.clone()))
    }

    pub async fn act_patron(
        State(ctrl): State<PatronControllerV1>,
        Json(body): Json<PatronActionRequest>,
    ) -> Result<Json<PatronActionResponse>> {
        let log = ctrl.log.new(o!("api" => "create_patron"));
        slog::debug!(log, "list_patron {:?}", body);
        Ok(Json(PatronActionResponse::default()))
    }

    pub async fn get_patron(
        State(ctrl): State<PatronControllerV1>,
        Path(id): Path<String>,
    ) -> Result<Json<Patron>> {
        let log = ctrl.log.new(o!("api" => "get_patron"));
        slog::debug!(log, "get_patron {:?}", id);
        Ok(Json(Patron::default()))
    }

    pub async fn list_patron(
        State(ctrl): State<PatronControllerV1>,
        Query(pagination): Query<PatronQuery>,
    ) -> Result<Json<CommonQueryResponse<Patron>>> {
        let log = ctrl.log.new(o!("api" => "list_patron"));
        slog::debug!(log, "list_patron {:?}", pagination);

        Ok(Json(CommonQueryResponse::default()))
    }
}
