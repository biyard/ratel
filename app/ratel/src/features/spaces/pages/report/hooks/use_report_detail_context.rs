//! Detail page context — turn-key data + UI state for the block editor.
//! Mirrors the `use_wall_context` pattern from PR #1593: a single
//! `use_loader` resolves the report (mock data for now), the wrapping
//! `DioxusController` exposes signals for picker / banner state, and
//! sub-components consume via `use_report_detail_context()`.
//!
//! Mutation (insert chart from picker → append `Chart` block) goes
//! through `insert_chart_for_item` which pushes to the `blocks` Signal;
//! the outline rail and `DocCanvas` re-render reactively.

use crate::features::spaces::pages::report::types::*;
use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DetailDrawerTarget {
    Closed,
    DataPicker,
}

/// Outline rail mode — default shows the heading/chart list + meta;
/// `ChartTypeSwap` swaps it out for a chart-type picker scoped to one
/// chart block (the gear-icon clicked block).
#[derive(Clone, PartialEq, Eq)]
pub enum OutlineMode {
    Default,
    ChartTypeSwap { block_id: String },
}

/// State for the slash-command popup (`/data`, `/data:analyze`, ...).
/// `level` mirrors the mockup's tier system: 0=command, 1=analyze,
/// 2=source, 3=item. `caret_x` / `caret_y` are viewport-relative pixels
/// from `window.getSelection().getRangeAt(0).getBoundingClientRect()`
/// so the popup anchors right under the caret. `selected_index` is
/// the keyboard-driven highlight inside the visible option list.
#[derive(Clone, PartialEq)]
pub struct SlashState {
    pub block_id: String,
    pub raw: String,
    pub level: u8,
    pub query: String,
    pub analyze_id: Option<String>,
    pub source: Option<ActionSource>,
    /// Doc-relative caret position (scrolls with `.report-detail__doc`).
    pub caret_x: f64,
    pub caret_y: f64,
    /// "below" = popup top at caret_y; "above" = popup bottom at caret_y.
    pub placement: String,
    pub selected_index: usize,
}

/// One rendered row in the slash popup. Built by `slash_options()` from
/// the current `SlashState` + the analyze list, consumed by both the
/// popup component and the keyboard-driven Enter handler.
#[derive(Clone, PartialEq)]
pub struct SlashOption {
    pub id: String,
    pub title: String,
    pub meta: String,
    pub action: SlashAction,
    pub filters: Vec<CrossFilterChip>,
}

#[derive(Clone, PartialEq)]
pub enum SlashAction {
    PickCommand,
    PickAnalyze {
        analyze_id: String,
    },
    PickSource {
        source: ActionSource,
    },
    InsertItem {
        analyze_id: String,
        source: ActionSource,
        item_id: String,
    },
}

#[derive(Clone, Copy, DioxusController)]
pub struct UseReportDetailContext {
    pub space_id: ReadSignal<SpacePartition>,
    pub report_id: ReadSignal<String>,
    pub detail: Loader<ReportDetail>,
    /// Block list rendered by `DocCanvas`. Mutated by the picker (insert
    /// chart) and by `remove_block` / `swap_chart_source`.
    pub blocks: Signal<Vec<ReportBlock>>,
    /// Editable title / subtitle for the doc header.
    pub title: Signal<String>,
    pub subtitle: Signal<String>,
    /// Picker / overlay routing. Only one drawer is open at a time.
    pub drawer: Signal<DetailDrawerTarget>,
    /// Currently selected analyze id in the picker dropdown.
    pub picker_analyze_id: Signal<String>,
    /// Active source tab in the picker.
    pub picker_source: Signal<ActionSource>,
    /// Right-rail mode (default outline + meta vs chart-type swap).
    pub outline_mode: Signal<OutlineMode>,
    /// Slash-command popup state. `None` means the popup is hidden.
    pub slash: Signal<Option<SlashState>>,
    /// Monotonic counter used to mint unique block ids. Mock blocks own
    /// authored ids ("report-h1-insights", "chart-policy-q1") so we
    /// start the sequence well above those to avoid collisions even
    /// when an inserted block happens to share an item id with a mock.
    pub next_block_seq: Signal<u64>,
    /// Save the current title/subtitle/blocks back to the server via
    /// `update_report_handler`. The action restarts the `detail`
    /// loader on success so any server-side normalization (timestamps,
    /// trimmed fields) round-trips back into the editor.
    pub handle_save: Action<(), ()>,
}

