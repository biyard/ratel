//! Report detail page context — single rich-text body editor backed by
//! the shared `crate::common::components::editor::Editor`, plus the
//! picker / slash / outline UI pieces that wrap it.
//!
//! Charts are embedded in the body as `<figure contenteditable="false">`
//! elements built by `figure_html::build_chart_figure`. Picker selection,
//! chart-type swaps, and chart deletions all flow through small JS
//! dispatches against the editor's contenteditable so the Editor's own
//! debounced `on_content_change` callback handles persistence.

use super::super::views::detail::figure_html::{build_chart_figure, build_chart_inner};
use crate::common::components::editor::EditorSlashSignal;
use crate::features::spaces::pages::report::types::*;
use crate::*;
use dioxus::document::eval as dx_eval;
use regex::Regex;

/// Autosave lifecycle, mirrored from `post_edit`'s `EditorStatus`. Drives
/// the top-bar label ("자동 저장됨 · 방금" → "저장 안됨" → "저장 중…" →
/// "저장됨") and is bumped in lockstep with `save_version` so the
/// debounced effect knows when to actually fire the PATCH.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveStatus {
    Idle,
    Unsaved,
    Saving,
    Saved,
}

/// How long to wait after the last keystroke before persisting. Matches
/// `post_edit`'s autosave debounce so the two flows feel identical.
const AUTOSAVE_DEBOUNCE_SECS: u64 = 3;

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
    ChartTypeSwap { chart_id: String },
}

/// Snapshot of one chart figure embedded in the body HTML, sufficient
/// to drive the swap UI and re-render the chart after a type change.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartMeta {
    pub chart_id: String,
    pub source: ActionSource,
    pub chart_type: ChartType,
    pub analyze_name: String,
    pub item_title: String,
    pub item_meta: String,
    pub respondent_count: u32,
    pub options: Vec<ChartOption>,
    pub discussion_data: Option<DiscussionData>,
    pub text_answers: Vec<String>,
}

/// State for the slash-command popup (`/data`, `/data:analyze`, ...).
/// `level` mirrors the mockup's tier system: 0=command, 1=analyze,
/// 2=source, 3=item. Coordinates are viewport pixels emitted by the
/// editor's slash watcher.
#[derive(Clone, PartialEq)]
pub struct SlashState {
    pub raw: String,
    pub level: u8,
    pub query: String,
    pub analyze_id: Option<String>,
    pub source: Option<ActionSource>,
    pub caret_x: f64,
    pub caret_y: f64,
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
    pub detail: Loader<ReportDetail>,
    /// Editable title / subtitle for the doc header.
    pub title: Signal<String>,
    pub subtitle: Signal<String>,
    /// Body HTML — the single source of truth for the body. Mirrored
    /// from the shared `Editor`'s `on_content_change` callback.
    pub body_html: Signal<String>,
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
    /// Save the current title/subtitle/body_html back to the server.
    /// No longer wired to per-keystroke callbacks — the autosave effect
    /// (set up in the provider) watches `save_version` + the edit signals
    /// and triggers the PATCH at most once every
    /// `AUTOSAVE_DEBOUNCE_SECS`. Kept exposed so the publish path and
    /// any future "Save now" affordance can fire a save synchronously.
    pub handle_save: Action<(), ()>,
    /// Autosave lifecycle. Flips Idle → Unsaved on any edit, Saving →
    /// Saved (or back to Unsaved on error) once the debounced effect
    /// settles.
    pub status: Signal<SaveStatus>,
    /// Monotonic counter bumped by `mark_unsaved`. The autosave effect
    /// reads this in its dep set; an old captured value vs. the current
    /// one is the signal that "another edit landed during the debounce
    /// window — skip this stale save attempt."
    pub save_version: Signal<u64>,
    /// Last persisted (title, subtitle, body_html) snapshot. Used by
    /// `mark_unsaved` to skip the version bump when an edit was actually
    /// reverted to the saved state (so the status doesn't flicker to
    /// Unsaved on a no-op change).
    pub last_saved: Signal<(String, String, String)>,
}

impl UseReportDetailContext {
    // ──────────────────────────────────────────────────────────────────
    // Lazy accessors — every consumer reads through these methods so
    // the loader / signal access happens at the RSX node that needs the
    // value (not at the top of a component). See
    // `feedback_lazy_loader_resolve.md`.
    // ──────────────────────────────────────────────────────────────────

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

    pub fn title_value(&self) -> String {
        self.title.read().clone()
    }

