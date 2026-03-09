// Utility functions have been migrated to crate::common::utils
// Re-export for backwards compatibility
#[cfg(feature = "server")]
pub mod password {
    pub use crate::common::utils::password::*;
}

#[cfg(feature = "server")]
pub mod evm;
#[cfg(feature = "server")]
pub mod rand_utils;
#[cfg(feature = "server")]
pub mod referral_code;
#[cfg(feature = "server")]
pub mod sha256_baseurl;
#[cfg(feature = "server")]
pub mod telegram;
#[cfg(feature = "server")]
pub mod uuid;
#[cfg(feature = "server")]
pub mod validator;

#[cfg(feature = "server")]
pub use rand_utils::*;