impl UseReportDetailContext {
    // ──────────────────────────────────────────────────────────────────
    // Lazy accessors — every consumer reads through these methods so
    // the loader / signal access happens at the RSX node that needs the
    // value (not at the top of a component). Once the upcoming
    // `Blur<Result<Loader<T>, _>>` wrapper lands the same accessor list
    // can be reshaped to return the wrapped result instead of the bare
    // value, with no caller-side change. See
    // `feedback_lazy_loader_resolve.md`.
    // ──────────────────────────────────────────────────────────────────

    // -- Loader-derived (immutable initial values from server) -------
    pub fn eyebrow(&self) -> String {
        self.detail().eyebrow.clone()
    }

    /// Title as it was loaded from the server. Used by the TopBar
    /// breadcrumb which intentionally shows the saved title — the
    /// live editable title lives in `title_value()`.
    pub fn initial_title(&self) -> String {
        self.detail().title.clone()
    }

    pub fn author(&self) -> String {
        self.detail().author.clone()
    }

    pub fn created_relative(&self) -> String {
        format_relative_kr(self.detail().created_at)
    }

    pub fn edited_relative(&self) -> String {
        format_relative_kr(self.detail().updated_at)
    }

    pub fn analyzes(&self) -> Vec<Analyze> {
        self.detail().analyzes.clone()
    }

    // -- Signal-derived (editor-mutable state) -----------------------
    pub fn title_value(&self) -> String {
        self.title.read().clone()
    }

    pub fn subtitle_value(&self) -> String {
        self.subtitle.read().clone()
    }

    pub fn blocks_list(&self) -> Vec<ReportBlock> {
        self.blocks.read().clone()
    }

    pub fn outline_mode_value(&self) -> OutlineMode {
        self.outline_mode.read().clone()
    }

    pub fn drawer_target(&self) -> DetailDrawerTarget {
        *self.drawer.read()
    }

    pub fn picker_analyze_id_value(&self) -> String {
        self.picker_analyze_id.read().clone()
    }

    pub fn picker_source_value(&self) -> ActionSource {
        *self.picker_source.read()
    }

    pub fn is_data_picker_open(&self) -> bool {
        matches!(self.drawer_target(), DetailDrawerTarget::DataPicker)
    }

    // -- Mutations ---------------------------------------------------
    pub fn open_data_picker(&mut self) {
        self.drawer.set(DetailDrawerTarget::DataPicker);
    }

    pub fn close_drawer(&mut self) {
        self.drawer.set(DetailDrawerTarget::Closed);
    }

    pub fn current_analyze(&self) -> Option<Analyze> {
        let id = self.picker_analyze_id.read().clone();
        self.detail()
            .analyzes
            .iter()
            .find(|a| a.id == id)
            .cloned()
            .or_else(|| self.detail().analyzes.first().cloned())
    }

    /// Aggregate items belonging to the currently-selected analyze /
    /// source tab combination in the right-rail picker. Computed at
    /// call time so the picker view can defer signal+loader reads to
    /// the RSX node that needs them.
    pub fn current_picker_items(&self) -> Vec<AnalyzeItem> {
        let source = self.picker_source_value();
        self.current_analyze()
            .as_ref()
            .map(|a| a.items_for(source).to_vec())
            .unwrap_or_default()
    }

    pub fn current_picker_items_is_empty(&self) -> bool {
        let source = self.picker_source_value();
        self.current_analyze()
            .as_ref()
            .map(|a| a.items_for(source).is_empty())
            .unwrap_or(true)
    }

    /// How many items the currently-selected analyze has for the given
    /// source. Used by the picker source-tab row to show counts and
    /// disable empty tabs.
    pub fn picker_items_count_for(&self, source: ActionSource) -> usize {
        self.current_analyze()
            .as_ref()
            .map(|a| a.items_for(source).len())
            .unwrap_or(0)
    }