    pub fn subtitle_value(&self) -> String {
        self.subtitle.read().clone()
    }

    /// Snapshot of the body HTML taken once at first render. Passed
    /// into the `Editor` as `content` — the Editor never re-reads this
    /// (it would clobber the caret on every keystroke), so the function
    /// must return the value that was current when the editor mounted.
    pub fn initial_body_html(&self) -> String {
        self.body_html.peek().clone()
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

    pub fn picker_items_count_for(&self, source: ActionSource) -> usize {
        self.current_analyze()
            .as_ref()
            .map(|a| a.items_for(source).len())
            .unwrap_or(0)
    }

    /// Mint a unique chart id. UUIDv7 keeps lex-order = creation order
    /// and prevents collisions across remounts (see PR comment in the
    /// old block-based implementation).
    fn mint_chart_id(&self) -> String {
        format!("chart-{}", uuid::Uuid::now_v7())
    }

    /// Picker selection — insert a fresh chart figure into the body at
    /// the editor's caret. Builds the figure HTML in Rust, then asks
    /// the editor's contenteditable to execute `insertHTML` with it. The
    /// Editor's input listener fires for free, which roundtrips to
    /// `body_html` via `on_content_change` → handle_save runs once
    /// debounce settles.
    pub fn insert_chart_from_picker(
        &mut self,
        analyze: &Analyze,
        item: &AnalyzeItem,
        source: ActionSource,
    ) {
        let chart_id = self.mint_chart_id();
        // Pick the chart type based on the item's data shape, not just
        // its source — a subjective poll/quiz question (empty options +
        // populated `text_answers`) gets `TextList` rendering.
        let chart_type = ChartType::default_for_item(item, source);
        let figure = build_chart_figure(&chart_id, source, chart_type, analyze, item);
        // Append an empty paragraph after the figure so the author has
        // somewhere to type. The figure itself is `contenteditable="false"`,
        // so without a trailing editable node the caret can get stuck.
        let html = format!("{figure}<p><br></p>");
        dispatch_insert_html(html);
        self.close_drawer();
    }

    /// Apply the picked slash option. `analyze` + `item` are resolved
    /// by the popup before calling so this stays a pure mutation.
    /// The chart is inserted right after the slash token is wiped from
    /// the editor.
    pub fn apply_slash_chart(
        &mut self,
        analyze: &Analyze,
        item: &AnalyzeItem,
        source: ActionSource,
    ) {
        let chart_id = self.mint_chart_id();
        let chart_type = ChartType::default_for_item(item, source);
        let figure = build_chart_figure(&chart_id, source, chart_type, analyze, item);
        let html = format!("{figure}<p><br></p>");
        let raw = self
            .slash
            .peek()
            .as_ref()
            .map(|s| s.raw.clone())
            .unwrap_or_default();
        self.slash.set(None);
        dispatch_replace_slash_and_insert(raw, html);
    }

    /// Update slash state from the Editor's on-slash signal. Empty `raw`
    /// (the cleared sentinel) hides the popup. Anything that doesn't
    /// match `/data...` is also dropped — the editor surfaces every
    /// `/<word>` token; this surface only cares about /data.
    pub fn handle_slash_signal(&mut self, signal: EditorSlashSignal) {
        if signal.raw.is_empty() {
            if self.slash.peek().is_some() {
                self.slash.set(None);
            }
            return;
        }
        let Some(parsed) = parse_slash_token(&signal.raw) else {
            if self.slash.peek().is_some() {
                self.slash.set(None);
            }
            return;
        };
        let prev_idx = self
            .slash
            .peek()
            .as_ref()
            .map(|s| (s.level, s.selected_index))
            .filter(|(lvl, _)| *lvl == parsed.level)
            .map(|(_, idx)| idx)
            .unwrap_or(0);
        self.slash.set(Some(SlashState {
            raw: parsed.raw,
            level: parsed.level,
            query: parsed.query,
            analyze_id: parsed.analyze_id,
            source: parsed.source,
            caret_x: signal.caret_x,
            caret_y: signal.caret_y,
            placement: signal.placement,
            selected_index: prev_idx,
        }));
    }

    pub fn close_slash(&mut self) {
        self.slash.set(None);
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
    /// ArrowUp=-1) with wrap-around.
    pub fn move_slash_selection(&mut self, delta: i32) {
        let opts = self.slash_options();
        if opts.is_empty() {
            return;
        }
        let Some(mut state) = self.slash.peek().clone() else {
            return;
        };
        let len = opts.len() as i32;
        let mut next = state.selected_index as i32 + delta;
        next = ((next % len) + len) % len;
        state.selected_index = next as usize;
        self.slash.set(Some(state));
    }

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
        let Some(mut state) = self.slash.peek().clone() else {
            return;
        };
        let old_raw = state.raw.clone();

        match action {
            SlashAction::PickCommand => {
                state.level = 1;
                state.query = String::new();
                state.raw = "/data:".to_string();
                state.selected_index = 0;
                let new_raw = state.raw.clone();
                self.slash.set(Some(state));
                dispatch_replace_slash_token(old_raw, new_raw);
            }
            SlashAction::PickAnalyze { analyze_id } => {
                state.level = 2;
                state.query = String::new();
                state.analyze_id = Some(analyze_id.clone());
                state.raw = format!("/data:{analyze_id}:");
                state.selected_index = 0;
                let new_raw = state.raw.clone();
                self.slash.set(Some(state));
                dispatch_replace_slash_token(old_raw, new_raw);
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
                dispatch_replace_slash_token(old_raw, new_raw);
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
                        self.apply_slash_chart(&analyze_clone, &item_clone, *source);
                    }
                }
            }
        }
    }

    /// Outline entries derived from the current body HTML. H1/H2/H3 +
    /// chart figures contribute; everything else (paragraphs, lists, …)
    /// is ignored.
    pub fn outline(&self) -> Vec<OutlineEntry> {
        let html = self.body_html.read().clone();
        parse_outline(&html)
    }

    pub fn outline_is_empty(&self) -> bool {
        self.outline().is_empty()
    }

    /// Read a chart figure's metadata back out of the body HTML — used
    /// by the outline-swap panel to know which source / type / data to
    /// offer for the picked chart.
    pub fn chart_meta(&self, chart_id: &str) -> Option<ChartMeta> {
        let html = self.body_html.read().clone();
        parse_chart_meta(&html, chart_id)
    }

    pub fn open_chart_swap(&mut self, id: &str) {
        self.outline_mode.set(OutlineMode::ChartTypeSwap {
            chart_id: id.to_string(),
        });
    }

    pub fn close_outline_swap(&mut self) {
        self.outline_mode.set(OutlineMode::Default);
    }

    /// Re-render an existing chart figure with a different chart type.
    /// Reads the figure's saved data attributes (via the parsed meta),
    /// rebuilds the inner HTML with the new type, and dispatches a JS
    /// replace-children op. The figure's data attributes themselves stay
    /// unchanged except for `data-type`, which the JS op updates inline.
    pub fn swap_chart_type(&mut self, chart_id: &str, new_type: ChartType) {
        let Some(meta) = self.chart_meta(chart_id) else {
            return;
        };
        let new_inner = build_chart_inner(
            chart_id,
            meta.source,
            new_type,
            &meta.analyze_name,
            &meta.item_title,
            &meta.item_meta,
            &meta.options,
            meta.respondent_count,
            meta.discussion_data.as_ref(),
            &meta.text_answers,
        );
        dispatch_swap_chart(chart_id.to_string(), new_type.as_token().to_string(), new_inner);
        self.close_outline_swap();
    }

    /// Trash icon click — remove the chart figure from the body.
    pub fn delete_chart(&mut self, chart_id: &str) {
        dispatch_delete_chart(chart_id.to_string());
    }

    // -- Autosave ----------------------------------------------------

    pub fn save_status(&self) -> SaveStatus {
        *self.status.read()
    }

    /// Called from every input handler. Compares the current edit
    /// buffers against `last_saved`; if anything actually changed, flips
    /// status to `Unsaved` and bumps `save_version` so the debounced
    /// effect picks up the change. A revert-to-saved input is a no-op
    /// (status stays `Saved`/`Idle`), matching post_edit semantics.
    pub fn mark_unsaved(&mut self) {
        let (saved_title, saved_subtitle, saved_body) = self.last_saved.peek().clone();
        let current_title = self.title.peek().clone();
        let current_subtitle = self.subtitle.peek().clone();
        let current_body = self.body_html.peek().clone();
        if current_title == saved_title
            && current_subtitle == saved_subtitle
            && current_body == saved_body
        {
            self.status.set(SaveStatus::Saved);
        } else {
            self.status.set(SaveStatus::Unsaved);
            *self.save_version.write() += 1;
        }
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
    let detail = use_loader(move || {
        let sid = space_id();
        let rid = report_id();
        async move {
            let report_id_typed: SpaceReportEntityType = rid.clone().into();
            let resp = crate::features::spaces::pages::report::controllers::get_report(
                sid.clone(),
                report_id_typed,
            )
            .await?;
            let analyzes =
                crate::features::spaces::pages::report::controllers::list_report_analyzes(
                    sid.clone(),
                )
                .await
                .map(|r| r.items)
                .unwrap_or_default();
            Ok::<_, crate::common::Error>(ReportDetail {
                id: resp.id,
                eyebrow: "Action · Report".to_string(),
                title: resp.title,
                subtitle: resp.description,
                blocks: resp.blocks,
                author: resp.author,
                created_at: resp.created_at,
                updated_at: resp.updated_at,
                analyzes,
                html_contents: resp.html_contents.unwrap_or_default(),
            })
        }
    })?;

    let snapshot = detail();
    // Edit-buffer signals seeded once from the loader snapshot. They are
    // the exception to the "no signal duplication" rule — the editor
    // needs writable local state independent of the server value until
    // `handle_save` persists it. The signals do NOT auto-resync on
    // `detail.restart()` (by design — that would clobber in-flight
    // edits).
    let title = use_signal(|| snapshot.title.clone());
    let subtitle = use_signal(|| snapshot.subtitle.clone());
    let body_html = use_signal(|| snapshot.html_contents.clone());
    let drawer = use_signal(|| DetailDrawerTarget::Closed);
    let picker_analyze_id = use_signal(String::new);
    let picker_source = use_signal(|| ActionSource::Poll);
    let outline_mode = use_signal(|| OutlineMode::Default);
    let slash = use_signal(|| Option::<SlashState>::None);

    // Autosave bookkeeping. `last_saved` is seeded from the same snapshot
    // so the very first `mark_unsaved` after mount only flips status when
    // the user actually diverges from the server value.
    let status = use_signal(|| SaveStatus::Idle);
    let save_version = use_signal(|| 0u64);
    let last_saved = use_signal(move || {
        (
            snapshot.title.clone(),
            snapshot.subtitle.clone(),
            snapshot.html_contents.clone(),
        )
    });

    let mut detail_for_save = detail;
    let handle_save = use_action(move || async move {
        let report_id_typed: SpaceReportEntityType = report_id().into();
        let req = crate::features::spaces::pages::report::controllers::UpdateReportRequest {
            title: Some(title.peek().clone()),
            description: Some(subtitle.peek().clone()),
            blocks: None,
            status: None,
            html_contents: Some(body_html.peek().clone()),
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

    // Debounced autosave — mirrors `post_edit`. Reactive deps (the four
    // signals) are read on the synchronous prefix so dependency tracking
    // sees them; the actual work runs in a spawned future that sleeps,
    // then re-checks `save_version` to detect "another edit landed
    // during the wait — bail and let the next tick handle it."
    let mut status_for_effect = status;
    let mut last_saved_for_effect = last_saved;
    let mut detail_for_effect = detail;
    use_effect(move || {
        let ver = save_version();
        let current_title = title();
        let current_subtitle = subtitle();
        let current_body = body_html();
        let saved = last_saved();
        if ver == 0 {
            return;
        }
        let report_id_for_save = report_id;
        let space_id_for_save = space_id;
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(
                AUTOSAVE_DEBOUNCE_SECS,
            ))
            .await;
            // A later edit bumped the version while we were sleeping —
            // skip this stale attempt; the newer effect run will handle
            // the latest content.
            if save_version() != ver {
                return;
            }
            let (saved_title, saved_subtitle, saved_body) = saved;
            if current_title == saved_title
                && current_subtitle == saved_subtitle
                && current_body == saved_body
            {
                return;
            }
            status_for_effect.set(SaveStatus::Saving);
            let report_id_typed: SpaceReportEntityType = report_id_for_save().into();
            let req = crate::features::spaces::pages::report::controllers::UpdateReportRequest {
                title: Some(current_title.clone()),
                description: Some(current_subtitle.clone()),
                blocks: None,
                status: None,
                html_contents: Some(current_body.clone()),
            };
            match crate::features::spaces::pages::report::controllers::update_report(
                space_id_for_save(),
                report_id_typed,
                req,
            )
            .await
            {
                Ok(_) => {
                    last_saved_for_effect.set((current_title, current_subtitle, current_body));
                    status_for_effect.set(SaveStatus::Saved);
                    detail_for_effect.restart();
                }
                Err(e) => {
                    crate::error!("report autosave failed: {e:?}");
                    status_for_effect.set(SaveStatus::Unsaved);
                }
            }
        });
    });

    let ctx = use_context_provider(move || UseReportDetailContext {
        detail,
        title,
        subtitle,
        body_html,
        drawer,
        picker_analyze_id,
        picker_source,
        outline_mode,
        slash,
        handle_save,
        status,
        save_version,
        last_saved,
    });

    Ok(ctx)
}

