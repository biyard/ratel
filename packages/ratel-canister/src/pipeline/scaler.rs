use nalgebra::DMatrix;

use crate::error::SamplingError;

pub struct ScalerResult {
    pub scaled: DMatrix<f64>,
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
}

pub fn fit_transform(data: &DMatrix<f64>) -> Result<ScalerResult, SamplingError> {
    let (nrows, ncols) = data.shape();
    if nrows == 0 {
        return Err(SamplingError::EmptyData);
    }
    let n = nrows as f64;

    let mut means = Vec::with_capacity(ncols);
    let mut stds = Vec::with_capacity(ncols);

    for j in 0..ncols {
        let col = data.column(j);
        let mean = col.sum() / n;
        let variance = col.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
        let std = variance.sqrt();

        means.push(mean);
        stds.push(if std < 1e-12 { 1.0 } else { std });
    }

    let mut scaled = data.clone();
    for j in 0..ncols {
        for i in 0..nrows {
            scaled[(i, j)] = (data[(i, j)] - means[j]) / stds[j];
        }
    }

    Ok(ScalerResult {
        scaled,
        means,
        stds,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_scaler() {
        let data = DMatrix::from_row_slice(3, 2, &[1.0, 10.0, 2.0, 20.0, 3.0, 30.0]);

        let result = fit_transform(&data).unwrap();

        for j in 0..2 {
            let col_mean: f64 = result.scaled.column(j).iter().sum::<f64>() / 3.0;
            assert!(
                col_mean.abs() < 1e-10,
                "Mean should be ~0, got {}",
                col_mean
            );
        }

        for j in 0..2 {
            let col = result.scaled.column(j);
            let mean = col.sum() / 3.0;
            let var: f64 = col.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / 3.0;
            let std = var.sqrt();
            assert!((std - 1.0).abs() < 1e-10, "Std should be ~1, got {}", std);
        }
    }
}
