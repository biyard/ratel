#[derive(Debug, thiserror::Error)]
pub enum VotingError {
    #[error("votes cannot be empty")]
    EmptyVotes,
    #[error("vote encode failed: {0}")]
    EncodeFailed(String),
    #[error("vote decode failed: {0}")]
    DecodeFailed(String),
}