// ── Slash token parsing ────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParsedSlash {
    pub raw: String,
    pub level: u8,
    pub query: String,
    pub analyze_id: Option<String>,
    pub source: Option<ActionSource>,
}

fn parse_slash_token(raw: &str) -> Option<ParsedSlash> {
    if !raw.starts_with('/') {
        return None;
    }
    let body = raw.trim_start_matches('/');
    let parts: Vec<&str> = body.split(':').collect();
    match parts.as_slice() {
        [cmd] => Some(ParsedSlash {
            raw: raw.to_string(),
            level: 0,
            query: (*cmd).to_string(),
            analyze_id: None,
            source: None,
        }),
        [cmd, q] if *cmd == "data" => Some(ParsedSlash {
            raw: raw.to_string(),
            level: 1,
            query: (*q).to_string(),
            analyze_id: None,
            source: None,
        }),
        [cmd, aid, q] if *cmd == "data" => Some(ParsedSlash {
            raw: raw.to_string(),
            level: 2,
            query: (*q).to_string(),
            analyze_id: Some((*aid).to_string()),
            source: None,
        }),
        [cmd, aid, src, q] if *cmd == "data" => Some(ParsedSlash {
            raw: raw.to_string(),
            level: 3,
            query: (*q).to_string(),
            analyze_id: Some((*aid).to_string()),
            source: parse_source(src),
        }),
        _ => None,
    }
}