    pub fn outline(&self) -> Vec<OutlineEntry> {
        self.blocks
            .read()
            .iter()
            .filter_map(|b| match b {
                ReportBlock::H1 { id, text } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::H1,
                    label: text.clone(),
                }),
                ReportBlock::H2 { id, text } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::H2,
                    label: text.clone(),
                }),
                ReportBlock::H3 { id, text } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::H3,
                    label: text.clone(),
                }),
                ReportBlock::Chart { id, item_title, .. } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::Chart,
                    label: item_title.clone(),
                }),
                ReportBlock::Text { .. } => None,
            })
            .collect()
    }

    /// True when no block in the current document contributes an
    /// outline entry. Lets the outline rail render the empty-state
    /// branch without first materializing the entry list.
    pub fn outline_is_empty(&self) -> bool {
        !self
            .blocks
            .read()
            .iter()
            .any(|b| !matches!(b, ReportBlock::Text { .. }))
    }

    /// Mint a unique block id with the given prefix. The sequence
    /// counter is monotonic across the session so even repeated picks of
    /// the same analyze item produce distinct chart blocks (the keyed
    /// diff panics on duplicate sibling keys).
    fn mint_id(&mut self, prefix: &str) -> String {
        let seq = *self.next_block_seq.peek();
        self.next_block_seq.set(seq + 1);
        format!("{prefix}-{seq}")
    }

    /// Push a Chart block built from the picker's selection. The chart
    /// type is picked from `ChartType::default_for(source)` so
    /// discussion data lands as a topics list, follow data as a pie, etc.
    /// If `after_block_id` is provided the chart is inserted right after
    /// that block (slash-command flow); otherwise it is appended (right-
    /// rail picker flow). A trailing empty Text block is always inserted
    /// so the author has somewhere to type below the new chart.
    pub fn insert_chart_for_item(
        &mut self,
        analyze: &Analyze,
        item: &AnalyzeItem,
        after_block_id: Option<&str>,
    ) -> (String, String) {
        let src = analyze
            .filters
            .first()
            .map(|c| c.source)
            .unwrap_or(ActionSource::Poll);
        let chart_id = self.mint_id("chart");
        let text_id = self.mint_id("text");
        let chart_block = ReportBlock::Chart {
            id: chart_id.clone(),
            source: src,
            chart_type: ChartType::default_for(src),
            analyze_name: analyze.name.clone(),
            item_title: item.title.clone(),
            meta: item.meta.clone(),
        };
        let trailing_text = ReportBlock::Text {
            id: text_id.clone(),
            html: String::new(),
        };

        let mut cur = self.blocks.peek().clone();
        let insert_pos = after_block_id
            .and_then(|aid| cur.iter().position(|b| b.id() == aid))
            .map(|i| i + 1)
            .unwrap_or(cur.len());
        cur.insert(insert_pos, chart_block);
        cur.insert(insert_pos + 1, trailing_text);
        self.blocks.set(cur);
        (chart_id, text_id)
    }

    pub fn update_block_text(&mut self, block_id: &str, new_text: String) {
        let mut cur = self.blocks.peek().clone();
        let mut changed = false;
        for b in cur.iter_mut() {
            if b.id() == block_id {
                match b {
                    ReportBlock::H1 { text, .. }
                    | ReportBlock::H2 { text, .. }
                    | ReportBlock::H3 { text, .. } => {
                        if *text != new_text {
                            *text = new_text.clone();
                            changed = true;
                        }
                    }
                    ReportBlock::Text { html, .. } => {
                        if *html != new_text {
                            *html = new_text.clone();
                            changed = true;
                        }
                    }
                    ReportBlock::Chart { .. } => {}
                }
                break;
            }
        }
        if changed {
            self.blocks.set(cur);
        }
    }

    pub fn append_text_block(&mut self) -> String {
        let text_id = self.mint_id("text");
        let new_block = ReportBlock::Text {
            id: text_id.clone(),
            html: String::new(),
        };
        let mut cur = self.blocks.peek().clone();
        cur.push(new_block);
        self.blocks.set(cur);
        text_id
    }

    pub fn first_editable_block_id(&self) -> Option<String> {
        self.blocks
            .peek()
            .iter()
            .find(|b| !matches!(b, ReportBlock::Chart { .. }))
            .map(|b| b.id().to_string())
    }

    /// Insert an empty Text block right after `after_block_id` and
    /// return its id. Used by the Enter-key handler in `DocBlock` so the
    /// author can break out of a heading or chart into a fresh paragraph.
    pub fn insert_text_after(&mut self, after_block_id: &str) -> String {
        let text_id = self.mint_id("text");
        let new_block = ReportBlock::Text {
            id: text_id.clone(),
            html: String::new(),
        };
        let mut cur = self.blocks.peek().clone();
        let pos = cur
            .iter()
            .position(|b| b.id() == after_block_id)
            .map(|i| i + 1)
            .unwrap_or(cur.len());
        cur.insert(pos, new_block);
        self.blocks.set(cur);
        text_id
    }

    /// Backspace-on-empty-block path: remove the block at `block_id` and
    /// return the id of the previous editable block (H1/H2/H3/Text) so
    /// the caller can focus it. Returns `None` when there is no editable
    /// sibling before this block (caret has nowhere to land — leave the
    /// block in place so the user doesn't get stuck on a chart).
    pub fn collapse_block(&mut self, block_id: &str) -> Option<String> {
        let cur = self.blocks.peek().clone();
        let idx = cur.iter().position(|b| b.id() == block_id)?;
        // Walk backwards to find the nearest editable (non-Chart) block.
        let prev_id = cur[..idx].iter().rev().find_map(|b| match b {
            ReportBlock::Chart { .. } => None,
            _ => Some(b.id().to_string()),
        })?;
        let mut next = cur;
        next.remove(idx);
        self.blocks.set(next);
        Some(prev_id)
    }

    /// Drop a block (chart trash icon, or future block menu).
    pub fn remove_block(&mut self, id: &str) {
        let mut cur = self.blocks.peek().clone();
        cur.retain(|b| b.id() != id);
        self.blocks.set(cur);
    }

    /// Re-render an existing chart block as a different visual type
    /// (bar → pie etc.). Triggered from the outline rail's chart-swap
    /// mode.
    pub fn swap_chart_type(&mut self, id: &str, new_type: ChartType) {
        let mut cur = self.blocks.peek().clone();
        for b in cur.iter_mut() {
            if b.id() == id {
                if let ReportBlock::Chart { chart_type, .. } = b {
                    *chart_type = new_type;
                }
            }
        }
        self.blocks.set(cur);
    }

    pub fn open_chart_swap(&mut self, id: &str) {
        self.outline_mode.set(OutlineMode::ChartTypeSwap {
            block_id: id.to_string(),
        });
    }

    pub fn close_outline_swap(&mut self) {
        self.outline_mode.set(OutlineMode::Default);
    }

    pub fn close_slash(&mut self) {
        self.slash.set(None);
    }

    /// Apply the picked slash option. `analyze` + `item` are resolved
    /// by the popup before calling so this stays a pure mutation.
    /// The chart is inserted right after the block where the slash was
    /// active, the trailing slash token is stripped from that block's
    /// contenteditable, and focus jumps to the freshly-created empty
    /// Text block so the author can keep typing.
    pub fn apply_slash_chart(&mut self, analyze: &Analyze, item: &AnalyzeItem) {
        let (source_id, raw_token) = self
            .slash
            .peek()
            .as_ref()
            .map(|s| (s.block_id.clone(), s.raw.clone()))
            .unzip();
        let (_, text_id) = self.insert_chart_for_item(analyze, item, source_id.as_deref());
        self.slash.set(None);
        if let (Some(sid), Some(raw)) = (source_id, raw_token) {
            cleanup_slash_and_focus(&sid, &raw, Some(&text_id));
        } else {
            cleanup_slash_and_focus("", "", Some(&text_id));
        }
    }

    /// Build the visible option list for the current slash state.
    /// Returns an empty Vec when no slash popup is active.
    pub fn slash_options(&self) -> Vec<SlashOption> {
        let Some(state) = self.slash.read().clone() else {
            return Vec::new();
        };
        build_slash_options(&state, &self.detail().analyzes)
    }

    /// Adjust the keyboard-highlighted option (ArrowDown=+1,
    /// ArrowUp=-1) with wrap-around. Clamped to the current option
    /// list size so the keyboard cursor stays in range across level
    /// transitions.
    pub fn move_slash_selection(&mut self, delta: i32) {
        let opts = self.slash_options();
        if opts.is_empty() {
            return;
        }
        let cur = self.slash.peek().clone();
        let Some(mut state) = cur else { return };
        let len = opts.len() as i32;
        let mut next = state.selected_index as i32 + delta;
        next = ((next % len) + len) % len;
        state.selected_index = next as usize;
        self.slash.set(Some(state));
    }

    /// Apply whatever option is currently highlighted (Enter key from
    /// the contenteditable, or click from the popup). Routes through
    /// the same `SlashAction` switch as the click handler so the two
    /// paths stay in sync.
    pub fn apply_slash_selected(&mut self) {
        let opts = self.slash_options();
        if opts.is_empty() {
            return;
        }
        let idx = self
            .slash
            .peek()
            .as_ref()
            .map(|s| s.selected_index.min(opts.len().saturating_sub(1)))
            .unwrap_or(0);
        let action = opts[idx].action.clone();
        self.apply_slash_action(&action);
    }

    pub fn apply_slash_action(&mut self, action: &SlashAction) {
        let cur = self.slash.peek().clone();
        let Some(mut state) = cur else { return };
        let block_id = state.block_id.clone();
        let old_raw = state.raw.clone();

        match action {
            SlashAction::PickCommand => {
                state.level = 1;
                state.query = String::new();
                state.raw = "/data:".to_string();
                state.selected_index = 0;
                let new_raw = state.raw.clone();
                self.slash.set(Some(state));
                replace_slash_text(&block_id, &old_raw, &new_raw);
            }
            SlashAction::PickAnalyze { analyze_id } => {
                state.level = 2;
                state.query = String::new();
                state.analyze_id = Some(analyze_id.clone());
                state.raw = format!("/data:{analyze_id}:");
                state.selected_index = 0;
                let new_raw = state.raw.clone();
                self.slash.set(Some(state));
                replace_slash_text(&block_id, &old_raw, &new_raw);
            }
            SlashAction::PickSource { source } => {
                state.level = 3;
                state.query = String::new();
                state.source = Some(*source);
                state.raw = format!(
                    "/data:{}:{}:",
                    state.analyze_id.clone().unwrap_or_default(),
                    source.as_token()
                );
                state.selected_index = 0;
                let new_raw = state.raw.clone();
                self.slash.set(Some(state));
                replace_slash_text(&block_id, &old_raw, &new_raw);
            }
            SlashAction::InsertItem {
                analyze_id,
                source,
                item_id,
            } => {
                let detail = self.detail();
                if let Some(analyze) = detail.analyzes.iter().find(|a| &a.id == analyze_id) {
                    if let Some(item) = analyze.items_for(*source).iter().find(|i| &i.id == item_id)
                    {
                        let analyze_clone = analyze.clone();
                        let item_clone = item.clone();
                        self.apply_slash_chart(&analyze_clone, &item_clone);
                    }
                }
            }
        }
    }
}

