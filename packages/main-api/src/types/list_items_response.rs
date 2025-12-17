use bdk::prelude::*;

pub type ListResponse<T> = ListItemsResponse<T>;

#[derive(
    Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, schemars::JsonSchema,
)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
#[schemars(bound = "T: JsonSchema")]
pub struct ListItemsResponse<T>
where
    T: Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + aide::OperationInput
        + aide::OperationOutput
        + schemars::JsonSchema,
{
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> From<(Vec<T>, Option<String>)> for ListItemsResponse<T>
where
    T: Clone
        + serde::de::DeserializeOwned
        + serde::Serialize
        + aide::OperationInput
        + aide::OperationOutput
        + JsonSchema,
{
    fn from((items, bookmark): (Vec<T>, Option<String>)) -> Self {
        Self { items, bookmark }
    }
}
