use super::i18n::ReportDetailTranslate;
use crate::features::spaces::pages::report::types::*;
use crate::features::spaces::pages::report::*;
use crate::*;

/// Right rail — default mode shows the auto-derived outline + meta;
/// chart-swap mode shows a source picker for one chart block. The mode
/// is owned by `UseReportDetailContext::outline_mode`.
#[component]
pub fn Outline() -> Element {
    let ctx = use_report_detail_context();
    rsx! {
        aside { class: "report-detail__outline",
            match ctx.outline_mode_value() {
                OutlineMode::Default => rsx! {
                    OutlineDefault {}
                },
                OutlineMode::ChartTypeSwap { chart_id } => rsx! {
                    OutlineSwap { chart_id }
                },
            }
        }
    }
}

#[component]
fn OutlineDefault() -> Element {
    let tr: ReportDetailTranslate = use_translate();
    let ctx = use_report_detail_context();
    rsx! {
        div { class: "report-detail__outline-section",
            div { class: "report-detail__outline-heading", "{tr.outline_heading}" }
            if ctx.outline_is_empty() {
                div { class: "report-detail__outline-empty", "{tr.outline_empty}" }
            } else {
                div { class: "report-detail__outline-list",
                    for entry in ctx.outline() {
                        OutlineRow { key: "{entry.id}", entry: entry.clone() }
                    }
                }
            }
        }
        div { class: "report-detail__outline-section",
            div { class: "report-detail__outline-heading", "{tr.meta_heading}" }
            div { class: "report-detail__outline-meta",
                MetaRow { label: tr.meta_author.to_string(), value: ctx.author() }
                MetaRow {
                    label: tr.meta_created.to_string(),
                    value: ctx.created_relative(),
                }
                MetaRow {
                    label: tr.meta_edited.to_string(),
                    value: ctx.edited_relative(),
                }
            }
        }
    }
}

/// Chart-type swap mode — picks a new visual rendering for the focused
/// chart figure. The option list is filtered by the chart's source
/// (`ChartType::options_for(source)`): poll/quiz/follow get
/// Bar/Pie/Table, discussion gets LDA/TF-IDF/Network. Source stays the
/// same — only the rendering shape changes.
#[component]
fn OutlineSwap(chart_id: String) -> Element {
    let mut ctx = use_report_detail_context();
    let meta = ctx.chart_meta(&chart_id);
    let (current_type, source, title) = match meta.as_ref() {
        Some(m) => (m.chart_type, m.source, m.item_title.clone()),
        None => (ChartType::Bar, ActionSource::Poll, String::new()),
    };
    let options = ChartType::options_for(source);
    rsx! {
        div { class: "report-detail__outline-section report-detail__outline-swap",
            div { class: "report-detail__outline-swap-head",
                div { class: "report-detail__outline-swap-title-col",
                    span { class: "report-detail__outline-swap-eyebrow", "차트 종류 변경" }
                    span { class: "report-detail__outline-swap-title", "{title}" }
                }
                button {
                    class: "report-detail__outline-swap-close",
                    "aria-label": "닫기",
                    onclick: move |_| ctx.close_outline_swap(),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        line {
                            x1: "18",
                            y1: "6",
                            x2: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            y1: "6",
                            x2: "18",
                            y2: "18",
                        }
                    }
                }
            }
            div { class: "report-detail__outline-swap-list",
                for ct in options.iter().copied() {
                    SwapOption {
                        key: "{ct.as_token()}",
                        chart_id: chart_id.clone(),
                        chart_type: ct,
                        active: ct == current_type,
                    }
                }
            }
            div { class: "report-detail__outline-swap-hint",
                "선택 즉시 본문 차트가 교체됩니다"
            }
        }
    }
}

#[component]
fn SwapOption(chart_id: String, chart_type: ChartType, active: bool) -> Element {
    let mut ctx = use_report_detail_context();
    let (label, hint, preview_icon) = match chart_type {
        ChartType::Bar => ("막대 차트", "응답 분포를 막대 그래프로", swap_icon_bar()),
        ChartType::Pie => ("파이 차트", "비율을 원형으로", swap_icon_pie()),
        ChartType::Table => ("표", "원본 집계 표 형식", swap_icon_table()),
        ChartType::Lda => ("LDA Topics", "토픽별 상위 키워드", swap_icon_lda()),
        ChartType::TfIdf => ("TF-IDF", "단어 중요도 순위표", swap_icon_tfidf()),
        ChartType::Network => ("Network", "공출현 단어 네트워크", swap_icon_network()),
        // `TextList` never reaches this branch — the chart-type swap
        // button is hidden on TextList figures, so the user can't open
        // the swap panel against one. Render a generic table icon for
        // exhaustiveness in case the enum gets reused elsewhere.
        ChartType::TextList => ("주관식 응답", "응답 목록", swap_icon_table()),
    };
    let id_for_click = chart_id.clone();
    rsx! {
        button {
            class: "report-detail__swap-opt",
            "data-type": chart_type.as_token(),
            "aria-selected": active,
            r#type: "button",
            onclick: move |_| {
                ctx.swap_chart_type(&id_for_click, chart_type);
            },
            div { class: "report-detail__swap-opt-icon", {preview_icon} }
            div { class: "report-detail__swap-opt-text",
                span { class: "report-detail__swap-opt-label", "{label}" }
                span { class: "report-detail__swap-opt-hint", "{hint}" }
            }
        }
    }
}

