use crate::features::ai_moderator::*;

use super::list_materials::MaterialResponse;

const MAX_MATERIALS: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMaterialRequest {
    pub file_name: String,
    pub file_url: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/ai-moderator/materials", role: SpaceUserRole)]
pub async fn upload_material(
    space_id: SpacePartition,
    discussion_sk: String,
    req: UploadMaterialRequest,
) -> Result<MaterialResponse> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id.clone(), discussion_sk.clone());

    let opt = AiModeratorMaterial::opt()
        .sk(EntityType::AiModeratorMaterial(String::default()).to_string());
    let (existing, _) = AiModeratorMaterial::query(cli, &pk, opt).await?;
    if existing.len() >= MAX_MATERIALS {
        return Err(AiModeratorError::MaterialLimitReached.into());
    }

    let material = AiModeratorMaterial::new(space_id, discussion_sk, req.file_name, req.file_url);
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
