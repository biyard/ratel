use serde::{Deserialize, Serialize};

use super::error::SamplingError;

// ── Input ──

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct DataRow {
    pub id: String,
    pub answers: Vec<f64>,
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

// ── Output ──

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

// ── Model ──

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct ModelParams {
    pub scaler_means: Vec<f64>,
    pub scaler_stds: Vec<f64>,
    pub pca_projection: Vec<f64>,
    pub n_features: u32,
    pub n_components: u32,
    pub centroids: Vec<f64>,
    pub k: u32,
    pub explained_variance_ratio: Vec<f64>,
}

impl ModelParams {
    pub fn predict(&self, data: &[DataRow]) -> Result<PredictResult, SamplingError> {
        if data.is_empty() {
            return Err(SamplingError::EmptyData);
        }
        let nf = self.n_features as usize;
        let nc = self.n_components as usize;
        let k = self.k as usize;

        let mut assignments = Vec::with_capacity(data.len());
        for row in data {
            if row.answers.len() != nf {
                return Err(SamplingError::InconsistentFeatures {
                    expected: nf,
                    got: row.answers.len(),
                });
            }

            let scaled: Vec<f64> = row
                .answers
                .iter()
                .enumerate()
                .map(|(j, &x)| (x - self.scaler_means[j]) / self.scaler_stds[j])
                .collect();

            let projected: Vec<f64> = (0..nc)
                .map(|c| {
                    (0..nf)
                        .map(|f| scaled[f] * self.pca_projection[f * nc + c])
                        .sum::<f64>()
                })
                .collect();

            let mut best_cluster = 0u32;
            let mut best_dist = f64::MAX;
            for c in 0..k {
                let dist: f64 = (0..nc)
                    .map(|j| (projected[j] - self.centroids[c * nc + j]).powi(2))
                    .sum();
                if dist < best_dist {
                    best_dist = dist;
                    best_cluster = c as u32;
                }
            }

            assignments.push(Assignment {
                id: row.id.clone(),
                cluster: best_cluster + 1,
            });
        }

        Ok(PredictResult { assignments })
    }
}

// ── Perf ──

#[cfg(feature = "perf")]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct StepInstructions {
    pub step: String,
    pub instructions: u64,
}

#[cfg(feature = "perf")]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct PerfMetrics {
    pub instructions_used: u64,
    pub heap_memory_bytes: u64,
    pub cycles_balance_before: u64,
    pub cycles_balance_after: u64,
    pub cycles_consumed: u64,
    pub data_rows: u32,
    pub features: u32,
    pub k_range_tested: String,
    pub step_instructions: Vec<StepInstructions>,
}

#[cfg(feature = "perf")]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct SamplingWithMetrics {
    pub result: SamplingResult,
    pub metrics: PerfMetrics,
}
