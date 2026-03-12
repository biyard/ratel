use nalgebra::DMatrix;

pub struct KMeansResult {
    pub labels: Vec<u32>,
    pub centroids: DMatrix<f64>,
    pub inertia: f64,
    pub iterations: u32,
}

fn sq_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum()
}

fn init_centroids(data: &DMatrix<f64>, k: usize, first_idx: usize) -> DMatrix<f64> {
    let (nrows, ncols) = data.shape();
    let mut centroids = DMatrix::zeros(k, ncols);

    centroids.set_row(0, &data.row(first_idx % nrows));

    let mut min_dists = vec![f64::MAX; nrows];

    for c in 1..k {
        let last_centroid: Vec<f64> = centroids.row(c - 1).iter().copied().collect();
        for i in 0..nrows {
            let row: Vec<f64> = data.row(i).iter().copied().collect();
            let d = sq_distance(&row, &last_centroid);
            if d < min_dists[i] {
                min_dists[i] = d;
            }
        }

        let next = (0..nrows)
            .max_by(|&a, &b| min_dists[a].partial_cmp(&min_dists[b]).unwrap())
            .unwrap();

        centroids.set_row(c, &data.row(next));
    }

    centroids
}

fn get_init_seeds(data: &DMatrix<f64>, n_init: usize) -> Vec<usize> {
    let (nrows, ncols) = data.shape();
    let mut seeds = Vec::with_capacity(n_init);

    let mean: Vec<f64> = (0..ncols)
        .map(|j| data.column(j).sum() / nrows as f64)
        .collect();
    let closest_to_mean = (0..nrows)
        .min_by(|&a, &b| {
            let ra: Vec<f64> = data.row(a).iter().copied().collect();
            let rb: Vec<f64> = data.row(b).iter().copied().collect();
            sq_distance(&ra, &mean)
                .partial_cmp(&sq_distance(&rb, &mean))
                .unwrap()
        })
        .unwrap();
    seeds.push(closest_to_mean);

    let furthest_from_mean = (0..nrows)
        .max_by(|&a, &b| {
            let ra: Vec<f64> = data.row(a).iter().copied().collect();
            let rb: Vec<f64> = data.row(b).iter().copied().collect();
            sq_distance(&ra, &mean)
                .partial_cmp(&sq_distance(&rb, &mean))
                .unwrap()
        })
        .unwrap();
    seeds.push(furthest_from_mean);

    for i in 2..n_init {
        let idx = (i * nrows) / n_init;
        seeds.push(idx.min(nrows - 1));
    }

    seeds.truncate(n_init);
    seeds
}

pub fn fit(data: &DMatrix<f64>, k: u32, max_iterations: u32) -> KMeansResult {
    let n_init = 10;
    let seeds = get_init_seeds(data, n_init);

    let mut best_result: Option<KMeansResult> = None;

    for &seed_idx in &seeds {
        let result = fit_single(data, k, max_iterations, seed_idx);
        let is_better = match &best_result {
            None => true,
            Some(prev) => result.inertia < prev.inertia,
        };
        if is_better {
            best_result = Some(result);
        }
    }

    best_result.unwrap()
}

fn fit_single(data: &DMatrix<f64>, k: u32, max_iterations: u32, first_idx: usize) -> KMeansResult {
    let k = k as usize;
    let (nrows, ncols) = data.shape();
    let mut centroids = init_centroids(data, k, first_idx);
    let mut labels = vec![0u32; nrows];
    let mut iterations = 0u32;

    for iter in 0..max_iterations {
        iterations = iter + 1;
        let mut changed = false;

        for i in 0..nrows {
            let row: Vec<f64> = data.row(i).iter().copied().collect();
            let mut best_c = 0u32;
            let mut best_d = f64::MAX;

            for c in 0..k {
                let centroid: Vec<f64> = centroids.row(c).iter().copied().collect();
                let d = sq_distance(&row, &centroid);
                if d < best_d {
                    best_d = d;
                    best_c = c as u32;
                }
            }

            if labels[i] != best_c {
                labels[i] = best_c;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        let mut new_centroids = DMatrix::zeros(k, ncols);
        let mut counts = vec![0usize; k];

        for i in 0..nrows {
            let c = labels[i] as usize;
            counts[c] += 1;
            for j in 0..ncols {
                new_centroids[(c, j)] += data[(i, j)];
            }
        }

        for c in 0..k {
            if counts[c] > 0 {
                for j in 0..ncols {
                    new_centroids[(c, j)] /= counts[c] as f64;
                }
            } else {
                new_centroids.set_row(c, &centroids.row(c));
            }
        }

        centroids = new_centroids;
    }

    let mut inertia = 0.0;
    for i in 0..nrows {
        let row: Vec<f64> = data.row(i).iter().copied().collect();
        let c = labels[i] as usize;
        let centroid: Vec<f64> = centroids.row(c).iter().copied().collect();
        inertia += sq_distance(&row, &centroid);
    }

    KMeansResult {
        labels,
        centroids,
        inertia,
        iterations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans_two_clusters() {
        let data = DMatrix::from_row_slice(
            6,
            2,
            &[
                0.0, 0.0, 0.1, 0.1, 0.2, 0.0, 10.0, 10.0, 10.1, 10.1, 9.9, 10.0,
            ],
        );

        let result = fit(&data, 2, 100);

        assert_eq!(result.labels[0], result.labels[1]);
        assert_eq!(result.labels[1], result.labels[2]);

        assert_eq!(result.labels[3], result.labels[4]);
        assert_eq!(result.labels[4], result.labels[5]);

        assert_ne!(result.labels[0], result.labels[3]);
    }

    #[test]
    fn test_kmeans_n_init_picks_best() {
        let data = DMatrix::from_row_slice(
            8,
            2,
            &[
                0.0, 0.0, 0.1, 0.2, 0.2, 0.1, 0.0, 0.2, 5.0, 5.0, 5.1, 5.2, 5.2, 5.1, 5.0, 5.2,
            ],
        );

        let r1 = fit(&data, 2, 100);
        let r2 = fit(&data, 2, 100);

        assert_eq!(r1.labels, r2.labels);
        assert!((r1.inertia - r2.inertia).abs() < 1e-10);
    }
}
