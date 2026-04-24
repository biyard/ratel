//! Placeholder page modules for the sub-team governance feature.
//!
//! Each sub-module owns a single top-level `#[component]` that renders a
//! "page under construction" placeholder. Follow-up dispatches will flesh
//! these out by consuming the controller hooks in
//! `features::sub_team::hooks` and mounting the HTML-first design
//! mockups from `app/ratel/assets/design/sub-team/`.

pub mod application_status;
pub mod apply;
pub mod broadcast_compose;
pub mod bylaws;
pub mod deregister;
pub mod detail;
pub mod doc_compose;
pub mod leave_parent;
pub mod management;

pub use application_status::*;
pub use apply::*;
pub use broadcast_compose::*;
pub use bylaws::*;
pub use deregister::*;
pub use detail::*;
pub use doc_compose::*;
pub use leave_parent::*;
pub use management::*;
