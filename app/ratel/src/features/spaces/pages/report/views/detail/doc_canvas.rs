use super::i18n::ReportDetailTranslate;
use super::slash_popup::SlashPopup;
use crate::features::spaces::pages::report::types::*;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Focus a newly inserted block and place the caret at its start.
/// Used after `insert_text_after` so the author lands inside the fresh
/// paragraph instead of staying in the previous block. Defers to the
/// next frame so Dioxus has mounted the new contenteditable before we
/// query for it.
fn focus_block(block_id: &str) {
    let mut runner = document::eval(
        r#"
        const id = await dioxus.recv();
        requestAnimationFrame(() => {
            const el = document.getElementById(id);
            if (!el) return;
            el.focus();
            const range = document.createRange();
            const sel = window.getSelection();
            range.selectNodeContents(el);
            range.collapse(true);
            sel.removeAllRanges();
            sel.addRange(range);
        });
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!(block_id));
}

/// Focus a block and place the caret at the END of its content. Used
/// when a Backspace-collapse merges the current empty block into the
/// previous one — caret should land where the user was about to type,
/// not at the start.
fn focus_block_end(block_id: &str) {
    let mut runner = document::eval(
        r#"
        const id = await dioxus.recv();
        requestAnimationFrame(() => {
            const el = document.getElementById(id);
            if (!el) return;
            el.focus();
            const range = document.createRange();
            const sel = window.getSelection();
            range.selectNodeContents(el);
            range.collapse(false);
            sel.removeAllRanges();
            sel.addRange(range);
        });
        dioxus.send(null);
        "#,
    );
    let _ = runner.send(serde_json::json!(block_id));
}

/// Return true iff the contenteditable with id `block_id` is empty AND
/// the current selection's caret sits at offset 0 inside it. Backspace
/// in that state should collapse the block into the previous one — in
/// any other state we leave the native Backspace behavior alone.
async fn is_block_empty_at_start(block_id: &str) -> bool {
    let mut runner = document::eval(
        r#"
        const id = await dioxus.recv();
        const el = document.getElementById(id);
        if (!el) { dioxus.send(false); return; }
        const text = (el.textContent || "").replace(/​/g, "");
        const empty = text.trim().length === 0;
        const sel = window.getSelection();
        let atStart = true;
        if (sel && sel.rangeCount > 0) {
            const r = sel.getRangeAt(0);
            // Caret is "at start" when both the anchor and the
            // selection start sit at offset 0 inside (or before) the
            // contenteditable — covers both empty text node and
            // empty-element edge cases.
            atStart = r.startOffset === 0 && r.collapsed;
        }
        dioxus.send(empty && atStart);
        "#,
    );
    if runner.send(serde_json::json!(block_id)).is_err() {
        return false;
    }
    runner.recv::<bool>().await.unwrap_or(false)
}

fn jump_into_first_block(ctx: &mut UseReportDetailContext) {
    let target_id = ctx
        .first_editable_block_id()
        .unwrap_or_else(|| ctx.append_text_block());
    focus_block(&target_id);
}

async fn read_block_text(block_id: &str) -> Option<String> {
    let mut runner = document::eval(
        r#"
        const id = await dioxus.recv();
        const el = document.getElementById(id);
        if (!el) { dioxus.send(null); return; }
        dioxus.send(el.textContent || "");
        "#,
    );
    runner.send(serde_json::json!(block_id)).ok()?;
    runner.recv::<Option<String>>().await.ok().flatten()
}