fn swap_icon_bar() -> Element {
    rsx! {
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
    }
}

fn swap_icon_pie() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M21.21 15.89A10 10 0 1 1 8 2.83" }
            path { d: "M22 12A10 10 0 0 0 12 2v10z" }
        }
    }
}

fn swap_icon_table() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect {
                x: "3",
                y: "3",
                width: "18",
                height: "18",
                rx: "2",
            }
            line {
                x1: "3",
                y1: "9",
                x2: "21",
                y2: "9",
            }
            line {
                x1: "3",
                y1: "15",
                x2: "21",
                y2: "15",
            }
            line {
                x1: "9",
                y1: "3",
                x2: "9",
                y2: "21",
            }
        }
    }
}

fn swap_icon_lda() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            circle {
                cx: "6",
                cy: "6",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "18",
                cy: "6",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "6",
                cy: "18",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "18",
                cy: "18",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "2",
                fill: "currentColor",
            }
        }
    }
}

fn swap_icon_tfidf() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            text {
                x: "3",
                y: "17",
                font_family: "Orbitron",
                font_size: "10",
                font_weight: "800",
                fill: "currentColor",
                "Aa"
            }
            line {
                x1: "14",
                y1: "6",
                x2: "21",
                y2: "6",
            }
            line {
                x1: "14",
                y1: "12",
                x2: "21",
                y2: "12",
            }
            line {
                x1: "14",
                y1: "18",
                x2: "19",
                y2: "18",
            }
        }
    }
}

fn swap_icon_network() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.6",
            circle {
                cx: "6",
                cy: "6",
                r: "2.4",
                fill: "currentColor",
            }
            circle {
                cx: "18",
                cy: "6",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "6",
                cy: "18",
                r: "2",
                fill: "currentColor",
            }
            circle {
                cx: "18",
                cy: "18",
                r: "2.2",
                fill: "currentColor",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "2.6",
                fill: "currentColor",
            }
            line {
                x1: "8",
                y1: "8",
                x2: "10",
                y2: "10",
            }
            line {
                x1: "14",
                y1: "14",
                x2: "16",
                y2: "16",
            }
            line {
                x1: "8",
                y1: "16",
                x2: "10",
                y2: "14",
            }
            line {
                x1: "14",
                y1: "10",
                x2: "16",
                y2: "8",
            }
        }
    }
}

#[component]
fn OutlineRow(entry: ReportListItemRow) -> Element {
    let kind_class = match entry.kind {
        OutlineKind::H1 => "report-detail__outline-row--h1",
        OutlineKind::H2 => "report-detail__outline-row--h2",
        OutlineKind::H3 => "report-detail__outline-row--h3",
        OutlineKind::Chart => "report-detail__outline-row--chart",
    };
    let id_for_click = entry.id.clone();
    let kind_for_click = entry.kind;
    rsx! {
        button {
            class: "report-detail__outline-row {kind_class}",
            r#type: "button",
            onclick: move |_| scroll_to_outline_entry(&id_for_click, kind_for_click),
            span { "{entry.label}" }
        }
    }
}

/// Scroll to the outline entry's underlying DOM node inside the body
/// editor. For chart figures we have a real DOM id (the chart-id);
/// for headings, the editor doesn't auto-add ids, so we synthesize
/// `h{level}-{n}` in the outline parser and walk to the n-th heading
/// of that level on click.
fn scroll_to_outline_entry(id: &str, kind: OutlineKind) {
    let mut runner = dioxus::document::eval(
        r#"
        const data = await dioxus.recv();
        const editor = document.querySelector('.report-detail .ratel-editor .re-content');
        if (!editor) { dioxus.send(null); return; }
        let target = null;
        if (data.kind === 'chart') {
            target = editor.querySelector('figure[data-chart-id="' + data.id + '"]');
        } else {
            const m = data.id.match(/^h([1-3])-(\d+)$/);
            if (m) {
                const tag = 'h' + m[1];
                const nth = parseInt(m[2], 10) - 1;
                const list = editor.querySelectorAll(tag);
                target = list[nth] || null;
            }
        }
        if (target && target.scrollIntoView) {
            target.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
        dioxus.send(null);
        "#,
    );
    let kind_token = match kind {
        OutlineKind::Chart => "chart",
        OutlineKind::H1 | OutlineKind::H2 | OutlineKind::H3 => "heading",
    };
    let _ = runner.send(serde_json::json!({
        "id": id,
        "kind": kind_token,
    }));
    dioxus::prelude::spawn(async move {
        let _ = runner.recv::<Option<()>>().await;
    });
}

#[component]
fn MetaRow(label: String, value: String) -> Element {
    rsx! {
        div { class: "report-detail__meta-row",
            span { class: "report-detail__meta-key", "{label}" }
            span { class: "report-detail__meta-val", "{value}" }
        }
    }
}

// Re-export the outline entry type as a local prop alias so the
// component signature stays readable.
pub use crate::features::spaces::pages::report::types::OutlineEntry as ReportListItemRow;
