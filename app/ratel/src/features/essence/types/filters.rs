use super::source_kind::EssenceSourceKind;
use serde::{Deserialize, Serialize};

/// Client-side filter chip over the breakdown cards. `All` bypasses the
/// kind filter; every other variant maps to a single `EssenceSourceKind`.
/// `Comment` collapses both `PostComment` and `DiscussionComment` into one
/// chip — the row-level badge still distinguishes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KindFilter {
    #[default]
    All,
    Notion,
    Post,
    Comment,
    Poll,
    Quiz,
}

impl KindFilter {
    pub fn matches(&self, kind: EssenceSourceKind) -> bool {
        match self {
            Self::All => true,
            Self::Notion => kind == EssenceSourceKind::Notion,
            Self::Post => kind == EssenceSourceKind::Post,
            Self::Comment => matches!(
                kind,
                EssenceSourceKind::PostComment | EssenceSourceKind::DiscussionComment
            ),
            Self::Poll => kind == EssenceSourceKind::Poll,
            Self::Quiz => kind == EssenceSourceKind::Quiz,
        }
    }
}

/// Server-backed sort order for the `GET /api/essences` list. Each variant
/// corresponds to a GSI on `Essence`, so pagination stays correct no
/// matter how many rows a user has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum EssenceSort {
    /// GSI1 — `updated_at` descending.
    #[default]
    LastEditedDesc,
    /// GSI2 — `word_count` descending.
    WordCountDesc,
    /// GSI3 — `title_lower` ascending (case-insensitive A–Z).
    TitleAsc,
}