/// Read the caret's position INSIDE the scrolling doc container.
/// Returns `(x, y, placement)` where:
/// - `x` / `y` are pixel offsets relative to `.report-detail__doc`'s
///   scroll origin, so a `position: absolute` popup rendered inside
///   the doc tracks the caret as the user scrolls the doc.
/// - `placement` is `"below"` (popup top sits right under the caret)
///   or `"above"` (popup bottom sits right above the caret) — flipped
///   when there isn't enough room beneath the caret in the viewport.
async fn read_caret_pos() -> Option<(f64, f64, String)> {
    let mut runner = document::eval(
        r#"
        const ESTIMATED_POPUP_H = 340;
        const PAD = 6;
        const sel = window.getSelection();
        const doc = document.querySelector(".report-detail__doc");
        if (!sel || sel.rangeCount === 0 || !doc) { dioxus.send(null); return; }
        let rect = null;
        const range = sel.getRangeAt(0).cloneRange();
        range.collapse(false);
        const r = range.getBoundingClientRect();
        if (r && (r.left || r.top || r.bottom)) rect = r;
        if (!rect) {
            // Empty contenteditable fallback — use the anchor element rect.
            const node = sel.anchorNode;
            const el = node && node.nodeType === 1 ? node : (node ? node.parentElement : null);
            if (!el) { dioxus.send(null); return; }
            rect = el.getBoundingClientRect();
        }
        const docRect = doc.getBoundingClientRect();
        // Flip above the caret if there isn't enough room below in the
        // *viewport* — even though we render inside the doc, the doc's
        // scroll area is bounded by its visible height.
        const spaceBelow = window.innerHeight - rect.bottom;
        const placement = spaceBelow >= ESTIMATED_POPUP_H + PAD ? "below" : "above";
        const yViewport = placement === "below" ? rect.bottom + PAD : rect.top - PAD;
        const x = rect.left - docRect.left + doc.scrollLeft;
        const y = yViewport - docRect.top + doc.scrollTop;
        dioxus.send([x, y, placement]);
        "#,
    );
    runner
        .recv::<Option<(f64, f64, String)>>()
        .await
        .ok()
        .flatten()
}

/// Light-weight parser for the trailing slash token in a block's
/// innerText. Returns `None` when there is no active `/...` token at
/// the end of the text.
struct ParsedSlash {
    raw: String,
    level: u8,
    query: String,
    analyze_id: Option<String>,
    source: Option<ActionSource>,
}

