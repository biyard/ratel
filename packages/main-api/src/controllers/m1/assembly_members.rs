use by_axum::{
    axum::{
        extract::{Path, State},
        routing::post,
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

// NOTE: This is a real model and recommended to be moved to shared_models
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct AssemblyMember {
    id: String,
    r#type: String,
    crated_at: u64,
    updated_at: u64,
    deleted_at: Option<u64>,

    name: Option<String>,

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    gsi1: String,
    gsi2: String,
}

impl AssemblyMemberControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "AssemblyMemberControllerV1"));
        let ctrl = AssemblyMemberControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_assembly_member_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_assembly_member))
            .with_state(ctrl.clone()))
    }

    pub async fn act_assembly_member(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Json(body): Json<ActionAssemblyMemberRequest>,
    ) -> Result<Json<AssemblyMember>> {
        let log = ctrl.log.new(o!("api" => "create_assembly_member"));
        slog::debug!(log, "act_assembly_member {:?}", body);
        // TODO: implement it

        Ok(Json(AssemblyMember::default()))
    }

    pub async fn act_assembly_member_by_id(
        State(ctrl): State<AssemblyMemberControllerV1>,
        Path(id): Path<String>,
        Json(body): Json<ActionAssemblyMemberByIdRequest>,
    ) -> Result<()> {
        let log = ctrl.log.new(o!("api" => "update_assembly_member"));
        slog::debug!(log, "act_assembly_member_by_id {:?} {:?}", id, body);
        // TODO: implement it

        Ok(())
    }
}
