use crate::features::ai_moderator::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaterialResponse {
    pub material_id: String,
    pub file_name: String,
    pub file_url: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MaterialListResponse {
    pub items: Vec<MaterialResponse>,
}

#[get("/api/spaces/{space_id}/discussions/{discussion_id}/ai-moderator/materials", role: SpaceUserRole)]
pub async fn list_materials(
    space_id: SpacePartition,
    discussion_id: SpaceDiscussionEntityType,
) -> Result<MaterialListResponse> {
    role.is_creator()?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id, discussion_id.to_string());

    let opt = AiModeratorMaterial::opt()
        .sk(EntityType::AiModeratorMaterial(String::default()).to_string());
    let (materials, _) = AiModeratorMaterial::query(cli, &pk, opt).await?;

    let items = materials
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
        .collect();

    Ok(MaterialListResponse { items })
}