fn parse_slash_token(text: &str) -> Option<ParsedSlash> {
    // Take the substring after the last `/` if it's still part of the
    // current word (no whitespace after the slash). Mockup behaviour.
    let last_slash = text.rfind('/')?;
    let rest = &text[last_slash..];
    if rest.chars().skip(1).any(char::is_whitespace) {
        return None;
    }
    let body = rest.trim_start_matches('/');
    let parts: Vec<&str> = body.split(':').collect();
    match parts.as_slice() {
        [cmd] => Some(ParsedSlash {
            raw: rest.to_string(),
            level: 0,
            query: (*cmd).to_string(),
            analyze_id: None,
            source: None,
        }),
        [cmd, q] if *cmd == "data" => Some(ParsedSlash {
            raw: rest.to_string(),
            level: 1,
            query: (*q).to_string(),
            analyze_id: None,
            source: None,
        }),
        [cmd, aid, q] if *cmd == "data" => Some(ParsedSlash {
            raw: rest.to_string(),
            level: 2,
            query: (*q).to_string(),
            analyze_id: Some((*aid).to_string()),
            source: None,
        }),
        [cmd, aid, src, q] if *cmd == "data" => Some(ParsedSlash {
            raw: rest.to_string(),
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

/// Main document column — Notion-style block editor.
/// Title input + subtitle input + a vertical block stack
/// (H1/H2/H3/Text/Chart). Matches the mockup at
/// `assets/design/reports/reports-edit.html`. The bottom format-toolbar
/// (rendered by the page assembly) drives formatting commands.
#[component]
pub fn DocCanvas() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let mut ctx = use_report_detail_context();

    rsx! {
        div { class: "report-detail__doc",
            // Popup rendered INSIDE the scrolling doc so it scrolls
            // along with the text (`position: absolute` + doc-relative
            // coords from `read_caret_pos`).
            SlashPopup {}
            div { class: "report-detail__doc-inner",
                div { class: "report-detail__cover",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                        polyline { points: "14 2 14 8 20 8" }
                    }
                    span { "{ctx.eyebrow()}" }
                }
                input {
                    class: "report-detail__title",
                    placeholder: tr.title_placeholder,
                    value: "{ctx.title_value()}",
                    oninput: move |e| ctx.title.set(e.value()),
                    // Save on blur — Dioxus event handlers can be async,
                    // so this maps the focusout into an `update_report`
                    // call without firing on every keystroke. Same
                    // pattern on subtitle / block contenteditables below.
                    onfocusout: move |_| ctx.handle_save.call(),
                    // Pressing Enter in the title shouldn't submit a
                    // form (we're not in one) — just move focus into
                    // the first editable body block instead.
                    onkeydown: move |e| {
                        if matches!(e.key(), Key::Enter) {
                            e.prevent_default();
                            jump_into_first_block(&mut ctx);
                        }
                    },
                }
                input {
                    class: "report-detail__subtitle",
                    placeholder: tr.subtitle_placeholder,
                    value: "{ctx.subtitle_value()}",
                    oninput: move |e| ctx.subtitle.set(e.value()),
                    onfocusout: move |_| ctx.handle_save.call(),
                    onkeydown: move |e| {
                        if matches!(e.key(), Key::Enter) {
                            e.prevent_default();
                            jump_into_first_block(&mut ctx);
                        }
                    },
                }
                div { class: "report-detail__blocks",
                    for block in ctx.blocks_list() {
                        DocBlock { key: "{block.id()}", block: block.clone() }
                    }
                }
            }
        }
    }
}

/// One rendered block — branches on `ReportBlock` to pick the matching
/// markup. Headings/text are contenteditable with a per-block
/// `oninput` that detects trailing `/data...` tokens and surfaces them
/// as `SlashState` via context. Caret rect is read via
/// `dioxus::document::eval` so the popup anchors right under the caret.
#[component]
fn DocBlock(block: ReportBlock) -> Element {
    let mut ctx = use_report_detail_context();
    let id = block.id().to_string();
    let id_for_input = id.clone();

    // Reading the caret rect requires DOM access — eval a tiny JS
    // snippet, await the (x, y, placement) tuple, then write to context.
    let oninput = move |evt: Event<FormData>| {
        let text = evt.value();
        let block_id = id_for_input.clone();
        async move {
            match parse_slash_token(&text) {
                Some(parsed) => {
                    let (cx, cy, placement) = read_caret_pos().await.unwrap_or_else(|| {
                        let prev = ctx.slash.peek().clone();
                        prev.map(|s| (s.caret_x, s.caret_y, s.placement))
                            .unwrap_or((160.0, 220.0, "below".to_string()))
                    });
                    let prev_idx = ctx
                        .slash
                        .peek()
                        .as_ref()
                        .map(|s| (s.level, s.selected_index))
                        .filter(|(lvl, _)| *lvl == parsed.level)
                        .map(|(_, idx)| idx)
                        .unwrap_or(0);
                    ctx.slash.set(Some(SlashState {
                        block_id: block_id.clone(),
                        raw: parsed.raw,
                        level: parsed.level,
                        query: parsed.query,
                        analyze_id: parsed.analyze_id,
                        source: parsed.source,
                        caret_x: cx,
                        caret_y: cy,
                        placement,
                        selected_index: prev_idx,
                    }));
                }
                None => {
                    if ctx.slash.peek().is_some() {
                        ctx.slash.set(None);
                    }
                }
            }
        }
    };

    // Keyboard handling falls into three lanes:
    // 1. Slash popup open → arrow/Enter/Escape navigate the popup.
    // 2. Enter (no shift) → break out into a fresh Text block below.
    // 3. Backspace at start of an empty block → collapse into the
    //    previous editable block. The "empty + at start" check is an
    //    async DOM probe so we run it from an async event handler;
    //    we deliberately do NOT prevent_default for Backspace, because
    //    empty contenteditables drop the keystroke anyway and a stray
    //    prevent_default would block real character deletion in the
    //    non-empty case.
    let id_for_keydown = id.clone();
    let onkeydown = move |evt: Event<KeyboardData>| {
        let block_id = id_for_keydown.clone();
        let key = evt.key();
        let shift = evt.modifiers().shift();
        let slash_open = ctx.slash.peek().is_some();

        // Sync `prevent_default` calls — must happen before the await
        // boundary or the browser will fire the native action first.
        if slash_open {
            match key {
                Key::Escape | Key::ArrowDown | Key::ArrowUp | Key::Enter => {
                    evt.prevent_default();
                }
                _ => {}
            }
        } else if matches!(key, Key::Enter) && !shift {
            evt.prevent_default();
        }

        async move {
            if slash_open {
                match key {
                    Key::Escape => ctx.close_slash(),
                    Key::ArrowDown => ctx.move_slash_selection(1),
                    Key::ArrowUp => ctx.move_slash_selection(-1),
                    Key::Enter => ctx.apply_slash_selected(),
                    _ => {}
                }
                return;
            }
            match key {
                Key::Enter if !shift => {
                    let new_id = ctx.insert_text_after(&block_id);
                    focus_block(&new_id);
                }
                Key::Backspace => {
                    if is_block_empty_at_start(&block_id).await {
                        if let Some(prev_id) = ctx.collapse_block(&block_id) {
                            focus_block_end(&prev_id);
                        }
                    }
                }
                _ => {}
            }
        }
    };

    let id_for_blur = id.clone();
    let onfocusout = move |evt: Event<FocusData>| {
        let block_id = id_for_blur.clone();
        let _ = evt;
        async move {
            if let Some(text) = read_block_text(&block_id).await {
                ctx.update_block_text(&block_id, text);
            }
            ctx.handle_save.call();
        }
    };

    match block {
        ReportBlock::H1 { id, text } => rsx! {
            div { class: "report-detail__block", "data-type": "h1",
                h1 {
                    id: "{id}",
                    class: "report-detail__block-h1",
                    contenteditable: "true",
                    "spellcheck": "false",
                    oninput,
                    onkeydown,
                    onfocusout,
                    "{text}"
                }
            }
        },
        ReportBlock::H2 { id, text } => rsx! {
            div { class: "report-detail__block", "data-type": "h2",
                h2 {
                    id: "{id}",
                    class: "report-detail__block-h2",
                    contenteditable: "true",
                    "spellcheck": "false",
                    oninput,
                    onkeydown,
                    onfocusout,
                    "{text}"
                }
            }
        },
        ReportBlock::H3 { id, text } => rsx! {
            div { class: "report-detail__block", "data-type": "h3",
                h3 {
                    id: "{id}",
                    class: "report-detail__block-h3",
                    contenteditable: "true",
                    "spellcheck": "false",
                    oninput,
                    onkeydown,
                    onfocusout,
                    "{text}"
                }
            }
        },
        ReportBlock::Text { id, html } => rsx! {
            div { class: "report-detail__block", "data-type": "text",
                div {
                    id: "{id}",
                    class: "report-detail__block-text",
                    contenteditable: "true",
                    "spellcheck": "false",
                    oninput,
                    onkeydown,
                    onfocusout,
                    dangerous_inner_html: "{html}",
                }
            }
        },
        ReportBlock::Chart {
            id,
            source,
            chart_type,
            analyze_name,
            item_title,
            meta,
        } => rsx! {
            ChartBlock {
                id,
                source,
                chart_type,
                analyze_name,
                item_title,
                meta,
            }
        },
    }
}

#[component]
fn ChartBlock(
    id: String,
    source: ActionSource,
    chart_type: ChartType,
    analyze_name: String,
    item_title: String,
    meta: String,
) -> Element {
    let mut ctx = use_report_detail_context();
    let src_token = source.as_token();
    let type_token = chart_type.as_token();
    let src_word = match source {
        ActionSource::Poll => "POLL",
        ActionSource::Quiz => "QUIZ",
        ActionSource::Discussion => "DISCUSSION",
        ActionSource::Follow => "FOLLOW",
    };
    let type_word = match chart_type {
        ChartType::Bar => "BAR",
        ChartType::Pie => "PIE",
        ChartType::Table => "TABLE",
        ChartType::Lda => "LDA",
        ChartType::TfIdf => "TF-IDF",
        ChartType::Network => "NETWORK",
    };
    let src_label = format!("{src_word} · {type_word}");
    let id_for_swap = id.clone();
    let id_for_delete = id.clone();
    rsx! {
        figure {
            id: "{id}",
            class: "report-detail__chart-block",
            "data-source": src_token,
            "data-type": type_token,
            div { class: "report-detail__chart-top",
                span { class: "report-detail__chart-badge",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "18",
                            y1: "20",
                            x2: "18",
                            y2: "10",
                        }
                        line {
                            x1: "12",
                            y1: "20",
                            x2: "12",
                            y2: "4",
                        }
                        line {
                            x1: "6",
                            y1: "20",
                            x2: "6",
                            y2: "14",
                        }
                    }
                    "{src_label}"
                }
                span { class: "report-detail__chart-title", "{item_title}" }
                div { class: "report-detail__chart-actions",
                    button {
                        class: "report-detail__chart-action",
                        "aria-label": "차트 종류 변경",
                        r#type: "button",
                        onclick: move |_| ctx.open_chart_swap(&id_for_swap),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                    }
                    button {
                        class: "report-detail__chart-action report-detail__chart-action--danger",
                        "aria-label": "차트 삭제",
                        r#type: "button",
                        onclick: move |_| ctx.remove_block(&id_for_delete),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" }
                            path { d: "M10 11v6M14 11v6" }
                        }
                    }
                }
            }
            div { class: "report-detail__chart-meta",
                span { class: "report-detail__chart-meta-key", "분석:" }
                span { class: "report-detail__chart-meta-val", "{analyze_name}" }
                span { class: "report-detail__chart-meta-sep", "·" }
                span { class: "report-detail__chart-meta-val", "{meta}" }
            }
            ChartCanvas { chart_type }
        }
    }
}

