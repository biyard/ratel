pub mod controllers;
pub mod i18n;
#[cfg(feature = "server")]
pub mod models;
pub mod types;

// PR1 ships only backend scaffold + admin headline CRUD. UI scaffolding
// (hooks/components/pages) is added in PR2+, so those submodules stay
// empty until then.

pub use controllers::*;
pub use i18n::*;
#[cfg(feature = "server")]
pub use models::*;
pub use types::*;