fn parse_source(s: &str) -> Option<ActionSource> {
    match s {
        "poll" => Some(ActionSource::Poll),
        "quiz" => Some(ActionSource::Quiz),
        "discussion" => Some(ActionSource::Discussion),
        "follow" => Some(ActionSource::Follow),
        _ => None,
    }
}

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

// ── HTML parsing for outline + chart meta ────────────────

fn parse_outline(html: &str) -> Vec<OutlineEntry> {
    // Two passes — headings and chart figures. We don't have full DOM
    // parsing here, but the editor produces well-formed `<h1>` / `<h2>`
    // / `<h3>` / `<figure>` tags so naïve regex is sufficient. Heading
    // ids may not exist (the editor doesn't auto-generate them), so we
    // index headings positionally and synthesize ids for outline jumps.
    //
    // Rust's `regex` crate doesn't support backreferences, so we run a
    // separate regex per heading level instead of `<h([1-3])>…</h\1>`.
    let mut entries: Vec<(usize, OutlineEntry)> = Vec::new();
    let mut counters = [0usize; 3];

    for level in 1..=3usize {
        let pattern = format!(r#"(?is)<h{level}\b[^>]*>(.*?)</h{level}>"#);
        let Ok(re) = Regex::new(&pattern) else {
            continue;
        };
        for caps in re.captures_iter(html) {
            let full = caps.get(0).unwrap();
            let inner = caps.get(1).unwrap().as_str();
            let text = strip_tags(inner).trim().to_string();
            if text.is_empty() {
                continue;
            }
            counters[level - 1] += 1;
            let kind = match level {
                1 => OutlineKind::H1,
                2 => OutlineKind::H2,
                _ => OutlineKind::H3,
            };
            let id = format!("h{}-{}", level, counters[level - 1]);
            entries.push((
                full.start(),
                OutlineEntry {
                    id,
                    kind,
                    label: text,
                },
            ));
        }
    }

    // Chart figures — capture id + item title.
    let figure_re = Regex::new(
        r#"(?is)<figure\b[^>]*\bid=['"]([^'"]+)['"][^>]*\bdata-item-title=['"]([^'"]*)['"][^>]*>"#,
    )
    .unwrap();
    for caps in figure_re.captures_iter(html) {
        let full = caps.get(0).unwrap();
        let id = caps.get(1).unwrap().as_str().to_string();
        let title_attr = caps.get(2).unwrap().as_str();
        let label = decode_attr(title_attr);
        entries.push((
            full.start(),
            OutlineEntry {
                id,
                kind: OutlineKind::Chart,
                label,
            },
        ));
    }

    entries.sort_by_key(|(pos, _)| *pos);
    // Dedupe by `id` — the body HTML can legitimately end up with two
    // figures sharing the same chart-id when the user copy-pastes a
    // chart inside the editor (the browser preserves the `id`
    // attribute verbatim). A duplicate Dioxus `key:` further down
    // panics the keyed-diff assertion, so we keep the FIRST occurrence
    // and tag subsequent duplicates with an `-N` suffix so the outline
    // still surfaces them as separate rows.
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    entries
        .into_iter()
        .map(|(_, mut e)| {
            let count = seen.entry(e.id.clone()).or_insert(0);
            if *count > 0 {
                e.id = format!("{}-{}", e.id, *count);
            }
            *count += 1;
            e
        })
        .collect()
}

fn parse_chart_meta(html: &str, chart_id: &str) -> Option<ChartMeta> {
    let escaped_id = regex::escape(chart_id);
    let pattern = format!(
        r#"(?is)<figure\b[^>]*\bid=['"]{escaped_id}['"][^>]*>"#
    );
    let re = Regex::new(&pattern).ok()?;
    let m = re.find(html)?;
    let opening = m.as_str();
    let source_str = read_attr(opening, "data-source")?;
    let type_str = read_attr(opening, "data-type")?;
    let source = parse_source(&source_str)?;
    let chart_type = parse_chart_type(&type_str)?;
    let analyze_name = read_attr(opening, "data-analyze-name").unwrap_or_default();
    let item_title = read_attr(opening, "data-item-title").unwrap_or_default();
    let item_meta = read_attr(opening, "data-meta").unwrap_or_default();
    let respondent_count: u32 = read_attr(opening, "data-respondent-count")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let options: Vec<ChartOption> = read_attr(opening, "data-options")
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    let discussion_data: Option<DiscussionData> = read_attr(opening, "data-discussion")
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| serde_json::from_str(&s).ok());
    let text_answers: Vec<String> = read_attr(opening, "data-answers")
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    Some(ChartMeta {
        chart_id: chart_id.to_string(),
        source,
        chart_type,
        analyze_name,
        item_title,
        item_meta,
        respondent_count,
        options,
        discussion_data,
        text_answers,
    })
}

