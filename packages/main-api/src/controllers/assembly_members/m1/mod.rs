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
use crate::{
    models::{
        assembly_member::AssemblyMember,
        openapi::member::{EnMember, Member}
    }, 
    utils::openapi::*
};

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct FetchMemberRequest {   
    _lang: Option<String>,
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
        let cli = easy_dynamodb::get_client(&log);

        if body == ActionAssemblyMemberRequest::FetchMembers {
            if let Some(rows) = get_active_members().await?["row"].as_array() {
                for row in rows {
                    // korean info
                    let member: Member = serde_json::from_value(row.clone())?;

                    // profile image
                    let image_url = get_member_profile_image(member.code.clone()).await?;

                    cli.create(
                        &AssemblyMember::try_from((member.code.clone(), image_url.clone(), "ko", &member))
                    )
                    .await?;

                    // english info
                    if let Some(en_row) = get_active_member_en(member.code.clone()).await?["row"].as_array() {
                        for en_row in en_row {
                            let en_member: EnMember = serde_json::from_value(en_row.clone())?;
                        }
                    }
                }
            }
        }
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
