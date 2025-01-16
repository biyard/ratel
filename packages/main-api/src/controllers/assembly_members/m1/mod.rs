use crate::{models::assembly_member::Member, utils::openapi::*};
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
pub struct AssemblyMemberControllerM1 {
    log: slog::Logger,
}

// TODO: add authorization (service key or signiture)
impl AssemblyMemberControllerM1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "AssemblyMemberControllerM1"));
        let ctrl = AssemblyMemberControllerM1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_assembly_member_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_assembly_member))
            .with_state(ctrl.clone()))
    }

    pub async fn act_assembly_member(
        State(ctrl): State<AssemblyMemberControllerM1>,
        Json(body): Json<AssemblyMemberAdminActionRequest>,
    ) -> Result<Json<AssemblyMemberResponse>> {
        let log = ctrl.log.new(o!("api" => "create_assembly_member"));
        slog::debug!(log, "act_assembly_member {:?}", body);

        if body == AssemblyMemberAdminActionRequest::FetchMembers {
            ctrl.fetch_members().await?;
        } else {
            return Err(ServiceError::BadRequest);
        }

        Ok(Json(AssemblyMemberResponse {
            request_id: uuid::Uuid::new_v4().to_string(),
        }))
    }

    pub async fn act_assembly_member_by_id(
        State(ctrl): State<AssemblyMemberControllerM1>,
        Path(id): Path<String>,
        Json(body): Json<AssemblyMemberByIdAdminActionRequest>,
    ) -> Result<()> {
        let log = ctrl.log.new(o!("api" => "update_assembly_member"));
        slog::debug!(log, "act_assembly_member_by_id {:?} {:?}", id, body);
        // TODO: implement it

        Ok(())
    }

    async fn fetch_members(&self) -> Result<()> {
        let log = self.log.new(o!("api" => "fetch_members"));
        let cli = easy_dynamodb::get_client(&log);

        let members = get_active_members().await?;

        for member in members {
            let image_url = get_member_profile_image(member.code.clone()).await?;
            let doc: Member =
                Member::try_from((member.code.clone(), image_url.clone(), "ko", &member))?;
            cli.upsert(&doc).await.map_err(|e| {
                slog::error!(log, "Failed to upsert doc: {}", e);
                ServiceError::from(e)
            })?;

            // TODO: handle missing value for district field
            let en_member = get_active_member_en(member.code.clone()).await?;
            let en_doc: Member =
                Member::try_from((member.code.clone(), image_url.clone(), "en", &en_member))?;
            cli.upsert(&en_doc).await.map_err(|e| {
                slog::error!(log, "Failed to upsert en_doc: {}", e);
                ServiceError::from(e)
            })?;
        }

        Ok(())
    }
}
