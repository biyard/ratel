use crate::features::spaces::pages::actions::actions::poll::Question;
use crate::features::spaces::pages::actions::actions::poll::controllers::{
    PollResultResponse, PollResultSummary,
};
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::space_common::hooks::use_space;
use crate::features::spaces::space_common::providers::use_space_context;

/// Chart palette. MUST stay in sync with `style.css` `--sap-c1`..`--sap-c6`.
const ANALYZE_CHART_COLORS: [&str; 6] = [
    "#f97316", "#6366f1", "#22c55e", "#3b82f6", "#8b5cf6", "#eab308",
];
const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

#[derive(Debug, Clone, PartialEq)]
struct AnalyzeFilterOption {
    key: String,
    label: String,
}

/// Per-choice stat used by the arena bar chart and pie chart.
///
/// `label` is the positional index ("1", "2", …) shown in the left
/// gutter of each bar row. `option_text` is the human-readable option
/// (e.g. "재생에너지 확대") shown next to the count/percentage.
///
/// Two different normalised widths live on each stat:
///
/// * `percentage` — `count / sum(counts) × 100`. Shown next to every
///   label and used for pie slice angles. Sums to 100 across all
///   stats, which keeps the pie consistent with the legend even for
///   multi-select questions where response rates exceed 100%.
/// * `bar_width` — `count / max(counts) × 100`. Only used for the bar
///   fill width. Anchors the leading option at 100% of the track so
///   short bars and empty (0-count) bars stay visually distinct; when
///   every option ties, every bar hits 100% too.
#[derive(Debug, Clone, PartialEq)]
struct AnalyzeChoiceStat {
    label: String,
    option_text: String,
    count: i64,
    percentage: f64,
    bar_width: f64,
    color: &'static str,
}

#[component]
pub fn SpaceAnalyzeDetailPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    // Read the underlying role — `current_role` flips Creator →
    // Participant once the space is Ongoing, but analyze is always
    // creator-only. See `views/home/mod.rs` for the full story.
    let mut ctx = use_space_context();
    let real_role = ctx.role();

    if real_role != SpaceUserRole::Creator {
        return rsx! {
            document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
            document::Link { rel: "stylesheet", href: asset!("./style.css") }
            div { class: "sap-arena",
                div { class: "sap-viewer-empty", "Creator access only." }
            }
        };
    }

    rsx! {
        PollAnalyzeArena { space_id, poll_id }
    }
}

