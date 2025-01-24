use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CommonQueryResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> Default for CommonQueryResponse<T> {
    fn default() -> Self {
        CommonQueryResponse {
            items: Vec::<T>::new(),
            bookmark: None,
        }
    }
}

#[cfg(feature = "server")]
impl<T> CommonQueryResponse<T>
where
    T: std::fmt::Debug + serde::de::DeserializeOwned,
{
    pub async fn query<F>(
        log: &slog::Logger,
        index: &str,
        bookmark: Option<String>,
        size: Option<i32>,
        filter: Vec<(&str, F)>,
    ) -> Result<Self, crate::error::ServiceError>
    where
        F: std::fmt::Debug + serde::Serialize,
    {
        let cli = easy_dynamodb::get_client(log);

        match cli
            .find(index, bookmark, Some(size.unwrap_or(100)), filter)
            .await
        {
            Ok((Some(items), bookmark)) => Ok(CommonQueryResponse { items, bookmark }),
            Ok((None, bookmark)) => Ok(CommonQueryResponse {
                items: vec![],
                bookmark,
            }),
            Err(e) => {
                return Err(crate::error::ServiceError::Unknown(format!("{:?}", e)));
            }
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct QueryResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
}

impl<T> From<(Vec<T>, i64)> for QueryResponse<T> {
    fn from((items, total_count): (Vec<T>, i64)) -> Self {
        QueryResponse { items, total_count }
    }
}
