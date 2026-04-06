pub mod material_indexer;
pub mod post_indexer;
pub mod reply_indexer;

pub use material_indexer::*;
pub use post_indexer::*;
pub use reply_indexer::*;

/// Tenant ID derived from the table prefix.
pub fn tenant_id() -> String {
    let cfg = crate::common::CommonConfig::default();
    cfg.qdrant.prefix.to_string()
}
