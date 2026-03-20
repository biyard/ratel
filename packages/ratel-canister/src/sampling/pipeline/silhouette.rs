use nalgebra::DMatrix;

const SAMPLE_THRESHOLD: usize = 500;

pub fn score(data: &DMatrix<f64>, labels: &[u32]) -> f64 {
    let n = data.nrows();
    if n <= 1 {
        return 0.0;
    }

    let max_label = *labels.iter().max().unwrap() as usize;
    let k = max_label + 1;

    if k <= 1 {
        return 0.0;
    }

    let mut cluster_members: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (i, &l) in labels.iter().enumerate() {
        cluster_members[l as usize].push(i);
    }

    let sample_indices: Vec<usize> = if n <= SAMPLE_THRESHOLD {
        (0..n).collect()
    } else {
        deterministic_sample(n, SAMPLE_THRESHOLD)
    };

    let sample_n = sample_indices.len();
    let mut total_sil = 0.0;

    for &i in &sample_indices {
        let ci = labels[i] as usize;
        let row_i: Vec<f64> = data.row(i).iter().copied().collect();

        let a = if cluster_members[ci].len() > 1 {
            let sum: f64 = cluster_members[ci]
                .iter()
                .filter(|&&j| j != i)
                .map(|&j| {
                    let row_j: Vec<f64> = data.row(j).iter().copied().collect();
                    euclidean_distance(&row_i, &row_j)
                })
                .sum();
            sum / (cluster_members[ci].len() - 1) as f64
        } else {
            0.0
        };

        let mut b = f64::MAX;
        for c in 0..k {
            if c == ci || cluster_members[c].is_empty() {
                continue;
            }
            let mean_dist: f64 = cluster_members[c]
                .iter()
                .map(|&j| {
                    let row_j: Vec<f64> = data.row(j).iter().copied().collect();
                    euclidean_distance(&row_i, &row_j)
                })
                .sum::<f64>()
                / cluster_members[c].len() as f64;

            if mean_dist < b {
                b = mean_dist;
            }
        }

        if b == f64::MAX {
            b = 0.0;
        }

        let max_ab = a.max(b);
        let s = if max_ab > 1e-12 {
            (b - a) / max_ab
        } else {
            0.0
        };

        total_sil += s;
    }

    total_sil / sample_n as f64
}

fn deterministic_sample(n: usize, sample_size: usize) -> Vec<usize> {
    let step = n as f64 / sample_size as f64;
    (0..sample_size)
        .map(|i| ((i as f64 * step) as usize).min(n - 1))
        .collect()
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silhouette_perfect_clusters() {
        let data = DMatrix::from_row_slice(
            6,
            2,
            &[
                0.0, 0.0, 0.1, 0.1, 0.2, 0.0, 10.0, 10.0, 10.1, 10.1, 9.9, 10.0,
            ],
        );
        let labels = vec![0, 0, 0, 1, 1, 1];

        let s = score(&data, &labels);
        assert!(
            s > 0.9,
            "Silhouette should be high for well-separated clusters, got {}",
            s
        );
    }

    #[test]
    fn test_silhouette_single_cluster() {
        let data = DMatrix::from_row_slice(3, 2, &[0.0, 0.0, 1.0, 1.0, 2.0, 2.0]);
        let labels = vec![0, 0, 0];

        let s = score(&data, &labels);
        assert!(
            s.abs() < 1e-10,
            "Single cluster silhouette should be 0, got {}",
            s
        );
    }

    #[test]
    fn test_deterministic_sample() {
        let indices = deterministic_sample(5000, 1000);
        assert_eq!(indices.len(), 1000);
        assert_eq!(indices[0], 0);
        assert!(indices.windows(2).all(|w| w[0] < w[1]));
    }
}
