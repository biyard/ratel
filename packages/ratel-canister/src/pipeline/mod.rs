mod kmeans;
mod pca;
pub(crate) mod scaler;
mod silhouette;

use nalgebra::DMatrix;

#[cfg(feature = "perf")]
use crate::canister::perf::PerfTracker;
use crate::canister::storage;
use crate::error::SamplingError;
use crate::types::*;

pub fn run(input: SamplingInput) -> Result<SamplingResult, SamplingError> {
    let (matrix, ids) = build_matrix(&input)?;
    run_pipeline(&matrix, &ids, &input)
}

#[cfg(feature = "perf")]
pub fn run_with_metrics(
    input: SamplingInput,
) -> Result<(SamplingResult, PerfMetrics), SamplingError> {
    let features = if input.data.is_empty() {
        0
    } else {
        input.data[0].answers.len() as u32
    };
    let mut tracker = PerfTracker::new(
        input.data.len() as u32,
        features,
        input.min_k(),
        input.max_k(),
    );

    let (matrix, ids) = build_matrix(&input)?;

    let t0 = PerfTracker::start();
    let scaler_result = scaler::fit_transform(&matrix)?;
    tracker.record("scaler", t0);

    let t0 = PerfTracker::start();
    let pca_result = pca::fit_transform(&scaler_result.scaled, input.variance_threshold())?;
    tracker.record("pca", t0);

    let mut k_scores = Vec::new();
    let mut best_k = input.min_k();
    let mut best_sil = f64::NEG_INFINITY;
    let mut best_labels = Vec::new();
    let mut best_centroids = DMatrix::zeros(0, 0);

    for k in input.min_k()..=input.max_k() {
        let t0 = PerfTracker::start();
        let km_result = kmeans::fit(&pca_result.scores, k, input.max_iterations());
        tracker.record(&format!("kmeans_k{}, iter:{}", k, km_result.iterations), t0);

        let t0 = PerfTracker::start();
        let sil = silhouette::score(&pca_result.scores, &km_result.labels);
        tracker.record(&format!("silhouette_k{}", k), t0);

        k_scores.push(KScore {
            k,
            silhouette_score: sil,
            inertia: km_result.inertia,
        });

        if sil > best_sil {
            best_sil = sil;
            best_k = k;
            best_labels = km_result.labels;
            best_centroids = km_result.centroids;
        }
    }

    let result = build_result(
        &input.id,
        &matrix,
        &ids,
        best_k,
        &best_labels,
        &best_centroids,
        k_scores,
        &scaler_result,
        &pca_result,
    );

    let metrics = tracker.finish();
    Ok((result, metrics))
}

fn build_matrix(input: &SamplingInput) -> Result<(DMatrix<f64>, Vec<String>), SamplingError> {
    let n = input.data.len();
    if n == 0 {
        return Err(SamplingError::EmptyData);
    }

    let ncols = input.data[0].answers.len();
    for row in &input.data {
        if row.answers.len() != ncols {
            return Err(SamplingError::InconsistentFeatures {
                expected: ncols,
                got: row.answers.len(),
            });
        }
    }

    let flat: Vec<f64> = input
        .data
        .iter()
        .flat_map(|row| row.answers.clone())
        .collect();
    let ids: Vec<String> = input.data.iter().map(|r| r.id.clone()).collect();
    let matrix = DMatrix::from_row_slice(n, ncols, &flat);

    Ok((matrix, ids))
}

fn run_pipeline(
    matrix: &DMatrix<f64>,
    ids: &[String],
    input: &SamplingInput,
) -> Result<SamplingResult, SamplingError> {
    if input.min_k() < 1 {
        return Err(SamplingError::InvalidK(input.min_k()));
    }
    if (matrix.nrows() as u32) < input.max_k() {
        return Err(SamplingError::InsufficientData {
            n: matrix.nrows(),
            k: input.max_k(),
        });
    }

    let scaler_result = scaler::fit_transform(matrix)?;
    let pca_result = pca::fit_transform(&scaler_result.scaled, input.variance_threshold())?;

    let mut k_scores = Vec::new();
    let mut best_k = input.min_k();
    let mut best_sil = f64::NEG_INFINITY;
    let mut best_labels = Vec::new();
    let mut best_centroids = DMatrix::zeros(0, 0);

    for k in input.min_k()..=input.max_k() {
        let km_result = kmeans::fit(&pca_result.scores, k, input.max_iterations());
        let sil = silhouette::score(&pca_result.scores, &km_result.labels);

        k_scores.push(KScore {
            k,
            silhouette_score: sil,
            inertia: km_result.inertia,
        });

        if sil > best_sil {
            best_sil = sil;
            best_k = k;
            best_labels = km_result.labels;
            best_centroids = km_result.centroids;
        }
    }

    Ok(build_result(
        &input.id,
        matrix,
        ids,
        best_k,
        &best_labels,
        &best_centroids,
        k_scores,
        &scaler_result,
        &pca_result,
    ))
}

