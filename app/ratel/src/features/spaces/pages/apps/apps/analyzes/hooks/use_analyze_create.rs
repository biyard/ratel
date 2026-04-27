//! Controller for the Analyze CREATE wizard (Phase 2).
//!
//! Owns every signal that drives the cross-filter card (.cross-filter)
//! and the cf-sunji picker. Mirrors the JS state machine in
//! `assets/design/analyze-create-arena.html`:
//!
//! ```text
//! mode: Create / Preview
//! └── add_state: Idle / PickingAction / PickingItem
//!     ├── picking_type: Option<AnalyzeFilterSource>
//!     ├── picked_item_id: Option<String>
//!     ├── picked_sunji: HashSet<String>      // "questionId:optionId" tokens
//!     └── keyword_input: String              // discussion comma-separated
//! ```
//!
//! Confirming the cf-sunji drains `picked_sunji` (and for discussion
//! items, `keyword_input` tokens) into `filters` and resets the
//! cross-filter to idle. There is no actual save — `goto_preview` /
//! `back_to_create` flip the wizard mode and the page navigates to
//! the existing detail mock report on confirm. Real persistence is
//! out of scope for Phase 2.

use std::collections::HashSet;

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

/// Bundle every signal + helper the CREATE wizard needs.
///
/// All methods are plain `&mut self` because they only flip signals;
/// there are no async mutations in Phase 2 (the whole flow is mock
/// data + local state). Once Phase 5+ adds a real "보고서 생성" call,
/// the confirm path becomes a `use_action` and lives here too.
#[derive(Clone, Copy)]
pub struct UseAnalyzeCreate {
    pub mode: Signal<CreateMode>,
    pub add_state: Signal<AddState>,
    pub picking_type: Signal<Option<AnalyzeFilterSource>>,
    pub picked_item_id: Signal<Option<String>>,
    pub picked_sunji: Signal<HashSet<String>>,
    pub keyword_input: Signal<String>,
    pub filters: Signal<Vec<AnalyzeReportFilter>>,
    pub preview_name: Signal<String>,
}

impl UseAnalyzeCreate {
    /// `idle → picking-action`.
    pub fn start_add(&mut self) {
        self.add_state.set(AddState::PickingAction);
        self.picking_type.set(None);
        self.picked_item_id.set(None);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    /// `picking-action → idle` (취소 button inside picking-action).
    pub fn cancel_add(&mut self) {
        self.add_state.set(AddState::Idle);
        self.picking_type.set(None);
        self.picked_item_id.set(None);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    /// `picking-action → picking-item` for the chosen action type.
    pub fn pick_action(&mut self, src: AnalyzeFilterSource) {
        self.picking_type.set(Some(src));
        self.picked_item_id.set(None);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
        self.add_state.set(AddState::PickingItem);
    }

    /// "← 액션 다시 선택" — back from picking-item to picking-action.
    pub fn back_to_action(&mut self) {
        self.picked_item_id.set(None);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
        self.add_state.set(AddState::PickingAction);
    }

    /// Item radio change — auto-opens the cf-sunji card. Switching the
    /// radio re-renders cf-sunji for the new item (callers also call
    /// `clear_sunji_state` first, but that's wrapped in here).
    pub fn pick_item(&mut self, item_id: String) {
        self.picked_item_id.set(Some(item_id));
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    /// "← 다시 선택" inside cf-sunji — closes the picker, leaves the
    /// radio list visible so the user can pick another item.
    pub fn clear_item(&mut self) {
        self.picked_item_id.set(None);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    /// Toggle a `${question_id}:${option_id}` token in/out of the
    /// `picked_sunji` set.
    pub fn toggle_sunji(&mut self, token: String) {
        let mut current = self.picked_sunji.write();
        if current.contains(&token) {
            current.remove(&token);
        } else {
            current.insert(token);
        }
    }

    /// Drain `picked_sunji` + (discussion-only) keyword_input tokens
    /// into `filters` and reset the cross-filter back to idle. Each
    /// picked option becomes one chip; each comma-separated keyword
    /// becomes its own chip with `option_id = "kw-{keyword}"`.
    pub fn confirm_sunji(&mut self) {
        let item_id = match self.picked_item_id.read().clone() {
            Some(id) => id,
            None => return,
        };
        let src = match *self.picking_type.read() {
            Some(s) => s,
            None => return,
        };
        let mut new_filters: Vec<AnalyzeReportFilter> = Vec::new();

        // Discussion: drain keyword input first (one chip per token).
        if matches!(src, AnalyzeFilterSource::Discussion) {
            let raw = self.keyword_input.read().clone();
            let mut seen: HashSet<String> = HashSet::new();
            for kw in raw
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
            {
                if seen.contains(&kw) {
                    continue;
                }
                seen.insert(kw.clone());
                new_filters.push(AnalyzeReportFilter {
                    source: src,
                    source_label: src.type_label().to_string(),
                    label: kw.clone(),
                    item_id: item_id.clone(),
                    question_id: "keywords".to_string(),
                    option_id: format!("kw-{}", kw),
                    option_text: kw,
                    question_title: "키워드 입력".to_string(),
                    correct: false,
                });
            }
        }

        // Poll / Quiz / Follow / discussion-topics — one chip per option.
        let tokens: Vec<String> = self.picked_sunji.read().iter().cloned().collect();
        for token in tokens {
            if let Some((q, o)) = resolve_sunji(&item_id, &token) {
                new_filters.push(AnalyzeReportFilter {
                    source: src,
                    source_label: src.type_label().to_string(),
                    label: o.label.clone(),
                    item_id: item_id.clone(),
                    question_id: q.id.clone(),
                    option_id: o.id.clone(),
                    option_text: o.label.clone(),
                    question_title: q.title.clone(),
                    correct: o.correct,
                });
            }
        }

        if !new_filters.is_empty() {
            let mut all = self.filters.write();
            all.extend(new_filters);
        }

        // Reset back to idle.
        self.add_state.set(AddState::Idle);
        self.picking_type.set(None);
        self.picked_item_id.set(None);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    /// Remove a chip by index.
    pub fn remove_filter(&mut self, idx: usize) {
        let mut all = self.filters.write();
        if idx < all.len() {
            all.remove(idx);
        }
    }

    /// Clear every chip.
    pub fn clear_filters(&mut self) {
        self.filters.set(Vec::new());
    }

    /// Step 1 → Step 2.
    pub fn goto_preview(&mut self) {
        self.mode.set(CreateMode::Preview);
    }

    /// Step 2 → Step 1 (← 이전 button).
    pub fn back_to_create(&mut self) {
        self.mode.set(CreateMode::Create);
    }
}

#[track_caller]
pub fn use_analyze_create() -> std::result::Result<UseAnalyzeCreate, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeCreate>() {
        return Ok(ctx);
    }

    let mode = use_signal(|| CreateMode::Create);
    let add_state = use_signal(|| AddState::Idle);
    let picking_type = use_signal::<Option<AnalyzeFilterSource>>(|| None);
    let picked_item_id = use_signal::<Option<String>>(|| None);
    let picked_sunji = use_signal(HashSet::<String>::new);
    let keyword_input = use_signal(String::new);
    let filters = use_signal(Vec::<AnalyzeReportFilter>::new);
    let preview_name = use_signal(String::new);

    Ok(use_context_provider(|| UseAnalyzeCreate {
        mode,
        add_state,
        picking_type,
        picked_item_id,
        picked_sunji,
        keyword_input,
        filters,
        preview_name,
    }))
}
