use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ListItemsResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> From<(Vec<T>, Option<String>)> for ListItemsResponse<T> {
    fn from((items, bookmark): (Vec<T>, Option<String>)) -> Self {
        Self { items, bookmark }
    }
}
