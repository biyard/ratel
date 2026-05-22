//! Chart `<figure>` HTML builders — used to inject inline chart blocks
//! into the rich-text editor's contenteditable area.
//!
//! Each generated figure:
//! - is `contenteditable="false"` so the caret skips it during typing,
//! - carries every piece of data needed to re-render the viz (so chart
//!   type swaps don't need to round-trip back to the server),
//! - exposes `data-act="swap"` / `data-act="delete"` buttons that the
//!   page-level JS hooks via event delegation,
//! - shares CSS classes with the prior block-based renderer so the
//!   existing styling in `main.css` keeps working unchanged.

use crate::features::spaces::pages::report::types::{
    ActionSource, Analyze, AnalyzeItem, ChartOption, ChartType, DiscussionData,
};
use std::fmt::Write;

/// Build the entire `<figure>` for one chart block ready to be passed
/// to `document.execCommand("insertHTML", …)`.
pub fn build_chart_figure(
    chart_id: &str,
    source: ActionSource,
    chart_type: ChartType,
    analyze: &Analyze,
    item: &AnalyzeItem,
) -> String {
    let mut out = String::with_capacity(1024);
    let src_token = source.as_token();
    let type_token = chart_type.as_token();
    let meta_text = if item.meta.is_empty() {
        analyze.name.clone()
    } else {
        format!("{} · {}", analyze.name, item.meta)
    };

    // Serialize options + optional discussion data + text_answers into
    // HTML-attribute-safe JSON. We use single-quoted attributes
    // everywhere below so the JSON's `"` doesn't collide; the helper
    // escapes the only character that could break a single-quoted
    // attribute (`'`).
    let options_json =
        attr_safe_json(&serde_json::to_string(&item.options).unwrap_or_else(|_| "[]".to_string()));
    let discussion_json = item
        .discussion_data
        .as_ref()
        .and_then(|d| serde_json::to_string(d).ok())
        .map(|s| attr_safe_json(&s))
        .unwrap_or_default();
    let answers_json = if item.text_answers.is_empty() {
        String::new()
    } else {
        attr_safe_json(&serde_json::to_string(&item.text_answers).unwrap_or_else(|_| "[]".to_string()))
    };

    // ── Figure root ─────────────────────────────────────
    write!(
        out,
        r#"<figure id='{chart_id}' class="report-detail__chart-block" "#,
        chart_id = html_attr(chart_id),
    )
    .ok();
    write!(
        out,
        r#"data-source="{src_token}" data-type="{type_token}" data-chart-id='{chart_id}' "#,
        src_token = src_token,
        type_token = type_token,
        chart_id = html_attr(chart_id),
    )
    .ok();
    write!(
        out,
        r#"data-analyze-id='{aid}' data-analyze-name='{aname}' "#,
        aid = html_attr(&analyze.id),
        aname = html_attr(&analyze.name),
    )
    .ok();
    write!(
        out,
        r#"data-item-id='{iid}' data-item-title='{ititle}' data-meta='{m}' "#,
        iid = html_attr(&item.id),
        ititle = html_attr(&item.title),
        m = html_attr(&item.meta),
    )
    .ok();
    write!(
        out,
        r#"data-respondent-count="{rc}" data-options='{opts}' data-discussion='{disc}' data-answers='{ans}' "#,
        rc = item.respondent_count,
        opts = options_json,
        disc = discussion_json,
        ans = answers_json,
    )
    .ok();
    out.push_str(r#"contenteditable="false">"#);

    // ── Top strip: title + actions ─────────────────────
    // The source · type badge (e.g. "DISCUSSION · LDA") was dropped:
    // the meta line below already names the analyze and source, so the
    // badge was redundant chrome above the title.
    out.push_str(r#"<div class="report-detail__chart-top">"#);
    write!(
        out,
        r#"<span class="report-detail__chart-title">{}</span>"#,
        html_text(&item.title)
    )
    .ok();
    out.push_str(r#"<div class="report-detail__chart-actions">"#);
    // Hide the chart-type swap button for single-mode charts
    // (currently `TextList`): there's no alternate rendering to swap
    // to, so the gear icon would land on an empty panel.
    if !chart_type.is_single_mode() {
        push_action_button(&mut out, chart_id, "swap", SVG_SETTINGS, "Change chart type");
    }
    push_action_button(
        &mut out,
        chart_id,
        "delete",
        SVG_TRASH,
        "Delete chart",
    );
    out.push_str("</div></div>");

    // ── Meta line ──────────────────────────────────────
    write!(
        out,
        r#"<div class="report-detail__chart-meta">{}</div>"#,
        html_text(&meta_text)
    )
    .ok();

    // ── Viz body ───────────────────────────────────────
    render_chart_viz(
        &mut out,
        chart_type,
        &item.options,
        item.respondent_count,
        item.discussion_data.as_ref(),
        &item.text_answers,
    );

    // ── Per-item source / citation line ────────────────
    // Editable figcaption nested inside the otherwise-non-editable
    // figure. The empty `&nbsp;` keeps the line collapsed when blank
    // (without it the browser swallows the empty contenteditable).
    push_chart_caption(&mut out);

    out.push_str("</figure>");
    out
}

fn push_chart_caption(out: &mut String) {
    // Editable bottom caption — typed by the author in the style of
    // a publishable figure label (e.g. "그림 9. 성별에 따른 응답 비중").
    // `contenteditable="true"` overrides the figure's outer false so
    // the caption is the only editable region inside the figure.
    // The element is intentionally empty so the CSS `:empty::before`
    // placeholder hint is visible until the user types a caption.
    out.push_str(
        r#"<figcaption class="report-detail__chart-caption" contenteditable="true" data-placeholder="캡션 (예: 그림 1. 응답 비중)"></figcaption>"#,
    );
}

/// Re-render just the viz body + top badge + meta line for an existing
/// figure when the chart_type changes. The returned string contains
/// every CHILD of the figure (top strip, meta, viz) ready to be set as
/// `figure.innerHTML` from JS.
#[allow(clippy::too_many_arguments)]
pub fn build_chart_inner(
    chart_id: &str,
    source: ActionSource,
    chart_type: ChartType,
    analyze_name: &str,
    item_title: &str,
    item_meta: &str,
    options: &[ChartOption],
    respondent_count: u32,
    discussion: Option<&DiscussionData>,
    text_answers: &[String],
) -> String {
    let mut out = String::with_capacity(512);
    let meta_text = if item_meta.is_empty() {
        analyze_name.to_string()
    } else {
        format!("{} · {}", analyze_name, item_meta)
    };

    out.push_str(r#"<div class="report-detail__chart-top">"#);
    write!(
        out,
        r#"<span class="report-detail__chart-title">{}</span>"#,
        html_text(item_title)
    )
    .ok();
    out.push_str(r#"<div class="report-detail__chart-actions">"#);
    if !chart_type.is_single_mode() {
        push_action_button(&mut out, chart_id, "swap", SVG_SETTINGS, "Change chart type");
    }
    push_action_button(
        &mut out,
        chart_id,
        "delete",
        SVG_TRASH,
        "Delete chart",
    );
    out.push_str("</div></div>");
    write!(
        out,
        r#"<div class="report-detail__chart-meta">{}</div>"#,
        html_text(&meta_text)
    )
    .ok();
    render_chart_viz(&mut out, chart_type, options, respondent_count, discussion, text_answers);
    push_chart_caption(&mut out);
    out
}

fn render_chart_viz(
    out: &mut String,
    chart_type: ChartType,
    options: &[ChartOption],
    respondent_count: u32,
    discussion: Option<&DiscussionData>,
    text_answers: &[String],
) {
    match chart_type {
        ChartType::Bar => render_bar(out, options),
        ChartType::Pie => render_pie(out, options, respondent_count),
        ChartType::Table => render_table(out, options, respondent_count),
        ChartType::Lda => render_lda(out, discussion),
        ChartType::TfIdf => render_tfidf(out, discussion),
        ChartType::Network => render_network(out, discussion),
        ChartType::TextList => render_text_list(out, text_answers),
    }
}

fn render_text_list(out: &mut String, answers: &[String]) {
    if answers.is_empty() {
        return push_chart_empty(out);
    }
    out.push_str(
        r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--textlist">"#,
    );
    out.push_str(r#"<table class="report-detail__chart-table">"#);
    out.push_str(r#"<thead><tr><th class="report-detail__textlist-rank-h">#</th><th>응답</th></tr></thead>"#);
    out.push_str("<tbody>");
    for (i, ans) in answers.iter().enumerate() {
        write!(
            out,
            r#"<tr><td class="report-detail__textlist-rank">{}</td><td class="report-detail__textlist-cell">{}</td></tr>"#,
            i + 1,
            html_text(ans)
        )
        .ok();
    }
    out.push_str("</tbody></table></div>");
}

fn render_bar(out: &mut String, options: &[ChartOption]) {
    let max = options.iter().map(|o| o.count).max().unwrap_or(0);
    if options.is_empty() || max == 0 {
        return push_chart_empty(out);
    }
    out.push_str(r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--bar">"#);
    for o in options {
        let height = ((o.count as f32 / max as f32) * 100.0).round() as u32;
        write!(
            out,
            r#"<div class="report-detail__chart-bar" style="height: {h}%" title="{lbl}: {n}" data-label="{lbl}"></div>"#,
            h = height,
            lbl = html_attr(&o.label),
            n = o.count
        )
        .ok();
    }
    out.push_str("</div>");
}

fn render_pie(out: &mut String, options: &[ChartOption], respondent_count: u32) {
    // Pie slices must sum to 100% of the visible disc, so use the sum
    // of counts (= vote share) — NOT the respondent count. For a
    // mutually-exclusive poll the two are equal, but for a
    // multi-response question `respondent_count` is far smaller than
    // the sum and would push slices past the 360° boundary, making the
    // visual disagree with the legend.
    let _ = respondent_count;
    let total: u32 = options.iter().map(|o| o.count).sum();
    if options.is_empty() || total == 0 {
        return push_chart_empty(out);
    }
    let palette = [
        "#06b6d4", "#a855f7", "#f97316", "#60a5fa", "#fcb300", "#6eedd8",
    ];
    let mut stops: Vec<String> = Vec::with_capacity(options.len());
    let mut acc: f32 = 0.0;
    for (i, o) in options.iter().enumerate() {
        let pct = (o.count as f32 / total as f32) * 100.0;
        let next = acc + pct;
        let color = palette[i % palette.len()];
        stops.push(format!("{color} {acc:.2}% {next:.2}%"));
        acc = next;
    }
    let conic = format!("conic-gradient({})", stops.join(", "));
    out.push_str(r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--pie">"#);
    write!(
        out,
        r#"<div class="report-detail__chart-pie" style="background: {};"></div>"#,
        html_attr(&conic)
    )
    .ok();
    out.push_str(r#"<div class="report-detail__chart-pie-legend">"#);
    for (i, o) in options.iter().enumerate() {
        let pct = ((o.count as f32 / total as f32) * 100.0).round() as u32;
        let color = palette[i % palette.len()];
        out.push_str(r#"<div class="report-detail__chart-pie-row">"#);
        write!(
            out,
            r#"<span class="report-detail__chart-pie-swatch" style="background: {};"></span>"#,
            html_attr(color)
        )
        .ok();
        write!(out, r#"<span>{}</span>"#, html_text(&o.label)).ok();
        write!(out, r#"<strong>{pct}%</strong>"#).ok();
        out.push_str("</div>");
    }
    out.push_str("</div></div>");
}

fn render_table(out: &mut String, options: &[ChartOption], respondent_count: u32) {
    if options.is_empty() {
        return push_chart_empty(out);
    }
    out.push_str(
        r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--table">"#,
    );
    out.push_str(r#"<table class="report-detail__chart-table">"#);
    out.push_str(r#"<thead><tr><th>옵션</th><th>응답 수</th><th>비율</th></tr></thead>"#);
    out.push_str("<tbody>");
    for o in options {
        let pct = if respondent_count == 0 {
            "–".to_string()
        } else {
            let p = ((o.count as f32 / respondent_count as f32) * 100.0).round() as u32;
            format!("{p}%")
        };
        write!(
            out,
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            html_text(&o.label),
            o.count,
            html_text(&pct)
        )
        .ok();
    }
    out.push_str("</tbody></table></div>");
}

fn render_lda(out: &mut String, discussion: Option<&DiscussionData>) {
    let topics = discussion.map(|d| d.topics.as_slice()).unwrap_or(&[]);
    if topics.is_empty() {
        return push_chart_empty(out);
    }
    out.push_str(r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--lda">"#);
    for (i, topic) in topics.iter().enumerate() {
        let topic_label = if topic.topic.is_empty() {
            format!("토픽 {}", i + 1)
        } else {
            topic.topic.clone()
        };
        out.push_str(r#"<div class="report-detail__lda-topic">"#);
        // Topic label only — the percentage + ranking bar were noise
        // (rank ≠ topic weight) and made the row look like a chart
        // when it's really just a keyword grouping.
        write!(
            out,
            r#"<div class="report-detail__lda-topic-head"><span class="report-detail__lda-topic-id">{}</span></div>"#,
            html_text(&topic_label)
        )
        .ok();
        out.push_str(r#"<div class="report-detail__lda-keywords">"#);
        for kw in &topic.keywords {
            write!(
                out,
                r#"<span class="report-detail__lda-kw">{}</span>"#,
                html_text(kw)
            )
            .ok();
        }
        out.push_str("</div></div>");
    }
    out.push_str("</div>");
}

fn render_tfidf(out: &mut String, discussion: Option<&DiscussionData>) {
    let terms = discussion.map(|d| d.tfidf_terms.as_slice()).unwrap_or(&[]);
    if terms.is_empty() {
        return push_chart_empty(out);
    }
    out.push_str(
        r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--tfidf">"#,
    );
    out.push_str(r#"<table class="report-detail__chart-table">"#);
    out.push_str(
        r#"<thead><tr><th class="report-detail__tfidf-rank-h">Rank</th><th>Term</th><th class="report-detail__tfidf-score-h">TF-IDF</th></tr></thead>"#,
    );
    out.push_str("<tbody>");
    for (i, t) in terms.iter().enumerate() {
        write!(
            out,
            r#"<tr><td class="report-detail__tfidf-rank">{}</td><td>{}</td><td class="report-detail__tfidf-score">{:.3}</td></tr>"#,
            i + 1,
            html_text(&t.term),
            t.score
        )
        .ok();
    }
    out.push_str("</tbody></table></div>");
}

fn render_network(out: &mut String, discussion: Option<&DiscussionData>) {
    let Some(d) = discussion else {
        return push_chart_empty(out);
    };
    if d.network_nodes.is_empty() {
        return push_chart_empty(out);
    }

    // Polar layout (matches the Rust component renderer for parity).
    let cx = 250.0_f32;
    let cy = 140.0_f32;
    let radius = 110.0_f32;
    let n = d.network_nodes.len() as f32;
    let max_weight = d
        .network_nodes
        .iter()
        .map(|n| n.weight)
        .max()
        .unwrap_or(1)
        .max(1) as f32;
    struct Laid {
        x: f32,
        y: f32,
        r: f32,
        term: String,
    }
    let laid: Vec<Laid> = d
        .network_nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let angle = (i as f32 / n) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();
            let r = 8.0 + (node.weight as f32 / max_weight) * 14.0;
            Laid {
                x,
                y,
                r,
                term: node.term.clone(),
            }
        })
        .collect();
    let by_term: std::collections::HashMap<&str, &Laid> =
        laid.iter().map(|n| (n.term.as_str(), n)).collect();
    let max_edge_weight = d
        .network_edges
        .iter()
        .map(|e| e.weight)
        .max()
        .unwrap_or(1)
        .max(1) as f32;

    out.push_str(
        r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--network">"#,
    );
    out.push_str(r#"<svg class="report-detail__network-svg" viewBox="0 0 500 280">"#);
    // Edges: 100+ edges in a complete-ish graph stack their semi-
    // transparent strokes into one opaque purple wall. Keep individual
    // edges very light so the overall density reads as a softer mesh.
    for e in &d.network_edges {
        if let (Some(a), Some(b)) = (by_term.get(e.source.as_str()), by_term.get(e.target.as_str()))
        {
            // Stroke width 0.5..1.0 (was 1..3) so weighted edges still
            // differ but no single line dominates.
            let stroke = 0.5 + (e.weight as f32 / max_edge_weight) * 0.5;
            write!(
                out,
                r#"<line class="report-detail__network-edge" x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" style="stroke: rgba(168, 85, 247, 0.08); stroke-width: {sw};"/>"#,
                x1 = a.x,
                y1 = a.y,
                x2 = b.x,
                y2 = b.y,
                sw = stroke
            )
            .ok();
        }
    }
    for node in &laid {
        // Node fill 0.18..0.32 (was 0.35..1.0), stroke 0.35 (was 0.7).
        let opacity = (0.18 + node.r / 80.0).min(0.32);
        write!(
            out,
            r#"<circle class="report-detail__network-node" cx="{cx}" cy="{cy}" r="{r}" style="fill: rgba(168, 85, 247, {op}); stroke: rgba(168, 85, 247, 0.35);"/>"#,
            cx = node.x,
            cy = node.y,
            r = node.r,
            op = opacity
        )
        .ok();
    }
    for node in &laid {
        write!(
            out,
            r#"<text class="report-detail__network-label" x="{x}" y="{y}">{label}</text>"#,
            x = node.x,
            y = node.y + 4.0,
            label = html_text(&node.term)
        )
        .ok();
    }
    out.push_str("</svg>");
    write!(
        out,
        r#"<div class="report-detail__network-stats"><span><strong>{n}</strong>노드</span><span><strong>{e}</strong>엣지</span></div>"#,
        n = laid.len(),
        e = d.network_edges.len()
    )
    .ok();
    out.push_str("</div>");
}

fn push_chart_empty(out: &mut String) {
    out.push_str(
        r#"<div class="report-detail__chart-canvas report-detail__chart-canvas--empty">표시할 데이터가 없습니다</div>"#,
    );
}

fn push_action_button(
    out: &mut String,
    chart_id: &str,
    act: &str,
    svg: &str,
    aria: &str,
) {
    let cls = if act == "delete" {
        "report-detail__chart-action report-detail__chart-action--danger"
    } else {
        "report-detail__chart-action"
    };
    write!(
        out,
        r#"<button type="button" class="{cls}" data-act="{act}" data-chart-id='{cid}' aria-label="{aria}">{svg}</button>"#,
        cls = cls,
        act = act,
        cid = html_attr(chart_id),
        aria = aria,
        svg = svg
    )
    .ok();
}

/// HTML-encode a value for use inside a **single-quoted** attribute.
/// `'` is the only character that could break out of the value with
/// the quote style we use; `&` must be encoded to avoid entity confusion;
/// `<` is encoded out of paranoia even though it doesn't end an attr.
fn html_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '\'' => out.push_str("&#39;"),
            '<' => out.push_str("&lt;"),
            c => out.push(c),
        }
    }
    out
}

/// HTML-encode a value for textual content (between tags).
fn html_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

/// Same as `html_attr` but for serialized JSON — the JSON layer already
/// uses `"` so we only need to keep `'`, `&`, `<` safe.
fn attr_safe_json(json: &str) -> String {
    html_attr(json)
}

// ── Inline SVG markup constants ─────────────────────────
const SVG_SETTINGS: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#;

const SVG_TRASH: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg>"#;
