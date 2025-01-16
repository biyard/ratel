use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
    log::root,
};
use dto::*;
use rest_api::Signature;
use slog::o;

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerV1 {
    log: slog::Logger,
}

impl AssemblyMemberControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "AssemblyMemberControllerV1"));
        let ctrl = AssemblyMemberControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_assembly_members))
            .route("/parties", get(Self::list_parties))
            .route("/:id", post(Self::act_assembly_member_by_id))
            .with_state(ctrl.clone()))
    }

    pub async fn act_assembly_member_by_id(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Path(_id): Path<String>,
        Extension(_sig): Extension<Option<Signature>>,
        Json(body): Json<AssemblyMemberByIdActionRequest>,
    ) -> Result<Json<AssemblyMemberByIdActionResponse>> {
        let log = ctrl.log.new(o!("api" => "act_assembly_member_by_id"));
        slog::debug!(log, "act_assembly_member_by_id: {:?}", body);
        Ok(Json(AssemblyMemberByIdActionResponse::default()))
    }

    pub async fn list_assembly_members(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Query(req): Query<AssemblyMembersQuery>,
    ) -> Result<Json<CommonQueryResponse<AssemblyMember>>> {
        let log = ctrl.log.new(o!("api" => "list_assembly_members"));
        slog::debug!(log, "list assembly members {:?}", req);

        let lang = req.lang.unwrap_or_default();
        let filter = vec![("gsi1", format!("assembly_member#{}", lang))];

        let res: CommonQueryResponse<AssemblyMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            req.bookmark,
            req.size.map(|s| s as i32),
            filter,
        )
        .await?;

        Ok(Json(res))
    }

    pub async fn list_parties(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Extension(_sig): Extension<Option<Signature>>,
        Query(req): Query<PartiesQuery>,
    ) -> Result<Json<Vec<String>>> {
        let log = ctrl.log.new(o!("api" => "list_parties"));
        slog::debug!(log, "list parties: {req}");
        Ok(Json(vec![
            "test".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ]))
    }
}