#[component]
fn PollAnalyzeArena(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();

    let UseSpaceAnalyzePoll {
        poll,
        result,
        mut selected_filter_group,
        mut selected_filter_value,
        mut handle_export_excel,
        ..
    } = use_space_analyze_poll(space_id, poll_id)?;

    let poll_data = poll.read().clone();
    let result_data = result.read().clone();

    let filter_groups = build_filter_group_options(&result_data, &tr);
    let active_group = active_filter_group(&filter_groups, &selected_filter_group());
    let filter_values = build_filter_value_options(&result_data, &active_group, &tr);
    let active_value = active_filter_value(&filter_values, &selected_filter_value());
    let active_filter_key = compose_filter_key(&active_group, &active_value);
    let active_summaries = select_summaries(&result_data, &active_filter_key);

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();
    let poll_title = poll_data.title.clone();

    let export_pending = handle_export_excel.pending();

    let result_for_group_change = result_data.clone();
    let tr_for_group_change = tr.clone();

    let on_group_change = move |value: String| {
        let next_values = build_filter_value_options(
            &result_for_group_change,
            &value,
            &tr_for_group_change,
        );
        let next_value = next_values
            .first()
            .map(|option| option.key.clone())
            .unwrap_or_default();
        selected_filter_group.set(value);
        selected_filter_value.set(next_value);
    };

    let on_value_change = move |value: String| {
        selected_filter_value.set(value);
    };

    rsx! {
        document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "sap-arena",
            // ── Topbar ───────────────────────────────────────────
            header { class: "sap-topbar", role: "banner",
                div { class: "sap-topbar__left",
                    button {
                        r#type: "button",
                        class: "sap-back-btn",
                        "aria-label": "Back",
                        "data-testid": "topbar-back",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M19 12H5" }
                            path { d: "M12 19l-7-7 7-7" }
                        }
                    }
                    img {
                        class: "sap-topbar__logo",
                        alt: "Space logo",
                        src: "{space_logo}",
                    }
                    nav { class: "sap-breadcrumb",
                        span { class: "sap-breadcrumb__item", "{space_title}" }
                        span { class: "sap-breadcrumb__sep", "›" }
                        span { class: "sap-breadcrumb__item", "Apps" }
                        span { class: "sap-breadcrumb__sep", "›" }
                        span { class: "sap-breadcrumb__item", "Analyze" }
                        span { class: "sap-breadcrumb__sep", "›" }
                        span { class: "sap-breadcrumb__item sap-breadcrumb__current",
                            "Poll"
                        }
                    }
                    span { class: "sap-type-badge", "data-testid": "type-badge",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M3 3v18h18" }
                            path { d: "M7 14l4-4 4 4 5-5" }
                        }
                        "Analyze"
                    }
                    span { class: "sap-topbar__title", "{poll_title}" }
                }

                div { class: "sap-topbar__right",
                    button {
                        r#type: "button",
                        class: "sap-btn sap-btn--primary sap-btn--lg",
                        "data-testid": "export-excel-btn",
                        disabled: export_pending,
                        onclick: move |_| handle_export_excel.call(),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line {
                                x1: "12",
                                y1: "15",
                                x2: "12",
                                y2: "3",
                            }
                        }
                        "{tr.download_excel}"
                    }
                }
            }

            // ── Body ─────────────────────────────────────────────
            // `div` instead of `main` — AppLayout already renders an
            // outer <main> (SidebarInset), and nesting <main> inside
            // <main> is invalid HTML and trips some browsers into
            // clipping content.
            div { class: "sap-body",
                // Filter bar — primary (group) + optional secondary (value).
                div { class: "sap-filter-bar", "data-testid": "filter-bar",
                    ArenaFilterDropdown {
                        testid: "filter-select".to_string(),
                        aria_label: tr.filter_group_label.to_string(),
                        selected_key: active_group.clone(),
                        options: filter_groups.clone(),
                        on_change: on_group_change,
                    }

                    if active_group != "overall" && !filter_values.is_empty() {
                        ArenaFilterDropdown {
                            testid: "filter-select-value".to_string(),
                            aria_label: tr.filter_value_label.to_string(),
                            selected_key: active_value.clone(),
                            options: filter_values.clone(),
                            on_change: on_value_change,
                        }
                    }
                }

                // Question cards — one per question, styled based on type.
                for (idx , question) in poll_data.questions.iter().enumerate() {
                    if let Some(summary) = active_summaries.get(idx) {
                        {render_question_card(idx, question, summary, &active_filter_key, &tr)}
                    }
                }
            }
        }
    }
}

