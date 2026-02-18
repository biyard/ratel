// Utility functions have been migrated to common::utils
// Re-export for backwards compatibility
#[cfg(feature = "server")]
pub mod password {
    pub use common::utils::password::*;
}
#[cfg(feature = "server")]
pub mod telegram;
