use crate::*;

/// The kind of a source feeding the Essence House. Used both as a filter
/// pill in the breakdown grid and as the per-row icon/badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EssenceSourceKind {
    #[default]
    Notion,
    RatelPost,
    Comment,
    Action,
}

/// Filter selection on the breakdown cards above the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KindFilter {
    #[default]
    All,
    Notion,
    RatelPost,
    Comment,
    Action,
}

impl KindFilter {
    pub fn matches(&self, kind: EssenceSourceKind) -> bool {
        match self {
            Self::All => true,
            Self::Notion => kind == EssenceSourceKind::Notion,
            Self::RatelPost => kind == EssenceSourceKind::RatelPost,
            Self::Comment => kind == EssenceSourceKind::Comment,
            Self::Action => kind == EssenceSourceKind::Action,
        }
    }
}

/// Active/paused/AI-flagged filter chip selection in the controls bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusFilter {
    #[default]
    All,
    Active,
    Paused,
    AiFlagged,
}

/// Sort order of the sources table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    LastSyncedDesc,
    LastEditedDesc,
    WordCountDesc,
    QualityDesc,
    TitleAsc,
}

/// Quality tier assigned to a source by the embedding/evaluation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EssenceQuality {
    High,
    Medium,
    Low,
}

impl EssenceQuality {
    /// From a score of 0.0–5.0. Thresholds mirror the mockup.
    pub fn from_score(score: f32) -> Self {
        if score >= 4.0 {
            Self::High
        } else if score >= 3.0 {
            Self::Medium
        } else {
            Self::Low
        }
    }

    pub fn css_modifier(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "med",
            Self::Low => "low",
        }
    }
}

/// In-House toggle state. The "inferred public layer" flag per source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InHouseStatus {
    On,
    Off,
    Paused,
}

impl InHouseStatus {
    pub fn is_on(&self) -> bool {
        matches!(self, Self::On)
    }
}

/// A single source feeding the user's Essence. Currently populated from
/// mock data inside `use_essence_sources`; swap for a server response
/// type (with `Loader<_>`) once the backend endpoint lands.
#[derive(Debug, Clone, PartialEq)]
pub struct EssenceSourceResponse {
    pub id: String,
    pub kind: EssenceSourceKind,
    pub title: String,
    /// URL or path shown as the first meta line (e.g. "Notion · /workspace/essay/mcp").
    pub source_path: String,
    pub chunks: u32,
    pub extra_meta: Option<String>,
    pub word_count: u32,
    pub last_synced_label: String,
    /// Raw score 0.0–5.0 — used for both the badge number and quality tier.
    pub quality_score: f32,
    pub in_house: InHouseStatus,
    /// True when flagged by the AI moderator. Stored separately from
    /// `InHouseStatus::Paused` because a source can be "paused AND flagged"
    /// or just "flagged but still active".
    pub ai_flagged: bool,
}

impl EssenceSourceResponse {
    pub fn quality(&self) -> EssenceQuality {
        EssenceQuality::from_score(self.quality_score)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.in_house, InHouseStatus::Paused)
    }
}
