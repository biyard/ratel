use serde::{Deserialize, Serialize};

use crate::AdditionalResource;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PatronQuery {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
}

impl std::fmt::Display for PatronQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = serde_urlencoded::to_string(&self).unwrap();

        write!(f, "{query}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureStatus {
    Todo,
    Done,
    InProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatronSummary {
    pub id: String,

    pub nickname: String,
    pub profile_url: String,
    pub wallet_address: String,

    pub feature_title: Option<String>,
    pub feature_status: Option<FeatureStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Ethereum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatronActionRequest {
    Support {
        agree: bool,
        transaction_hash: String,
        network: Network,
        features: Vec<FeatureRequest>,
    },
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub enum PatronActionResponse {
    #[default]
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRequest {
    pub title: String,
    pub description: String,
    pub reference: String,
    pub attaches: Vec<AdditionalResource>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Patron {
    pub id: String,

    pub nickname: String,
    pub profile_url: String,
    pub wallet_address: String,

    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub title: String,
    pub description: String,
    pub reference: String,
    pub attaches: Vec<AdditionalResource>,
    pub status: FeatureStatus,
}
