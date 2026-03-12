use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::Category;
use crate::*;

#[get("/api/categories?bookmark")]
pub async fn list_categories_handler(bookmark: Option<String>) -> Result<ListItemsResponse<CategoryResponse>> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let mut opt = Category::opt().limit(100);
    if let Some(b) = bookmark {
        opt = opt.bookmark(b);
    }

    let (items, bookmark) = Category::query(cli, Partition::Category, opt).await?;
    Ok(ListItemsResponse {
        items: items.into_iter().map(CategoryResponse::from).collect(),
        bookmark,
    })
}