//! Mock report list item types for the in-progress local-only Report
//! app. Real persistence will replace these once the backend schema is
//! ready; the shapes here mirror what the HTML mockup at
//! `assets/design/reports/reports-list.html` renders.

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportStatus {
    Draft,
    Published,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportSourceKind {
    /// Action · Report — generic action-derived report.
    Action,
    /// Follow · Personas — follower segmentation analysis.
    Follow,
    /// Quiz · Pass Rate — quiz pass-rate aggregate.
    Quiz,
    /// Poll · Climate — poll-derived report.
    Poll,
}

/// One report card on the carousel list page. Fields map 1:1 to the
/// HTML mockup so the RSX port can stay close to the source markup.
/// `Serialize`/`Deserialize` are required by `use_loader` so the SSR
/// hydration cache can round-trip the mock list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportListItem {
    pub id: String,
    pub status: ReportStatus,
    pub source: ReportSourceKind,
    /// Eyebrow label above the title (e.g. "Action · Report").
    pub category: String,
    pub title: String,
    pub description: String,
    /// Relative-time label rendered in the footer (e.g. "2d ago", "방금").
    pub relative_time: String,
}