/// Visual body of a chart block — branches on the chart_type.
#[component]
fn ChartCanvas(chart_type: ChartType) -> Element {
    match chart_type {
        ChartType::Bar => rsx! {
            div { class: "report-detail__chart-canvas report-detail__chart-canvas--bar",
                for h in [80, 56, 42, 64, 38].iter() {
                    div {
                        class: "report-detail__chart-bar",
                        style: "height: {h}%",
                    }
                }
            }
        },
        ChartType::Pie => rsx! {
            div { class: "report-detail__chart-canvas report-detail__chart-canvas--pie",
                div { class: "report-detail__chart-pie" }
                div { class: "report-detail__chart-pie-legend",
                    div { class: "report-detail__chart-pie-row",
                        span { class: "report-detail__chart-pie-swatch report-detail__chart-pie-swatch--a" }
                        span { "팔로워" }
                        strong { "65%" }
                    }
                    div { class: "report-detail__chart-pie-row",
                        span { class: "report-detail__chart-pie-swatch report-detail__chart-pie-swatch--b" }
                        span { "비팔로워" }
                        strong { "35%" }
                    }
                }
            }
        },
        ChartType::Lda => rsx! {
            LdaCanvas {}
        },
        ChartType::TfIdf => rsx! {
            TfIdfCanvas {}
        },
        ChartType::Network => rsx! {
            NetworkCanvas {}
        },
        ChartType::Table => rsx! {
            div { class: "report-detail__chart-canvas report-detail__chart-canvas--table",
                table { class: "report-detail__chart-table",
                    thead {
                        tr {
                            th { "옵션" }
                            th { "응답 수" }
                            th { "비율" }
                        }
                    }
                    tbody {
                        for (opt , n , pct) in [
                            ("탄소 상쇄 정책", "800", "64%"),
                            ("재생에너지 확대", "280", "22%"),
                            ("규제 강화", "120", "10%"),
                            ("기타", "48", "4%"),
                        ]
                            .iter()
                        {
                            tr {
                                td { "{opt}" }
                                td { "{n}" }
                                td { "{pct}" }
                            }
                        }
                    }
                }
            }
        },
    }
}