/// Shared by `slash_options()` (context method) and the popup component.
fn build_slash_options(state: &SlashState, analyzes: &[Analyze]) -> Vec<SlashOption> {
    let q = state.query.to_lowercase();
    match state.level {
        0 => {
            if "data".starts_with(&q) || q.is_empty() {
                vec![SlashOption {
                    id: "data".to_string(),
                    title: "/data:".to_string(),
                    meta: "analyze 데이터로 차트 삽입".to_string(),
                    action: SlashAction::PickCommand,
                    filters: Vec::new(),
                }]
            } else {
                Vec::new()
            }
        }
        1 => analyzes
            .iter()
            .filter(|a| q.is_empty() || a.name.to_lowercase().contains(&q))
            .map(|a| SlashOption {
                id: a.id.clone(),
                title: a.name.clone(),
                meta: format!("응답자 {}명", a.respondents),
                action: SlashAction::PickAnalyze {
                    analyze_id: a.id.clone(),
                },
                filters: a.filters.clone(),
            })
            .collect(),
        2 => {
            let Some(aid) = &state.analyze_id else {
                return Vec::new();
            };
            let Some(analyze) = analyzes.iter().find(|a| &a.id == aid) else {
                return Vec::new();
            };
            ActionSource::VARIANTS
                .iter()
                .copied()
                .filter(|s| {
                    let token = s.as_token();
                    !analyze.items_for(*s).is_empty() && (q.is_empty() || token.starts_with(&q))
                })
                .map(|s| {
                    let label = match s {
                        ActionSource::Poll => "Poll",
                        ActionSource::Quiz => "Quiz",
                        ActionSource::Discussion => "Discussion",
                        ActionSource::Follow => "Follow",
                    };
                    SlashOption {
                        id: s.as_token().to_string(),
                        title: label.to_string(),
                        meta: format!("{}개", analyze.items_for(s).len()),
                        action: SlashAction::PickSource { source: s },
                        filters: Vec::new(),
                    }
                })
                .collect()
        }
        3 => {
            let Some(aid) = &state.analyze_id else {
                return Vec::new();
            };
            let Some(source) = state.source else {
                return Vec::new();
            };
            let Some(analyze) = analyzes.iter().find(|a| &a.id == aid) else {
                return Vec::new();
            };
            analyze
                .items_for(source)
                .iter()
                .filter(|item| q.is_empty() || item.title.to_lowercase().contains(&q))
                .map(|item| SlashOption {
                    id: item.id.clone(),
                    title: item.title.clone(),
                    meta: item.meta.clone(),
                    action: SlashAction::InsertItem {
                        analyze_id: aid.clone(),
                        source,
                        item_id: item.id.clone(),
                    },
                    filters: Vec::new(),
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

#[track_caller]
pub fn use_report_detail_context() -> UseReportDetailContext {
    use_context()
}

#[track_caller]
pub fn use_report_detail_context_provider(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> Result<UseReportDetailContext, Loading> {
    // Fetch the saved report from the server. No mock data — the
    // server response is mapped 1:1 into the page-local `ReportDetail`
    // shape so the rest of the controller layer doesn't need to
    // change. `analyzes` stays empty for now; the slash popup will
    // surface zero options until the analyze_reports integration is
    // wired in a follow-up.
    let detail = use_loader(move || {
        let sid = space_id();
        let rid = report_id();
        async move {
            let report_id_typed: SpaceReportEntityType = rid.clone().into();
            let resp = crate::features::spaces::pages::report::controllers::get_report(
                sid,
                report_id_typed,
            )
            .await?;
            Ok::<_, crate::common::Error>(ReportDetail {
                id: resp.id,
                eyebrow: "Action · Report".to_string(),
                title: resp.title,
                subtitle: resp.description,
                blocks: resp.blocks,
                author: resp.author,
                created_at: resp.created_at,
                updated_at: resp.updated_at,
                analyzes: Vec::new(),
            })
        }
    })?;

    let snapshot = detail();
    let blocks = use_signal(|| snapshot.blocks.clone());
    let title = use_signal(|| snapshot.title.clone());
    let subtitle = use_signal(|| snapshot.subtitle.clone());
    let drawer = use_signal(|| DetailDrawerTarget::Closed);
    let picker_analyze_id = use_signal(|| {
        snapshot
            .analyzes
            .first()
            .map(|a| a.id.clone())
            .unwrap_or_default()
    });
    let picker_source = use_signal(|| ActionSource::Poll);
    let outline_mode = use_signal(|| OutlineMode::Default);
    let slash = use_signal(|| Option::<SlashState>::None);
    let next_block_seq = use_signal(|| 1000u64);

    let mut detail_for_save = detail;
    let handle_save = use_action(move || async move {
        let report_id_typed: SpaceReportEntityType = report_id().into();
        let req = crate::features::spaces::pages::report::controllers::UpdateReportRequest {
            title: Some(title.peek().clone()),
            description: Some(subtitle.peek().clone()),
            blocks: Some(blocks.peek().clone()),
            status: None,
            html_contents: None,
        };
        crate::features::spaces::pages::report::controllers::update_report(
            space_id(),
            report_id_typed,
            req,
        )
        .await?;
        detail_for_save.restart();
        Ok::<(), crate::common::Error>(())
    });

    let ctx = use_context_provider(move || UseReportDetailContext {
        space_id,
        report_id,
        detail,
        blocks,
        title,
        subtitle,
        drawer,
        picker_analyze_id,
        picker_source,
        outline_mode,
        slash,
        next_block_seq,
        handle_save,
    });

    Ok(ctx)
}

fn format_relative_kr(timestamp_millis: i64) -> String {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let diff = (now - timestamp_millis).max(0);
    if diff < 60_000 {
        return "방금".to_string();
    }
    let mins = diff / 60_000;
    if mins < 60 {
        return format!("{mins}분 전");
    }
    let hours = mins / 60;
    if hours < 24 {
        return format!("{hours}시간 전");
    }
    let days = hours / 24;
    if days < 7 {
        return format!("{days}일 전");
    }
    let weeks = days / 7;
    if weeks < 5 {
        return format!("{weeks}주 전");
    }
    let months = days / 30;
    if months < 12 {
        return format!("{months}개월 전");
    }
    let years = days / 365;
    format!("{years}년 전")
}

/// Fire-and-forget DOM patch: replace the trailing `old_raw` substring
/// at the end of the source contenteditable with `new_raw`. Used when
/// the slash popup transitions levels (e.g. `/da` → `/data:` →
/// `/data:policy-priority:`) so the editor reflects the picked path
/// instead of leaving the user to keep typing the next segment by hand.
/// The caret is placed at the end of the inserted text so the next
/// keystroke continues the slash chain.
fn replace_slash_text(block_id: &str, old_raw: &str, new_raw: &str) {
    let mut runner = document::eval(
        r#"
        const data = await dioxus.recv();
        const blockId = data.blockId || "";
        const oldRaw = data.oldRaw || "";
        const newRaw = data.newRaw || "";
        const el = document.getElementById(blockId);
        if (!el) { dioxus.send(null); return; }

        // 1. Strip `oldRaw.length` characters from the tail, walking
        //    text nodes back-to-front so inline formatting earlier in
        //    the block is preserved.
        let remaining = oldRaw.length;
        let node = el.lastChild;
        while (node && remaining > 0) {
            if (node.nodeType === 3) {
                const t = node.textContent || "";
                if (t.length <= remaining) {
                    remaining -= t.length;
                    const prev = node.previousSibling;
                    node.remove();
                    node = prev;
                } else {
                    node.textContent = t.slice(0, t.length - remaining);
                    remaining = 0;
                }
            } else {
                node = node.previousSibling;
            }
        }

        // 2. Append the new raw token as a fresh text node.
        if (newRaw) {
            el.appendChild(document.createTextNode(newRaw));
        }

        // 3. Place the caret at the very end of the block so the user
        //    can keep typing the next segment.
        el.focus();
        const range = document.createRange();
        const sel = window.getSelection();
        range.selectNodeContents(el);
        range.collapse(false);
        sel.removeAllRanges();
        sel.addRange(range);
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!({
        "blockId": block_id,
        "oldRaw": old_raw,
        "newRaw": new_raw,
    }));
}

/// Fire-and-forget DOM cleanup after a block-list mutation:
/// 1. Strip the trailing slash token from the source block (so the
///    `/data:...` text the user typed disappears once the chart is in).
/// 2. Focus the new target block and place the caret at its start.
///
/// `source_block_id` / `raw_token` may be empty when there is no slash
/// state to clean up (right-rail picker flow, Enter-to-new-block);
/// in that case only the focus step runs.
fn cleanup_slash_and_focus(source_block_id: &str, raw_token: &str, focus_block_id: Option<&str>) {
    let mut runner = document::eval(
        r#"
        const data = await dioxus.recv();
        const sourceId = data.sourceId || "";
        const raw = data.raw || "";
        const focusId = data.focusId || "";

        if (sourceId && raw) {
            const el = document.getElementById(sourceId);
            if (el) {
                // Walk text nodes from the end, peeling off characters
                // until we have removed `raw.length` chars. Preserves
                // any inline HTML earlier in the block.
                let remaining = raw.length;
                let node = el.lastChild;
                while (node && remaining > 0) {
                    if (node.nodeType === 3) {
                        const t = node.textContent || "";
                        if (t.length <= remaining) {
                            remaining -= t.length;
                            const prev = node.previousSibling;
                            node.remove();
                            node = prev;
                        } else {
                            node.textContent = t.slice(0, t.length - remaining);
                            remaining = 0;
                        }
                    } else {
                        node = node.previousSibling;
                    }
                }
            }
        }

        if (focusId) {
            // Defer focus to the next frame so the freshly-inserted
            // Dioxus block is in the DOM before we try to focus it.
            requestAnimationFrame(() => {
                const target = document.getElementById(focusId);
                if (!target) return;
                target.focus();
                const range = document.createRange();
                const sel = window.getSelection();
                range.selectNodeContents(target);
                range.collapse(true);
                sel.removeAllRanges();
                sel.addRange(range);
            });
        }
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!({
        "sourceId": source_block_id,
        "raw": raw_token,
        "focusId": focus_block_id.unwrap_or(""),
    }));
}
