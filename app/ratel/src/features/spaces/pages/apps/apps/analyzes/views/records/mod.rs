//! "사용된 데이터 확인하기" — raw records page.
//!
//! Re-uses the same arena chrome as the report DETAIL view (back
//! button, breadcrumb) but renders only the chip strip + a per-source
//! table for whichever filter the user clicked. Records are loaded
//! lazily via `UseAnalyzeRecords` against the persisted, frozen
//! `matched_records` field on the report row.
//!
//! Filter chip layout intentionally mirrors `report/banner` so the
//! visual handoff from the detail page reads as continuous.

mod component;
pub use component::*;