/// LDA topic block — mirrors the mockup's `insertLdaBlock` viz: each
/// row is one discovered topic with a weight bar and the top-5
/// keywords (with their TF inside the topic in italics).
#[component]
fn LdaCanvas() -> Element {
    // Per-topic mock data: (weight%, [(keyword, weight)])
    let topics: [(u32, [(&str, f32); 5]); 5] = [
        (
            32,
            [
                ("탄소", 0.18),
                ("상쇄", 0.14),
                ("정책", 0.11),
                ("시장", 0.09),
                ("거래", 0.07),
            ],
        ),
        (
            24,
            [
                ("재생", 0.16),
                ("에너지", 0.13),
                ("보조금", 0.10),
                ("전력", 0.08),
                ("태양광", 0.06),
            ],
        ),
        (
            18,
            [
                ("규제", 0.15),
                ("법안", 0.12),
                ("기준", 0.09),
                ("감축", 0.08),
                ("의무", 0.06),
            ],
        ),
        (
            15,
            [
                ("협약", 0.13),
                ("국제", 0.10),
                ("파리", 0.09),
                ("목표", 0.07),
                ("공조", 0.06),
            ],
        ),
        (
            11,
            [
                ("시민", 0.12),
                ("교육", 0.10),
                ("참여", 0.08),
                ("홍보", 0.07),
                ("캠페인", 0.05),
            ],
        ),
    ];
    let max_w = topics.iter().map(|(w, _)| *w).max().unwrap_or(1) as f32;
    rsx! {
        div { class: "report-detail__chart-canvas report-detail__chart-canvas--lda",
            for (i , (weight , kws)) in topics.iter().enumerate() {
                {
                    let bar_pct = (*weight as f32 / max_w * 100.0) as u32;
                    rsx! {
                        div { key: "{i}", class: "report-detail__lda-topic",
                            div { class: "report-detail__lda-topic-head",
                                span { class: "report-detail__lda-topic-id", "토픽 {i + 1}" }
                                span { class: "report-detail__lda-topic-weight", "{weight}%" }
                            }
                            div { class: "report-detail__lda-topic-bar",
                                div { style: "width: {bar_pct}%" }
                            }
                            div { class: "report-detail__lda-keywords",
                                for (w , pr) in kws.iter() {
                                    span { key: "{w}", class: "report-detail__lda-kw",
                                        "{w}"
                                        em { "{pr:.2}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// TF-IDF ranked-term table — mirrors `insertTfidfBlock`. Ten rows of
/// (rank, term, score). Reuses the table rendering style of the
/// generic Table chart.
#[component]
fn TfIdfCanvas() -> Element {
    let terms: [(&str, f32); 10] = [
        ("탄소", 0.842),
        ("상쇄", 0.731),
        ("정책", 0.668),
        ("크레딧", 0.592),
        ("거래", 0.547),
        ("배출", 0.518),
        ("시장", 0.476),
        ("감축", 0.441),
        ("인증", 0.402),
        ("표준", 0.385),
    ];
    rsx! {
        div { class: "report-detail__chart-canvas report-detail__chart-canvas--tfidf",
            table { class: "report-detail__chart-table",
                thead {
                    tr {
                        th { class: "report-detail__tfidf-rank-h", "Rank" }
                        th { "Term" }
                        th { class: "report-detail__tfidf-score-h", "TF-IDF" }
                    }
                }
                tbody {
                    for (i , (term , score)) in terms.iter().enumerate() {
                        tr { key: "{term}",
                            td { class: "report-detail__tfidf-rank", "{i + 1}" }
                            td { "{term}" }
                            td { class: "report-detail__tfidf-score", "{score:.3}" }
                        }
                    }
                }
            }
        }
    }
}

/// Text-network graph — mirrors `insertNetworkBlock`. Nodes laid out
/// at the same coordinates as the mockup so visual debugging matches
/// the HTML reference 1:1.
#[component]
fn NetworkCanvas() -> Element {
    let nodes: [(f32, f32, f32, &str); 8] = [
        (110.0, 80.0, 22.0, "탄소"),
        (230.0, 110.0, 18.0, "정책"),
        (320.0, 70.0, 14.0, "상쇄"),
        (160.0, 180.0, 16.0, "거래"),
        (280.0, 200.0, 12.0, "시장"),
        (380.0, 150.0, 10.0, "규제"),
        (70.0, 220.0, 9.0, "감축"),
        (420.0, 240.0, 8.0, "배출"),
    ];
    let edges: [(usize, usize); 11] = [
        (0, 1),
        (0, 2),
        (1, 2),
        (1, 3),
        (1, 4),
        (3, 4),
        (2, 5),
        (4, 5),
        (3, 6),
        (4, 7),
        (5, 7),
    ];
    rsx! {
        div { class: "report-detail__chart-canvas report-detail__chart-canvas--network",
            svg { class: "report-detail__network-svg", view_box: "0 0 500 280",
                for (i , (a , b)) in edges.iter().enumerate() {
                    line {
                        key: "e-{i}",
                        class: "report-detail__network-edge",
                        x1: "{nodes[*a].0}",
                        y1: "{nodes[*a].1}",
                        x2: "{nodes[*b].0}",
                        y2: "{nodes[*b].1}",
                    }
                }
                // Nodes are rendered as two passes — circle layer
                // first, label layer on top — so each `for` loop owns a
                // single keyed root and Dioxus's "keys only on first
                // node in block" rule is satisfied.
                for (x , y , r , label) in nodes.iter() {
                    {
                        let opacity = (0.35 + r / 40.0).min(1.0);
                        rsx! {
                            circle {
                                key: "n-{label}",
                                class: "report-detail__network-node",
                                cx: "{x}",
                                cy: "{y}",
                                r: "{r}",
                                style: "fill: rgba(168, 85, 247, {opacity}); stroke: rgba(168, 85, 247, 0.7);",
                            }
                        }
                    }
                }
                for (x , y , _ , label) in nodes.iter() {
                    text {
                        key: "t-{label}",
                        class: "report-detail__network-label",
                        x: "{x}",
                        y: "{y + 4.0}",
                        "{label}"
                    }
                }
            }
            div { class: "report-detail__network-stats",
                span {
                    strong { "18" }
                    "노드"
                }
                span {
                    strong { "42" }
                    "엣지"
                }
                span {
                    strong { "0.62" }
                    "모듈러리티"
                }
                span {
                    strong { "0.31" }
                    "밀도"
                }
            }
        }
    }
}
