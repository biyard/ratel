//! Admin pages for *Fact or Fold*. All pages live under
//! `/admin/fact-or-fold/*` and share `FactFoldAdminLayout` (sub-tabs
//! + arena chrome) on top of the global `AdminLayout` (admin guard).

pub mod subjects;
pub mod i18n;
pub mod layout;
pub mod new_subject;
pub mod reports;
pub mod schedule;
pub mod settings;
pub mod stats;

pub use subjects::*;
pub use i18n::*;
pub use layout::*;
pub use new_subject::*;
pub use reports::*;
pub use schedule::*;
pub use settings::*;
pub use stats::*;
