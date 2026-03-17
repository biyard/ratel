use serde::{Deserialize, Serialize};

use super::input::DataRow;
use super::output::{Assignment, PredictResult};
use crate::error::SamplingError;

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

            // 1. Scale: (x - mean) / std
            let scaled: Vec<f64> = row
                .answers
                .iter()
                .enumerate()
                .map(|(j, &x)| (x - self.scaler_means[j]) / self.scaler_stds[j])
                .collect();

            // 2. PCA project: scaled (1×nf) × projection (nf×nc) → (1×nc)
            let projected: Vec<f64> = (0..nc)
                .map(|c| {
                    (0..nf)
                        .map(|f| scaled[f] * self.pca_projection[f * nc + c])
                        .sum::<f64>()
                })
                .collect();

            // 3. Nearest centroid
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