fn parse_chart_type(s: &str) -> Option<ChartType> {
    match s {
        "bar" => Some(ChartType::Bar),
        "pie" => Some(ChartType::Pie),
        "table" => Some(ChartType::Table),
        "lda" => Some(ChartType::Lda),
        "tfidf" => Some(ChartType::TfIdf),
        "network" => Some(ChartType::Network),
        "textlist" => Some(ChartType::TextList),
        _ => None,
    }
}

/// Pull a `name='...'` or `name="..."` attribute value out of a tag's
/// opening string. Returns the HTML-decoded value. We have to try
/// single- and double-quoted patterns separately because JSON
/// attribute values (`data-options='[{"…":"…"}]'`) embed `"` in their
/// payload — a combined `[^'"]*` capture would stop at the first `"`.
fn read_attr(tag: &str, name: &str) -> Option<String> {
    let escaped_name = regex::escape(name);
    let single = format!(r#"\b{escaped_name}='([^']*)'"#);
    if let Some(value) = Regex::new(&single)
        .ok()
        .and_then(|re| re.captures(tag).and_then(|c| c.get(1).map(|m| m.as_str().to_string())))
    {
        return Some(decode_attr(&value));
    }
    let double = format!(r#"\b{escaped_name}="([^"]*)""#);
    Regex::new(&double)
        .ok()
        .and_then(|re| re.captures(tag).and_then(|c| c.get(1).map(|m| m.as_str().to_string())))
        .map(|s| decode_attr(&s))
}

fn decode_attr(s: &str) -> String {
    // The browser re-serializes single-quoted attributes as
    // double-quoted on every `innerHTML` read and encodes any embedded
    // `"` as `&quot;`. Our JSON payloads (`data-options`,
    // `data-discussion`) are full of `"`, so without this decode the
    // first chart-type swap loses every option. `&amp;` is decoded last
    // so a literal `&amp;quot;` round-trips to `&quot;` rather than `"`.
    s.replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn strip_tags(s: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(s, "").to_string()
}

// ── JS dispatchers ─────────────────────────────────────────

fn dispatch_insert_html(html: String) {
    // Range-API based insertion. `execCommand("insertHTML")` is
    // deprecated and unreliable when focus has bounced off the editor
    // (picker modal → close → editor.focus())– in that case the cached
    // selection is gone and the command becomes a no-op, so the user
    // can insert exactly one chart then the second click silently does
    // nothing. We instead grab the editor's current Range if it still
    // lives inside the contenteditable; otherwise fall back to a
    // collapsed range at the very end of the content, which guarantees
    // the chart lands SOMEWHERE rather than vanishing.
    let mut runner = dx_eval(
        r#"
        const html = await dioxus.recv();
        const editor = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!editor) { dioxus.send(null); return; }
        editor.focus();
        const sel = window.getSelection();
        let range;
        if (sel && sel.rangeCount > 0 && editor.contains(sel.getRangeAt(0).startContainer)) {
          range = sel.getRangeAt(0);
        } else {
          range = document.createRange();
          range.selectNodeContents(editor);
          range.collapse(false);
        }
        // Build a fragment from the HTML and insert it at the range.
        const tmp = document.createElement('div');
        tmp.innerHTML = html;
        const frag = document.createDocumentFragment();
        let lastNode = null;
        while (tmp.firstChild) {
          lastNode = tmp.firstChild;
          frag.appendChild(tmp.firstChild);
        }
        range.deleteContents();
        range.insertNode(frag);
        // Position caret after the inserted block so the next keystroke
        // continues right where the insertion ended (typically inside
        // the trailing <p><br></p>).
        if (lastNode) {
          const r = document.createRange();
          r.setStartAfter(lastNode);
          r.collapse(true);
          sel.removeAllRanges();
          sel.addRange(r);
        }
        editor.dispatchEvent(new Event('input', { bubbles: true }));
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!(html));
    dioxus::prelude::spawn(async move {
        let _ = runner.recv::<Option<()>>().await;
    });
}

/// Shared JS helper code: walks back from the current caret to find a
/// range that exactly covers the last `oldLen` characters and returns
/// it. Returns null when the walk can't find that much text behind the
/// caret without leaving the editor. Used by both slash-text replace
/// flows below.
const SLASH_HELPERS_JS: &str = r#"
function findSlashRange(editor, oldLen) {
    const sel = window.getSelection();
    if (!sel || sel.rangeCount === 0) return null;
    const caret = sel.getRangeAt(0);
    if (!editor.contains(caret.startContainer)) return null;
    // Caret end of the range to be replaced.
    const endContainer = caret.startContainer;
    const endOffset = caret.startOffset;
    // Walk backwards through text nodes only (skip `<figure
    // contenteditable=false>` and other element boundaries — the slash
    // chain is always inside one paragraph).
    let needed = oldLen;
    let startNode = endContainer;
    let startOffset = endOffset;
    while (needed > 0 && startNode) {
        if (startNode.nodeType === 3) {
            if (startOffset > 0) {
                const take = Math.min(startOffset, needed);
                startOffset -= take;
                needed -= take;
                if (needed === 0) break;
            }
            // Step to previous text node within the editor — walk up to
            // parent then to its previousSibling, descending into the
            // last text leaf there.
            let cursor = startNode;
            while (cursor && !cursor.previousSibling && cursor !== editor) {
                cursor = cursor.parentNode;
            }
            if (!cursor || cursor === editor) return null;
            let prev = cursor.previousSibling;
            // Skip non-text leaves (e.g. figure, br) — they don't
            // contribute to the slash token.
            while (prev && prev.nodeType !== 3) {
                if (prev.lastChild) {
                    prev = prev.lastChild;
                } else {
                    prev = prev.previousSibling;
                }
            }
            if (!prev || prev.nodeType !== 3) return null;
            startNode = prev;
            startOffset = (prev.textContent || "").length;
        } else {
            return null;
        }
    }
    if (needed > 0) return null;
    const range = document.createRange();
    range.setStart(startNode, startOffset);
    range.setEnd(endContainer, endOffset);
    return range;
}
"#;

fn dispatch_replace_slash_token(old_raw: String, new_raw: String) {
    let mut runner = dx_eval(&format!(
        r#"
        {helpers}
        const data = await dioxus.recv();
        const editor = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!editor) {{ dioxus.send(null); return; }}
        const r = findSlashRange(editor, data.old.length);
        if (!r) {{ dioxus.send(null); return; }}
        r.deleteContents();
        const textNode = document.createTextNode(data.new);
        r.insertNode(textNode);
        // Caret must land INSIDE the new text node so the editor's
        // slash watcher (which only parses when caret is in a text
        // node) re-emits the new `/data:` token. `setStartAfter` would
        // place the caret in the parent element and break detection.
        const finalRange = document.createRange();
        finalRange.setStart(textNode, (textNode.textContent || "").length);
        finalRange.collapse(true);
        const sel = window.getSelection();
        sel.removeAllRanges();
        sel.addRange(finalRange);
        editor.dispatchEvent(new Event('input', {{ bubbles: true }}));
        dioxus.send(null);
        "#,
        helpers = SLASH_HELPERS_JS
    ));
    let _ = runner.send(serde_json::json!({ "old": old_raw, "new": new_raw }));
    dioxus::prelude::spawn(async move {
        let _ = runner.recv::<Option<()>>().await;
    });
}

fn dispatch_replace_slash_and_insert(old_raw: String, figure_html: String) {
    let mut runner = dx_eval(&format!(
        r#"
        {helpers}
        const data = await dioxus.recv();
        const editor = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!editor) {{ dioxus.send(null); return; }}
        const r = findSlashRange(editor, data.old.length);
        if (!r) {{
          // Fall through: still insert the chart at the end so the user
          // doesn't lose the picked item even when the slash chain is
          // weirdly placed.
          const fallback = document.createRange();
          fallback.selectNodeContents(editor);
          fallback.collapse(false);
          const tmp = document.createElement('div');
          tmp.innerHTML = data.html;
          const frag = document.createDocumentFragment();
          let lastNode = null;
          while (tmp.firstChild) {{ lastNode = tmp.firstChild; frag.appendChild(tmp.firstChild); }}
          fallback.insertNode(frag);
          if (lastNode) {{
            const c = document.createRange();
            c.setStartAfter(lastNode);
            c.collapse(true);
            const sel = window.getSelection();
            sel.removeAllRanges();
            sel.addRange(c);
          }}
          editor.dispatchEvent(new Event('input', {{ bubbles: true }}));
          dioxus.send(null);
          return;
        }}
        r.deleteContents();
        const tmp = document.createElement('div');
        tmp.innerHTML = data.html;
        const frag = document.createDocumentFragment();
        let lastNode = null;
        while (tmp.firstChild) {{ lastNode = tmp.firstChild; frag.appendChild(tmp.firstChild); }}
        r.insertNode(frag);
        // Caret right after the inserted block (typically inside the
        // trailing <p><br></p>).
        if (lastNode) {{
          const c = document.createRange();
          c.setStartAfter(lastNode);
          c.collapse(true);
          const sel = window.getSelection();
          sel.removeAllRanges();
          sel.addRange(c);
        }}
        editor.dispatchEvent(new Event('input', {{ bubbles: true }}));
        dioxus.send(null);
        "#,
        helpers = SLASH_HELPERS_JS
    ));
    let _ = runner.send(serde_json::json!({ "old": old_raw, "html": figure_html }));
    dioxus::prelude::spawn(async move {
        let _ = runner.recv::<Option<()>>().await;
    });
}

