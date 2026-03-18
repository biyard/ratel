#[derive(Debug, thiserror::Error)]
pub enum SamplingError {
    #[error("empty data")]
    EmptyData,
    #[error("inconsistent feature count: expected {expected}, got {got}")]
    InconsistentFeatures { expected: usize, got: usize },
    #[error("k must be >= 1, got {0}")]
    InvalidK(u32),
    #[error("not enough data points ({n}) for k={k}")]
    InsufficientData { n: usize, k: u32 },
    #[error("model not found: {0}")]
    ModelNotFound(String),
    #[error("SVD computation failed")]
    SvdFailed,
    #[error("storage encode failed: {0}")]
    EncodeFailed(String),
    #[error("storage decode failed: {0}")]
    DecodeFailed(String),
}
