//! Wizard-state enums shared between the picker view and the
//! `UseAnalyzeCreate` controller. Pure data — no server calls, no
//! mock seed data.

use super::report::AnalyzeFilterSource;

/// Top-level wizard mode — drives `data-mode` on `.split` /
/// `.analyze-builder`. Same attribute the original HTML mockup used,
/// so the existing CSS picks up "Create vs Preview" styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateMode {
    /// Cross-filter selection step (1 / 2). Shows the cross-filter
    /// card + cf-sunji picker.
    Create,
    /// Confirm step (2 / 2). Shows the name input, chip summary, and
    /// preview-count tiles.
    Preview,
}

/// State machine inside the cross-filter card. Mirrors
/// `[data-add-state]` in the HTML mockup — `idle | picking-action |
/// picking-item`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddState {
    /// Just the chips strip + "+ 필터 추가" CTA.
    Idle,
    /// 4-tile action picker (Poll / Quiz / Discussion / Follow).
    PickingAction,
    /// Single-select radio list of items for the picked action type.
    /// For Follow the radio list stays empty — cf-sunji's flat
    /// target list is rendered directly instead.
    PickingItem,
}

impl AddState {
    pub fn as_str(&self) -> &'static str {
        match self {
            AddState::Idle => "idle",
            AddState::PickingAction => "picking-action",
            AddState::PickingItem => "picking-item",
        }
    }
}

impl CreateMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CreateMode::Create => "create",
            CreateMode::Preview => "preview",
        }
    }
}

impl AnalyzeFilterSource {
    /// Display label used in the action tile (`Poll`, `Quiz`, …) and
    /// in chip badges. Capitalised.
    pub fn type_label(&self) -> &'static str {
        match self {
            AnalyzeFilterSource::Poll => "Poll",
            AnalyzeFilterSource::Quiz => "Quiz",
            AnalyzeFilterSource::Discussion => "Discussion",
            AnalyzeFilterSource::Follow => "Follow",
        }
    }

    /// Uppercase chip badge ("POLL" / "QUIZ" / "DISCUSSION" / "FOLLOW").
    pub fn badge(&self) -> &'static str {
        match self {
            AnalyzeFilterSource::Poll => "POLL",
            AnalyzeFilterSource::Quiz => "QUIZ",
            AnalyzeFilterSource::Discussion => "DISCUSSION",
            AnalyzeFilterSource::Follow => "FOLLOW",
        }
    }
}
