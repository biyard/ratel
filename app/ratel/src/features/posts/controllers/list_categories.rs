use crate::features::posts::models::Category;
use crate::*;

#[get("/api/categories")]
pub async fn list_categories_handler() -> Result<Vec<Category>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    Category::find_all(cli).await
}