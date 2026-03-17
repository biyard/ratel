use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct DataRow {
    pub id: String,
    pub answers: Vec<f64>, //USER ID
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct SamplingInput {
    pub id: String,
    pub data: Vec<DataRow>,
    pub min_k: Option<u32>,
    pub max_k: Option<u32>,
    pub variance_threshold: Option<f64>,
    pub max_iterations: Option<u32>,
}

impl SamplingInput {
    pub fn min_k(&self) -> u32 {
        self.min_k.unwrap_or(4)
    }
    pub fn max_k(&self) -> u32 {
        self.max_k.unwrap_or(9)
    }
    pub fn variance_threshold(&self) -> f64 {
        self.variance_threshold.unwrap_or(0.85)
    }
    pub fn max_iterations(&self) -> u32 {
        self.max_iterations.unwrap_or(300)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct PredictInput {
    pub model_id: String,
    pub data: Vec<DataRow>,
}
