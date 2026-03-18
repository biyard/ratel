use serde::{Deserialize, Serialize};

use super::model::ModelParams;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct KScore {
    pub k: u32,
    pub silhouette_score: f64,
    pub inertia: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct Assignment {
    pub id: String,
    pub cluster: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct ClusterProfile {
    pub cluster_id: u32,
    pub count: u32,
    pub mean_values: Vec<f64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct SamplingResult {
    pub optimal_k: u32,
    pub silhouette_scores: Vec<KScore>,
    pub assignments: Vec<Assignment>,
    pub cluster_profiles: Vec<ClusterProfile>,
    pub model_params: ModelParams,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct PredictResult {
    pub assignments: Vec<Assignment>,
}
