#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
#[cfg_attr(feature = "server", schemars(bound = "T: schemars::JsonSchema"))]
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
