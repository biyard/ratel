use by_macros::QueryDisplay;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

use crate::AdditionalResource;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, QueryDisplay)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PatronQuery {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum FeatureStatus {
    Todo,
    Done,
    InProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PatronSummary {
    pub id: String,

    pub nickname: String,
    pub profile_url: String,
    pub wallet_address: String,

    pub feature_title: Option<String>,
    pub feature_status: Option<FeatureStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Ethereum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum PatronActionRequest {
    Support {
        agree: bool,
        transaction_hash: String,
        network: Network,
        features: Vec<FeatureRequest>,
    },
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum PatronActionResponse {
    #[default]
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct FeatureRequest {
    pub title: String,
    pub description: String,
    pub reference: String,
    pub attaches: Vec<AdditionalResource>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Patron {
    pub id: String,

    pub nickname: String,
    pub profile_url: String,
    pub wallet_address: String,

    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Feature {
    pub title: String,
    pub description: String,
    pub reference: String,
    pub attaches: Vec<AdditionalResource>,
    pub status: FeatureStatus,
}
