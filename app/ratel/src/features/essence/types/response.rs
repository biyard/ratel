use super::source_kind::EssenceSourceKind;
use crate::*;
use serde::{Deserialize, Serialize};

/// Wire-format row in `GET /api/essences`. All fields the client needs to
/// render a sources-table entry — no quality score and no in-house toggle,
/// those columns were dropped from the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct EssenceResponse {
    /// Deterministic id derived from the referenced entity's pk+sk. Stable
    /// across reruns so upsert semantics work without a GSI lookup.
    pub id: String,
    pub source_kind: EssenceSourceKind,
    pub title: String,
    /// Human-readable breadcrumb shown under the title
    /// (e.g. `Ratel post · /p/abc`, `Ratel comment · /p/foo#c42`).
    pub source_path: String,
    /// Raw pk of the referenced entity. Handed back to the client so it can
    /// build links without re-parsing partition prefixes.
    pub source_pk: String,
    pub source_sk: String,
    /// `i64` to match the server model (whose GSI2 sort key needs i64); the
    /// client still treats this as a plain non-negative number.
    pub word_count: i64,
    /// Unix seconds. Client formats as "2m ago" / "yesterday".
    pub updated_at: i64,
}

/// Aggregate counts for the hero card. Backed by the `UserEssenceStats`
/// singleton row so a single roundtrip returns accurate totals regardless
/// of how many Essence rows the user has.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct EssenceStatsResponse {
    pub total_sources: i64,
    pub total_words: i64,
}
