use crate::common::traits::{Bookmarker, ItemIter};
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Debug)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct ListResponse<T>
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> From<(Vec<T>, Option<String>)> for ListResponse<T>
where
    T: Clone + serde::de::DeserializeOwned + serde::Serialize,
{
    fn from((items, bookmark): (Vec<T>, Option<String>)) -> Self {
        Self { items, bookmark }
    }
}

impl<T> Bookmarker<String> for ListResponse<T>
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    fn bookmark(&self) -> Option<String> {
        self.bookmark.clone()
    }
}

impl<T> ItemIter<T> for ListResponse<T>
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    fn items(&self) -> &'_ Vec<T> {
        &self.items
    }
}
