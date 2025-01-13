use by_axum::{
    axum::{
        extract::{Query, State},
        routing::get,
        Json,
    },
    log::root,
};
use dto::*;
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
            .with_state(ctrl.clone()))
    }

    pub async fn list_assembly_members(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Query(req): Query<ListAssemblyMembersRequest>,
    ) -> Result<Json<CommonQueryResponse<AssemblyMember>>> {
        let log = ctrl.log.new(o!("api" => "list_assembly_members"));
        slog::debug!(log, "list assembly members {:?}", req);
        let filter = req.lang.map(|lang| vec![("gsi1", format!("assembly_member#{}", lang))]);

        let res: CommonQueryResponse<AssemblyMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            req.bookmark,
            req.size.map(|s| s as i32),
            filter.unwrap_or_default(),
        )
        .await?;

        Ok(Json(res))
    }
}
