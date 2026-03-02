use crate::*;

#[get("/api/spaces/{space_id}/discussions/categories", role: SpaceUserRole)]
pub async fn list_categories(space_id: SpacePartition) -> Result<Vec<String>> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let opt = SpaceCategory::opt_all().sk(EntityType::SpaceCategory(String::default()).to_string());
    let (categories, _) = SpaceCategory::query(cli, space_pk, opt).await?;

    let names: Vec<String> = categories.into_iter().map(|c| c.name).collect();

    Ok(names)
}
