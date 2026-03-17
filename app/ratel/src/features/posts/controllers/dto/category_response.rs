use crate::features::posts::models::Category;
use crate::features::posts::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CategoryResponse {
    pub name: String,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        Self {
            name: category.name,
        }
    }
}