//! Report detail (edit) page — turn-key context provider + sub-components.
//! Mirrors the mockup at `assets/design/reports/reports-edit.html` but
//! follows the codebase conventions: data lives in
//! `use_report_detail_context`, the body editor reuses the shared
//! `common::components::editor::Editor`, and all user-facing strings
//! flow through `ReportDetailTranslate`.

mod component;
mod data_picker;
mod doc_canvas;
mod edit_banner;
mod format_toolbar;
mod i18n;
mod outline;
mod slash_popup;
mod top_bar;

pub use component::*;
