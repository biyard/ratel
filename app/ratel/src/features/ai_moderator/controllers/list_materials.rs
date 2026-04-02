use crate::features::ai_moderator::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialResponse {
    pub material_id: String,
    pub file_name: String,
    pub file_url: String,
    pub created_at: i64,
}

#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/ai-moderator/materials", _role: SpaceUserRole)]
pub async fn list_materials(
    space_id: SpacePartition,
    discussion_sk: String,
) -> Result<Vec<MaterialResponse>> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id, discussion_sk);

    let opt = AiModeratorMaterial::opt()
        .sk(EntityType::AiModeratorMaterial(String::default()).to_string());
    let (materials, _) = AiModeratorMaterial::query(cli, &pk, opt).await?;

    Ok(materials
        .into_iter()
        .map(|m| {
            let material_id = match &m.sk {
                EntityType::AiModeratorMaterial(id) => id.clone(),
                _ => String::new(),
            };
            MaterialResponse {
                material_id,
                file_name: m.file_name,
                file_url: m.file_url,
                created_at: m.created_at,
            }
        })
        .collect())
}