fn build_result(
    model_id: &str,
    original_data: &DMatrix<f64>,
    ids: &[String],
    best_k: u32,
    best_labels: &[u32],
    best_centroids: &DMatrix<f64>,
    k_scores: Vec<KScore>,
    scaler_result: &scaler::ScalerResult,
    pca_result: &pca::PcaResult,
) -> SamplingResult {
    let ncols = original_data.ncols();

    let assignments: Vec<Assignment> = ids
        .iter()
        .zip(best_labels.iter())
        .map(|(id, &cluster)| Assignment {
            id: id.clone(),
            cluster: cluster + 1,
        })
        .collect();

    let mut cluster_sums = vec![vec![0.0f64; ncols]; best_k as usize];
    let mut cluster_counts = vec![0u32; best_k as usize];

    for (i, &label) in best_labels.iter().enumerate() {
        let c = label as usize;
        cluster_counts[c] += 1;
        for j in 0..ncols {
            cluster_sums[c][j] += original_data[(i, j)];
        }
    }

    let cluster_profiles: Vec<ClusterProfile> = (0..best_k as usize)
        .map(|c| {
            let count = cluster_counts[c];
            let mean_values = if count > 0 {
                cluster_sums[c].iter().map(|&s| s / count as f64).collect()
            } else {
                vec![0.0; ncols]
            };
            ClusterProfile {
                cluster_id: (c + 1) as u32,
                count,
                mean_values,
            }
        })
        .collect();

    let model_params = ModelParams {
        scaler_means: scaler_result.means.clone(),
        scaler_stds: scaler_result.stds.clone(),
        pca_projection: pca_result.projection.as_slice().to_vec(),
        n_features: ncols as u32,
        n_components: pca_result.n_components as u32,
        centroids: best_centroids.as_slice().to_vec(),
        k: best_k,
        explained_variance_ratio: pca_result.explained_variance_ratio.clone(),
    };

    storage::save(model_id, model_params.clone());

    SamplingResult {
        optimal_k: best_k,
        silhouette_scores: k_scores,
        assignments,
        cluster_profiles,
        model_params,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        let data = vec![
            DataRow {
                id: "a1".into(),
                answers: vec![1.0, 1.0, 1.0, 1.0],
            },
            DataRow {
                id: "a2".into(),
                answers: vec![1.2, 1.1, 1.3, 1.0],
            },
            DataRow {
                id: "a3".into(),
                answers: vec![1.1, 0.9, 1.0, 1.2],
            },
            DataRow {
                id: "a4".into(),
                answers: vec![0.9, 1.0, 1.1, 0.8],
            },
            DataRow {
                id: "a5".into(),
                answers: vec![1.0, 1.2, 0.9, 1.1],
            },
            DataRow {
                id: "b1".into(),
                answers: vec![5.0, 5.0, 5.0, 5.0],
            },
            DataRow {
                id: "b2".into(),
                answers: vec![4.8, 5.1, 5.2, 4.9],
            },
            DataRow {
                id: "b3".into(),
                answers: vec![5.1, 4.9, 5.0, 5.2],
            },
            DataRow {
                id: "b4".into(),
                answers: vec![4.9, 5.0, 4.8, 5.1],
            },
            DataRow {
                id: "b5".into(),
                answers: vec![5.2, 5.1, 5.1, 4.8],
            },
        ];

        let input = SamplingInput {
            id: "test-model".into(),
            data,
            min_k: Some(2),
            max_k: Some(4),
            variance_threshold: Some(0.85),
            max_iterations: Some(100),
        };

        let result = run(input).unwrap();

        assert_eq!(result.optimal_k, 2, "Optimal K should be 2");

        let a_cluster = result
            .assignments
            .iter()
            .find(|a| a.id == "a1")
            .unwrap()
            .cluster;
        for id in &["a2", "a3", "a4", "a5"] {
            let c = result
                .assignments
                .iter()
                .find(|a| a.id == *id)
                .unwrap()
                .cluster;
            assert_eq!(c, a_cluster, "{} should be in same cluster as a1", id);
        }

        let b_cluster = result
            .assignments
            .iter()
            .find(|a| a.id == "b1")
            .unwrap()
            .cluster;
        assert_ne!(a_cluster, b_cluster);

        assert!(!result.model_params.scaler_means.is_empty());
        assert!(!result.model_params.centroids.is_empty());
        assert_eq!(result.model_params.k, 2);
        assert_eq!(result.model_params.n_features, 4);

        let loaded = storage::load("test-model");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().k, 2);
    }
}
