use nalgebra::DMatrix;

use crate::error::SamplingError;

pub struct PcaResult {
    pub scores: DMatrix<f64>,
    pub n_components: usize,
    pub explained_variance_ratio: Vec<f64>,
    pub projection: DMatrix<f64>,
}

pub fn fit_transform(
    scaled: &DMatrix<f64>,
    variance_threshold: f64,
) -> Result<PcaResult, SamplingError> {
    let (nrows, ncols) = scaled.shape();
    if nrows == 0 {
        return Err(SamplingError::EmptyData);
    }
    let n_minus_1 = (nrows - 1).max(1) as f64;

    let svd = scaled.clone().svd(true, true);
    let singular_values = &svd.singular_values;

    let explained_variance: Vec<f64> = singular_values.iter().map(|&s| s * s / n_minus_1).collect();

    let total_variance: f64 = explained_variance.iter().sum();
    let explained_variance_ratio: Vec<f64> = if total_variance > 1e-12 {
        explained_variance
            .iter()
            .map(|&v| v / total_variance)
            .collect()
    } else {
        vec![1.0 / ncols as f64; ncols]
    };

    let mut cumsum = 0.0;
    let mut n_components = ncols;
    for (i, &ratio) in explained_variance_ratio.iter().enumerate() {
        cumsum += ratio;
        if cumsum >= variance_threshold {
            n_components = i + 1;
            break;
        }
    }
    n_components = n_components.max(1);

    let v_t = svd.v_t.as_ref().ok_or(SamplingError::SvdFailed)?;

    let mut signs = vec![1.0f64; n_components];
    for comp in 0..n_components {
        let row = v_t.row(comp);
        let mut max_abs = 0.0f64;
        let mut max_sign = 1.0f64;
        for &val in row.iter() {
            if val.abs() > max_abs {
                max_abs = val.abs();
                max_sign = if val >= 0.0 { 1.0 } else { -1.0 };
            }
        }
        signs[comp] = max_sign;
    }

    let mut projection = DMatrix::zeros(ncols, n_components);
    for comp in 0..n_components {
        for j in 0..ncols {
            projection[(j, comp)] = v_t[(comp, j)] * signs[comp];
        }
    }

    let scores = scaled * &projection;

    Ok(PcaResult {
        scores,
        n_components,
        explained_variance_ratio: explained_variance_ratio[..n_components].to_vec(),
        projection,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pca_reduces_dimensions() {
        let data = DMatrix::from_row_slice(
            5,
            4,
            &[
                -1.5, -1.4, 0.1, 0.2, -0.5, -0.6, 0.8, 0.9, 0.0, 0.1, 0.0, 0.1, 0.5, 0.4, -0.8,
                -0.7, 1.5, 1.5, -0.1, -0.5,
            ],
        );

        let result = fit_transform(&data, 0.85).unwrap();

        assert!(result.n_components <= 4);
        assert!(result.n_components >= 1);
        assert_eq!(result.scores.nrows(), 5);
        assert_eq!(result.scores.ncols(), result.n_components);

        let cumvar: f64 = result.explained_variance_ratio.iter().sum();
        assert!(
            cumvar >= 0.85 || result.n_components == 4,
            "Cumulative variance {:.4} should reach threshold",
            cumvar
        );
    }

    #[test]
    fn test_pca_sign_flip_determinism() {
        let data = DMatrix::from_row_slice(
            4,
            3,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        );

        let r1 = fit_transform(&data, 0.85).unwrap();
        let r2 = fit_transform(&data, 0.85).unwrap();

        assert_eq!(r1.n_components, r2.n_components);
        for i in 0..r1.scores.nrows() {
            for j in 0..r1.scores.ncols() {
                assert!(
                    (r1.scores[(i, j)] - r2.scores[(i, j)]).abs() < 1e-10,
                    "Scores should be identical across runs"
                );
            }
        }
    }
}
