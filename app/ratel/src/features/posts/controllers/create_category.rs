use crate::features::auth::User;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::Category;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[post("/api/categories", _user: User)]
pub async fn create_category_handler(req: CreateCategoryRequest) -> Result<CategoryResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    if req.name.trim().is_empty() {
        return Err(crate::features::posts::types::PostError::CategoryNameRequired.into());
    }

    Category::get_or_create_by_name(cli, req.name.trim().to_string())
        .await
        .map(CategoryResponse::from)
}