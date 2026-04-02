use crate::features::ai_moderator::*;

use super::list_materials::MaterialResponse;

const MAX_MATERIALS: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMaterialRequest {
    pub file_name: String,
    pub file_url: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_id}/ai-moderator/materials", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn upload_material(
    space_id: SpacePartition,
    discussion_id: SpaceDiscussionEntityType,
    req: UploadMaterialRequest,
) -> Result<MaterialResponse> {
    role.is_creator()?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    // Server-side premium enforcement
    let membership = crate::features::membership::models::UserMembership::get(
        cli,
        user.pk.clone(),
        Some(EntityType::UserMembership),
    )
    .await?;
    let is_paid = membership
        .as_ref()
        .map_or(false, |m| !m.membership_pk.0.contains("Free"));
    if !is_paid {
        return Err(AiModeratorError::PremiumRequired.into());
    }
    let pk = CompositePartition(space_id.clone(), discussion_id.to_string());

    let opt = AiModeratorMaterial::opt()
        .sk(EntityType::AiModeratorMaterial(String::default()).to_string())
        .limit((MAX_MATERIALS + 1) as i64);
    let (existing, _) = AiModeratorMaterial::query(cli, &pk, opt).await?;
    if existing.len() >= MAX_MATERIALS {
        return Err(AiModeratorError::MaterialLimitReached.into());
    }

    let material = AiModeratorMaterial::new(space_id, discussion_id.to_string(), req.file_name, req.file_url);
    material.create(cli).await?;

    let material_id = match &material.sk {
        EntityType::AiModeratorMaterial(id) => id.clone(),
        _ => String::new(),
    };

    Ok(MaterialResponse {
        material_id,
        file_name: material.file_name,
        file_url: material.file_url,
        created_at: material.created_at,
    })
}
