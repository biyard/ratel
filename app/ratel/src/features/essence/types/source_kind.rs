use crate::*;
use dioxus_translate::Translate;
use serde::{Deserialize, Serialize};

/// The kind of source feeding a user's Essence House. One enum used by both
/// server (model field, API DTO) and client (filter pill + row icon). When a
/// source is a comment we split it into `PostComment` vs `DiscussionComment`
/// so the UI can render a parent-context tag without re-querying.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Translate,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum EssenceSourceKind {
    #[default]
    #[translate(en = "Post", ko = "포스트")]
    Post,
    #[translate(en = "Poll", ko = "투표")]
    Poll,
    #[translate(en = "Quiz", ko = "퀴즈")]
    Quiz,
    #[translate(en = "Post comment", ko = "포스트 댓글")]
    PostComment,
    #[translate(en = "Discussion comment", ko = "토론 댓글")]
    DiscussionComment,
    #[translate(en = "Notion", ko = "노션")]
    Notion,
}

impl EssenceSourceKind {
    /// `true` when this source is a comment — the UI renders a parent-kind
    /// tag badge on these rows.
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::PostComment | Self::DiscussionComment)
    }
}
