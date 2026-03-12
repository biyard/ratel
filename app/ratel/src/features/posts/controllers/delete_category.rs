use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::Category;
use crate::*;

#[delete("/api/categories/:name")]
pub async fn delete_category_handler(name: String) -> Result<CategoryResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    Category::delete(cli, Partition::Category, Some(EntityType::Category(name)))
        .await
        .map(CategoryResponse::from)
}