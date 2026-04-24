//! Sub-team management page — 6 tabs:
//! requirements / form / documents / sub-teams / queue / broadcast.
//!
//! Each tab is a small sub-component that destructures its
//! `UseSubTeamXxx` controller hook and wires UI events straight to the
//! action fields. The page itself is responsible for resolving the
//! `username` route param into a `TeamPartition` context so the hooks
//! (which all `use_context::<TeamPartition>()`) can run.

mod broadcast_tab;
mod component;
mod docs_tab;
mod form_tab;
mod list_tab;
mod queue_tab;
mod requirements_tab;

pub use component::*;
use broadcast_tab::*;
use docs_tab::*;
use form_tab::*;
use list_tab::*;
use queue_tab::*;
use requirements_tab::*;