/// Render a single question card — dispatches on whether the summary is
/// objective (bar chart + pie chart) or subjective (text response list).
fn render_question_card(
    idx: usize,
    question: &Question,
    summary: &PollResultSummary,
    filter_key: &str,
    tr: &SpaceAnalyzesAppTranslate,
) -> Element {
    let filter_suffix = filter_dom_suffix(filter_key);
    let card_key = format!("analyze-question-{filter_suffix}-{idx}");
    let title = question.title().to_string();
    let total = summary_total_count(summary);
    let response_unit = tr.total_response_count_unit.to_string();

    match summary {
        PollResultSummary::ShortAnswer {
            total_count,
            answers,
        }
        | PollResultSummary::Subjective {
            total_count,
            answers,
        } => {
            let responses = sorted_text_answers(answers);
            let no_responses_text = tr.no_text_responses.to_string();
            let is_empty = responses.is_empty();

            rsx! {
                section {
                    key: "{card_key}",
                    class: "sap-q-card",
                    "data-testid": "question-card-subjective",
                    div { class: "sap-q-card__head",
                        div { class: "sap-q-card__title", "{title}" }
                        span { class: "sap-q-card__count", "{total_count} {response_unit}" }
                    }
                    if is_empty {
                        div { class: "sap-text-empty", "{no_responses_text}" }
                    } else {
                        div { class: "sap-text-list", "data-testid": "text-response-list",
                            for (row_idx , (text , count)) in responses.into_iter().enumerate() {
                                div {
                                    key: "text-response-{row_idx}",
                                    class: "sap-text-item",
                                    "{text} ({count})"
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            let bars = build_choice_stats(question, summary, tr.other_label.to_string());
            let pie_gradient = build_pie_gradient(&bars);
            let total_label = total.to_string();
            let has_pie_data: bool = bars.iter().any(|stat| stat.count > 0);

            rsx! {
                section {
                    key: "{card_key}",
                    class: "sap-q-card",
                    "data-testid": "question-card-objective",
                    div { class: "sap-q-card__head",
                        div { class: "sap-q-card__title", "{title}" }
                        span { class: "sap-q-card__count", "{total} {response_unit}" }
                    }

                    // Bar chart — horizontal bars, one per choice.
                    div { class: "sap-bar-chart", "data-testid": "bar-chart",
                        for (bar_idx , stat) in bars.iter().enumerate() {
                            div { key: "bar-{bar_idx}", class: "sap-bar-row",
                                span { class: "sap-bar-row__label", "{stat.label}" }
                                div { class: "sap-bar-row__track",
                                    div {
                                        class: "sap-bar-row__fill",
                                        style: "width: {stat.bar_width}%; background: {stat.color};",
                                    }
                                }
                                span { class: "sap-bar-row__value",
                                    "{stat.option_text} · {stat.count} ({stat.percentage:.1}%)"
                                }
                            }
                        }
                    }

                    // Pie chart — conic-gradient + center donut hole + legend.
                    if !bars.is_empty() && has_pie_data {
                        div { class: "sap-pie-wrap", "data-testid": "pie-chart",
                            div {
                                class: "sap-pie",
                                role: "img",
                                "aria-label": build_pie_aria_label(&bars),
                                style: "background: {pie_gradient};",
                                div { class: "sap-pie__labels",
                                    div {
                                        span { class: "sap-pie__label-line sap-pie__label-line--top",
                                            "Total"
                                        }
                                        span { class: "sap-pie__label-line sap-pie__label-line--big",
                                            "{total_label}"
                                        }
                                        span { class: "sap-pie__label-line sap-pie__label-line--sub",
                                            "{response_unit}"
                                        }
                                    }
                                }
                            }
                            div { class: "sap-pie-legend", "aria-label": "chart legend",
                                for (legend_idx , stat) in bars.iter().enumerate() {
                                    div { key: "legend-{legend_idx}", class: "sap-pie-legend__row",
                                        span {
                                            class: "sap-pie-legend__swatch",
                                            style: "background: {stat.color};",
                                        }
                                        span { class: "sap-pie-legend__label", "{stat.option_text}" }
                                        span { class: "sap-pie-legend__value", "{stat.percentage:.1}%" }
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

// ── Chart helpers ─────────────────────────────────────────────────

/// Build the pie chart's `conic-gradient` value.
///
/// Safe to use `stat.percentage` directly because `build_choice_stats`
/// normalises percentages to sum to 100 (across both single-select and
/// multi-select questions), so slices always fit inside the 360° disc.
fn build_pie_gradient(bars: &[AnalyzeChoiceStat]) -> String {
    if bars.is_empty() {
        return String::new();
    }

    let mut stops = Vec::with_capacity(bars.len());
    let mut cursor: f64 = 0.0;
    for stat in bars.iter() {
        let next = cursor + (stat.percentage / 100.0) * 360.0;
        stops.push(format!("{} {:.4}deg {:.4}deg", stat.color, cursor, next));
        cursor = next;
    }

    format!("conic-gradient({})", stops.join(", "))
}

fn build_pie_aria_label(bars: &[AnalyzeChoiceStat]) -> String {
    bars.iter()
        .map(|stat| format!("{} {:.1}%", stat.option_text, stat.percentage))
        .collect::<Vec<_>>()
        .join(", ")
}

fn filter_dom_suffix(filter_key: &str) -> String {
    filter_key
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

// ── Filter builders (view-layer derivations) ─────────────────────

fn build_filter_group_options(
    result: &PollResultResponse,
    tr: &SpaceAnalyzesAppTranslate,
) -> Vec<AnalyzeFilterOption> {
    let mut options = vec![AnalyzeFilterOption {
        key: "overall".to_string(),
        label: tr.filter_all.to_string(),
    }];

    if !result.summaries_by_gender.is_empty() {
        options.push(AnalyzeFilterOption {
            key: "gender".to_string(),
            label: tr.filter_gender.to_string(),
        });
    }

    if !result.summaries_by_age.is_empty() {
        options.push(AnalyzeFilterOption {
            key: "age".to_string(),
            label: tr.filter_age.to_string(),
        });
    }

    if !result.summaries_by_school.is_empty() {
        options.push(AnalyzeFilterOption {
            key: "school".to_string(),
            label: tr.filter_school.to_string(),
        });
    }

    options
}

fn build_filter_value_options(
    result: &PollResultResponse,
    group: &str,
    tr: &SpaceAnalyzesAppTranslate,
) -> Vec<AnalyzeFilterOption> {
    let mut values = match group {
        "gender" => result
            .summaries_by_gender
            .keys()
            .cloned()
            .map(|key| AnalyzeFilterOption {
                label: humanize_group_value(&key, tr),
                key,
            })
            .collect::<Vec<_>>(),
        "age" => result
            .summaries_by_age
            .keys()
            .cloned()
            .map(|key| AnalyzeFilterOption {
                label: key.clone(),
                key,
            })
            .collect::<Vec<_>>(),
        "school" => result
            .summaries_by_school
            .keys()
            .cloned()
            .map(|key| AnalyzeFilterOption {
                label: humanize_group_value(&key, tr),
                key,
            })
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    values.sort_by(|left, right| left.label.cmp(&right.label));
    values
}

fn active_filter_group(options: &[AnalyzeFilterOption], key: &str) -> String {
    options
        .iter()
        .find(|option| option.key == key)
        .map(|option| option.key.clone())
        .unwrap_or_else(|| "overall".to_string())
}

fn active_filter_value(options: &[AnalyzeFilterOption], key: &str) -> String {
    options
        .iter()
        .find(|option| option.key == key)
        .map(|option| option.key.clone())
        .or_else(|| options.first().map(|option| option.key.clone()))
        .unwrap_or_default()
}

fn compose_filter_key(group: &str, value: &str) -> String {
    if group == "overall" || value.is_empty() {
        "overall".to_string()
    } else {
        format!("{group}:{value}")
    }
}

fn select_summaries(result: &PollResultResponse, key: &str) -> Vec<PollResultSummary> {
    if let Some(gender) = key.strip_prefix("gender:") {
        return result
            .summaries_by_gender
            .get(gender)
            .cloned()
            .unwrap_or_else(|| result.summaries.clone());
    }

    if let Some(age) = key.strip_prefix("age:") {
        return result
            .summaries_by_age
            .get(age)
            .cloned()
            .unwrap_or_else(|| result.summaries.clone());
    }

    if let Some(school) = key.strip_prefix("school:") {
        return result
            .summaries_by_school
            .get(school)
            .cloned()
            .unwrap_or_else(|| result.summaries.clone());
    }

    result.summaries.clone()
}

fn summary_total_count(summary: &PollResultSummary) -> i64 {
    match summary {
        PollResultSummary::SingleChoice { total_count, .. }
        | PollResultSummary::MultipleChoice { total_count, .. }
        | PollResultSummary::ShortAnswer { total_count, .. }
        | PollResultSummary::Subjective { total_count, .. }
        | PollResultSummary::Checkbox { total_count, .. }
        | PollResultSummary::Dropdown { total_count, .. }
        | PollResultSummary::LinearScale { total_count, .. } => *total_count,
    }
}

fn build_choice_stats(
    question: &Question,
    summary: &PollResultSummary,
    other_label: String,
) -> Vec<AnalyzeChoiceStat> {
    let mut stats = build_choice_stats_raw(question, summary, other_label);

    // Re-normalise `percentage` so every bar's width and the pie slice
    // both express the same thing: this option's share of all votes.
    //
    // For single-select questions (SingleChoice / Dropdown / LinearScale)
    // the sum of counts equals `total_count`, so percentage stays the
    // same as the response-rate the build helpers originally computed.
    //
    // For multi-select questions (MultipleChoice / Checkbox) a single
    // respondent can pick multiple options, so response-rate percentages
    // can sum to more than 100% and a 50/50 split would otherwise show
    // as 100% / 100%. Dividing by the total vote count produces the
    // slice shares you actually see in the pie chart.
    let total: i64 = stats.iter().map(|stat| stat.count).sum();
    if total > 0 {
        let total_f = total as f64;
        for stat in &mut stats {
            stat.percentage = (stat.count as f64 / total_f) * 100.0;
        }
    } else {
        for stat in &mut stats {
            stat.percentage = 0.0;
        }
    }

    // Max-normalise bar widths. The leading option fills the track
    // (100%), every other bar is proportional to it, and 0-count bars
    // render as an empty track — no coloured fill for dead options.
    let max: i64 = stats.iter().map(|stat| stat.count).max().unwrap_or(0);
    if max > 0 {
        let max_f = max as f64;
        for stat in &mut stats {
            stat.bar_width = (stat.count as f64 / max_f) * 100.0;
        }
    }

    stats
}

fn build_choice_stats_raw(
    question: &Question,
    summary: &PollResultSummary,
    other_label: String,
) -> Vec<AnalyzeChoiceStat> {
    match (question, summary) {
        (
            Question::SingleChoice(question) | Question::MultipleChoice(question),
            PollResultSummary::SingleChoice {
                total_count,
                answers,
                other_answers,
            }
            | PollResultSummary::MultipleChoice {
                total_count,
                answers,
                other_answers,
            },
        ) => question
            .options
            .iter()
            .enumerate()
            .map(|(idx, option_text)| {
                build_choice_stat(
                    &(idx + 1).to_string(),
                    option_text.clone(),
                    *answers.get(&idx.to_string()).unwrap_or(&0),
                    *total_count,
                    idx,
                )
            })
            .chain(build_other_choice_stat(
                question.allow_other.unwrap_or(false),
                other_answers,
                *total_count,
                question.options.len(),
                &other_label,
            ))
            .collect(),
        (
            Question::Checkbox(question),
            PollResultSummary::Checkbox {
                total_count,
                answers,
            },
        ) => question
            .options
            .iter()
            .enumerate()
            .map(|(idx, option_text)| {
                build_choice_stat(
                    &(idx + 1).to_string(),
                    option_text.clone(),
                    *answers.get(&idx.to_string()).unwrap_or(&0),
                    *total_count,
                    idx,
                )
            })
            .collect(),
        (
            Question::Dropdown(question),
            PollResultSummary::Dropdown {
                total_count,
                answers,
            },
        ) => question
            .options
            .iter()
            .enumerate()
            .map(|(idx, option_text)| {
                build_choice_stat(
                    &(idx + 1).to_string(),
                    option_text.clone(),
                    *answers.get(&idx.to_string()).unwrap_or(&0),
                    *total_count,
                    idx,
                )
            })
            .collect(),
        (
            Question::LinearScale(question),
            PollResultSummary::LinearScale {
                total_count,
                answers,
            },
        ) => (question.min_value..=question.max_value)
            .enumerate()
            .map(|(idx, value)| {
                let label = value.to_string();
                build_choice_stat(
                    &label,
                    label.clone(),
                    *answers.get(&value.to_string()).unwrap_or(&0),
                    *total_count,
                    idx,
                )
            })
            .collect(),
        _ => vec![],
    }
}

fn build_choice_stat(
    label: &str,
    option_text: String,
    count: i64,
    total_count: i64,
    index: usize,
) -> AnalyzeChoiceStat {
    let percentage = if total_count > 0 {
        (count as f64 / total_count as f64) * 100.0
    } else {
        0.0
    };

    AnalyzeChoiceStat {
        label: label.to_string(),
        option_text,
        count,
        percentage,
        // Filled in by `build_choice_stats` after the full list is known.
        bar_width: 0.0,
        color: ANALYZE_CHART_COLORS[index % ANALYZE_CHART_COLORS.len()],
    }
}

fn build_other_choice_stat(
    allow_other: bool,
    other_answers: &std::collections::HashMap<String, i64>,
    total_count: i64,
    index: usize,
    other_label: &str,
) -> std::option::IntoIter<AnalyzeChoiceStat> {
    if !allow_other {
        return None.into_iter();
    }

    let count: i64 = other_answers.values().copied().sum();
    Some(build_choice_stat(
        &(index + 1).to_string(),
        other_label.to_string(),
        count,
        total_count,
        index,
    ))
    .into_iter()
}

fn sorted_text_answers(answers: &std::collections::HashMap<String, i64>) -> Vec<(String, i64)> {
    let mut items: Vec<_> = answers
        .iter()
        .filter_map(|(text, count)| {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some((trimmed.to_string(), *count))
            }
        })
        .collect();

    items.sort_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| left.0.to_lowercase().cmp(&right.0.to_lowercase()))
    });
    items
}

// ── Arena filter dropdown ─────────────────────────────────────────
//
// Custom dropdown mirroring `common::TeamSelector` — trigger button +
// `position: fixed` backdrop for outside-click close + absolute-
// positioned panel underneath the trigger. Native `<select>` dropdowns
// are rendered by the OS and can't be styled to match the arena glass
// aesthetic, so we roll our own.

#[component]
fn ArenaFilterDropdown(
    testid: String,
    aria_label: String,
    selected_key: String,
    options: Vec<AnalyzeFilterOption>,
    on_change: EventHandler<String>,
) -> Element {
    let mut open = use_signal(|| false);

    let selected_label = options
        .iter()
        .find(|option| option.key == selected_key)
        .map(|option| option.label.clone())
        .unwrap_or_default();

    rsx! {
        div { class: "sap-filter-dropdown",
            button {
                r#type: "button",
                class: "sap-filter-trigger",
                "data-testid": testid,
                "aria-label": aria_label,
                "aria-haspopup": "listbox",
                "aria-expanded": open(),
                onclick: move |_| open.set(!open()),
                span { class: "sap-filter-trigger__label", "{selected_label}" }
                svg {
                    class: "sap-filter-trigger__chevron",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }

            if open() {
                // Click-outside backdrop — `fixed inset-0` so it blankets
                // the viewport regardless of page scroll.
                div {
                    class: "sap-filter-backdrop",
                    onclick: move |_| open.set(false),
                }
                div { class: "sap-filter-panel", role: "listbox",
                    for option in options.iter() {
                        {render_dropdown_option(option, &selected_key, on_change, open)}
                    }
                }
            }
        }
    }
}

fn render_dropdown_option(
    option: &AnalyzeFilterOption,
    selected_key: &str,
    on_change: EventHandler<String>,
    mut open: Signal<bool>,
) -> Element {
    let key = option.key.clone();
    let label = option.label.clone();
    let is_selected = option.key == selected_key;

    rsx! {
        button {
            key: "{option.key}",
            r#type: "button",
            class: "sap-filter-option",
            role: "option",
            "aria-selected": is_selected,
            "data-selected": is_selected,
            onclick: move |_| {
                open.set(false);
                on_change.call(key.clone());
            },
            "{label}"
        }
    }
}
