pub mod dashboard;

// Backward-compatible re-export for callers using the old path
pub mod dashboard_extension {
    pub use super::dashboard::dashboard_extension::*;
}
