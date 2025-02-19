#![allow(unused_variables, unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
use dioxus_translate::Translate;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AdditionalResource;

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum FeatureStatus {
    #[default]
    Todo = 0,
    Done = 1,
    InProgress = 2,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Network {
    #[default]
    #[serde(rename = "ETH")]
    Ethereum = 0,
}

#[api_model(base = "/v1/patron", table = patron, iter_type=QueryResponse)]
pub struct Patron {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary)]
    pub wallet_address: String,
    #[api_model(summary)]
    pub amount: i64,
    #[api_model(summary)]
    pub network: Network,
    #[api_model(summary, one_to_many = patron_feature)]
    pub features: Vec<Feature>,
}

#[api_model(base = "/v1/patron/:patron-id/feature", table = patron_feature, iter_type=QueryResponse)]
pub struct Feature {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, many_to_one = patron)]
    pub patron_id: i64,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, action = create)]
    pub reference: String,
    #[api_model(summary, action = create)]
    pub description: String,
    #[api_model(action = create, type = JSONB)]
    pub attaches: Vec<AdditionalResource>,
    #[api_model(summary, type = INTEGER, action = create, queryable)]
    pub status: FeatureStatus,
}
