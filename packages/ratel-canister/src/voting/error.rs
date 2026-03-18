#[derive(Debug, thiserror::Error)]
pub enum VotingError {
    #[error("votes cannot be empty")]
    EmptyVotes,
    #[error("duplicate vote: voter '{0}' has already submitted")]
    DuplicateVoter(String),
    #[error("voter not found: '{0}'")]
    VoterNotFound(String),
    #[error("vote encode failed: {0}")]
    EncodeFailed(String),
    #[error("vote decode failed: {0}")]
    DecodeFailed(String),
}