fn dispatch_swap_chart(chart_id: String, type_token: String, new_inner: String) {
    let mut runner = dx_eval(
        r#"
        const data = await dioxus.recv();
        const editor = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!editor) { dioxus.send(null); return; }
        const figure = editor.querySelector('figure[data-chart-id="' + data.id + '"]');
        if (!figure) { dioxus.send(null); return; }
        // Preserve the user-edited caption across chart-type swap.
        const oldCap = figure.querySelector(':scope > .report-detail__chart-caption');
        const savedCaption = oldCap ? oldCap.innerHTML : null;
        figure.innerHTML = data.inner;
        figure.setAttribute('data-type', data.type);
        if (savedCaption !== null) {
            const newCap = figure.querySelector(':scope > .report-detail__chart-caption');
            if (newCap) newCap.innerHTML = savedCaption;
        }
        editor.dispatchEvent(new Event('input', { bubbles: true }));
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!({
        "id": chart_id,
        "type": type_token,
        "inner": new_inner,
    }));
    dioxus::prelude::spawn(async move {
        let _ = runner.recv::<Option<()>>().await;
    });
}

fn dispatch_delete_chart(chart_id: String) {
    let mut runner = dx_eval(
        r#"
        const data = await dioxus.recv();
        const editor = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!editor) { dioxus.send(null); return; }
        const figure = editor.querySelector('figure[data-chart-id="' + data.id + '"]');
        if (!figure) { dioxus.send(null); return; }
        figure.remove();
        editor.dispatchEvent(new Event('input', { bubbles: true }));
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!({ "id": chart_id }));
    dioxus::prelude::spawn(async move {
        let _ = runner.recv::<Option<()>>().await;
    });
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

