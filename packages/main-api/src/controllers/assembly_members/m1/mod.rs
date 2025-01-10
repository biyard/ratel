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
    models::assembly_member::AssemblyMember,
    utils::openapi::*
};

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AssemblyMemberResponse {
    pub request_id: String,
}

// TODO: add authorization (service key or signiture)
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
    ) -> Result<Json<AssemblyMemberResponse>> {
        let log = ctrl.log.new(o!("api" => "create_assembly_member"));
        slog::debug!(log, "act_assembly_member {:?}", body);

        if body == ActionAssemblyMemberRequest::FetchMembers {
            ctrl.fetch_members().await?;
        } else {
            return Err(ServiceError::BadRequest);
        }

        Ok(Json(AssemblyMemberResponse {
            request_id: uuid::Uuid::new_v4().to_string(),
        }))
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

    async fn fetch_members(&self) -> Result<()> {
        let log = self.log.new(o!("api" => "fetch_members"));
        let cli = easy_dynamodb::get_client(&log);

        let members =  get_active_members().await?;

        for member in members {
            let image_url = get_member_profile_image(member.code.clone()).await?;
            let doc: AssemblyMember = AssemblyMember::try_from((member.code.clone(), image_url.clone(), "ko", &member))?;
            cli.upsert(&doc).await.map_err(|e| ServiceError::from(e))?;
            
            // TODO: handle missing value for district field
            let en_member = get_active_member_en(member.code.clone()).await?;
            let en_doc: AssemblyMember = AssemblyMember::try_from((member.code.clone(), image_url.clone(), "en", &en_member))?;
            cli.upsert(&en_doc).await.map_err(|e| ServiceError::from(e))?;
        }

        Ok(())
    }
}
