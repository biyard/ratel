//! Errors for Launchpad callbacks, mapped to HTTP status codes mirroring
//! the reference brand-demo.

#[derive(Debug, Clone, thiserror::Error)]
pub enum PartnerError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("project mismatch")]
    ProjectMismatch,
    #[error("unknown user")]
    UnknownUser,
    #[error("invalid point amount")]
    InvalidAmount,
    #[error("insufficient points")]
    Insufficient,
    #[error("server error")]
    Server,
}

#[cfg(feature = "server")]
impl PartnerError {
    pub fn status(&self) -> u16 {
        match self {
            PartnerError::InvalidSignature => 401,
            PartnerError::ProjectMismatch => 403,
            PartnerError::UnknownUser => 404,
            PartnerError::InvalidAmount => 400,
            PartnerError::Insufficient => 409,
            PartnerError::Server => 500,
        }
    }
}
