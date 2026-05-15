//! Admin pages for *Fact or Fold*. All pages live under
//! `/admin/fact-or-fold/*` and share `FactFoldAdminLayout` (sub-tabs
//! + arena chrome) on top of the global `AdminLayout` (admin guard).

pub mod headlines;
pub mod i18n;
pub mod layout;
pub mod new_headline;
pub mod reports;
pub mod schedule;
pub mod settings;
pub mod stats;

pub use headlines::*;
pub use i18n::*;
pub use layout::*;
pub use new_headline::*;
pub use reports::*;
pub use schedule::*;
pub use settings::*;
pub use stats::*;
