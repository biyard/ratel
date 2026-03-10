use crate::features::posts::models::Category;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[post("/api/categories")]
pub async fn create_category_handler(req: CreateCategoryRequest) -> Result<Category> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    if req.name.trim().is_empty() {
        return Err(crate::Error::BadRequest(
            "Category name cannot be empty".into(),
        ));
    }

    Category::upsert_by_name(cli, req.name.trim().to_string()).await
}