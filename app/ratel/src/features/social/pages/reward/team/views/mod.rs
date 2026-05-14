mod i18n;

use super::controllers::{
    get_team_monthly_summaries_handler, get_team_reward_permission_handler,
    get_team_rewards_handler, list_team_point_transactions_handler,
    request_team_claim_signature_handler, TeamClaimSignatureRequest,
};
use super::dto::TeamRewardsResponse;
use super::*;
use crate::common::services::{MonthlySummaryItem, PointTransactionResponse};
use crate::features::social::pages::team_arena::{use_team_arena, TeamArenaTab};
use crate::common::*;

pub use i18n::TeamRewardsTranslate;

pub fn format_points(points: i64) -> String {
    format_with_commas(points, None)
}

pub fn format_tokens(tokens: f64) -> String {
    let formatted = format!("{:.2}", tokens);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format_with_commas_str(trimmed)
}

pub fn format_with_commas(value: i64, suffix: Option<&str>) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let formatted: String = out.chars().rev().collect();
    match suffix {
        Some(suffix) => format!("{}{}{}", sign, formatted, suffix),
        None => format!("{}{}", sign, formatted),
    }
}

pub fn format_with_commas_str(value: &str) -> String {
    let (sign, raw) = if let Some(stripped) = value.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", value)
    };
    let mut parts = raw.split('.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next();
    let mut out = String::new();
    for (idx, ch) in int_part.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let int_formatted: String = out.chars().rev().collect();
    match frac_part {
        Some(frac) if !frac.is_empty() => format!("{}{}.{}", sign, int_formatted, frac),
        _ => format!("{}{}", sign, int_formatted),
    }
}

fn month_parts(month: &str) -> (String, String) {
    let mut it = month.split('-');
    let year = it.next().unwrap_or("").to_string();
    let m = it.next().unwrap_or("").to_string();
    let name = match m.as_str() {
        "01" => "January",
        "02" => "February",
        "03" => "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" => "July",
        "08" => "August",
        "09" => "September",
        "10" => "October",
        "11" => "November",
        "12" => "December",
        _ => "",
    };
    (name.to_string(), year)
}

fn pretty_month(month: &str) -> String {
    let (name, year) = month_parts(month);
    if name.is_empty() {
        month.to_string()
    } else {
        format!("{} {}", name, year)
    }
}

/// Mirrors the user-reward variant: launch quarter is 2026-04 … 2026-06.
/// While the current cycle is inside this window the team hero card
/// aggregates points / total / supply across all three months instead
/// of showing a single sparse month.
fn is_launch_quarter(month: &str) -> bool {
    matches!(month, "2026-04" | "2026-05" | "2026-06")
}

fn time_ago(ts: i64) -> String {
    crate::common::utils::time::time_ago(ts)
}

#[component]
pub fn Home(username: ReadSignal<String>) -> Element {
    let tr: TeamRewardsTranslate = use_translate();
    let current_month = utils::time::current_month();
    let nav = use_navigator();

    // Sync arena topbar tab.
    let mut arena = use_team_arena();
    use_effect(move || arena.active_tab.set(TeamArenaTab::Rewards));

    // Resolve the team's partition first — the team reward endpoints are
    // keyed by `TeamPartition`, not username, so we fetch the permission
    // context which carries the real team_pk, then use it for every other
    // loader.
    let perm_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(get_team_reward_permission_handler(username()).await.ok())
    })?;
    let Some(perm) = perm_resource() else {
        return rsx! {
            div { class: "rewards-arena",
                div { class: "page",
                    div { class: "empty",
                        div { class: "empty-desc", "{tr.activity_empty}" }
                    }
                }
            }
        };
    };
    let team_pk = perm.team_pk;
    let team_pk_signal: Signal<TeamPartition> = use_signal(|| team_pk.clone());

    let rewards_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(get_team_rewards_handler(team_pk_signal(), None).await.ok())
    })?;
    let transactions_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(
            list_team_point_transactions_handler(
                team_pk_signal(),
                Some(utils::time::current_month()),
                None,
            )
            .await
            .ok(),
        )
    })?;
    let summaries_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(
            get_team_monthly_summaries_handler(team_pk_signal()).await.ok(),
        )
    })?;
    let rewards: TeamRewardsResponse = rewards_resource().unwrap_or_default();
    let initial_transactions = transactions_resource().unwrap_or_default();
    let past_months = summaries_resource()
        .map(|s| s.months)
        .unwrap_or_default();

    let mut transactions = use_signal(Vec::<PointTransactionResponse>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut transactions_loaded = use_signal(|| false);
    let mut is_fetching_next = use_signal(|| false);

    use_effect(move || {
        if *transactions_loaded.read() {
            return;
        }
        let current = current_month.clone();
        let filtered: Vec<_> = initial_transactions
            .items
            .iter()
            .filter(|tx| tx.month == current)
            .cloned()
            .collect();
        transactions.set(filtered);
        next_bookmark.set(initial_transactions.bookmark.clone());
        transactions_loaded.set(true);
    });

    // Mirror the user-reward launch-quarter aggregation: while we're
    // inside 2026-04 … 2026-06, sum the team's `total_earned`,
    // `project_total_points`, and `monthly_token_supply` across every
    // launch-quarter row in `past_months` PLUS the current cycle's
    // `rewards.*`. From 2026-07 onward the hero passes through
    // `rewards.*` straight off the current-cycle endpoint.
    let in_launch_quarter = is_launch_quarter(&rewards.month);
    let (hero_points, hero_total_raw, hero_supply) = if in_launch_quarter {
        let mut pts: i64 = rewards.team_points;
        let mut total: i64 = rewards.total_points;
        let mut supply: i64 = rewards.monthly_token_supply;
        for m in past_months.iter() {
            if is_launch_quarter(&m.month) {
                pts += m.total_earned;
                total += m.project_total_points;
                supply += m.monthly_token_supply;
            }
        }
        (pts, total, supply)
    } else {
        (
            rewards.team_points,
            rewards.total_points,
            rewards.monthly_token_supply,
        )
    };

    let hero_total = hero_total_raw.max(1);
    let share_percent = (hero_points as f64 / hero_total as f64) * 100.0;
    let share_fill_pct = share_percent.clamp(0.0, 100.0);

    let estimated_tokens = if hero_total_raw > 0 {
        ((hero_points as f64 / hero_total_raw as f64) * hero_supply as f64).round()
    } else {
        0.0
    };

    // Chart current-bar tokens: per-month, independent of the hero
    // aggregation. Stays true to the chart's "Last 6 cycles · Monthly"
    // axis even while the hero is showing an APR-JUN bundle.
    let chart_current_tokens = if rewards.total_points > 0 {
        ((rewards.team_points as f64 / rewards.total_points as f64)
            * rewards.monthly_token_supply as f64)
            .round()
    } else {
        0.0
    };

    let month_pretty = if in_launch_quarter {
        "APR-JUN 2026".to_string()
    } else {
        pretty_month(&rewards.month)
    };
    let share_percent_str = format!("{:.2}", share_percent);

    let token_symbol = if rewards.token_symbol.is_empty() {
        "RATEL".to_string()
    } else {
        rewards.token_symbol.clone()
    };

    let has_next = next_bookmark.read().is_some();
    let is_fetching_next_value = *is_fetching_next.read();
    let month_for_load = rewards.month.clone();

    let on_load_more = move |_| {
        let month_for_load = month_for_load.clone();
        async move {
            if *is_fetching_next.read() {
                return;
            }
            let Some(bookmark) = next_bookmark.read().clone() else {
                return;
            };
            let pk = team_pk_signal();
            is_fetching_next.set(true);
            if let Ok(data) =
                list_team_point_transactions_handler(pk, Some(month_for_load), Some(bookmark)).await
            {
                let mut updated = transactions.read().clone();
                updated.extend(data.items);
                transactions.set(updated);
                next_bookmark.set(data.bookmark);
            }
            is_fetching_next.set(false);
        }
    };

    let tx_list = transactions.read().clone();
    let tx_count = tx_list.len();

    // Build combo chart: every past cycle (up to the last 5) + the
    // current cycle, each rendered as its own monthly bar. Launch
    // quarter is intentionally NOT bundled — only the hero card
    // aggregates 4-6월; the chart history stays per-month.
    let mut combo_points: Vec<(String, i64, f64)> = past_months
        .iter()
        .rev()
        .take(5)
        .rev()
        .map(|m| {
            let (name, _) = month_parts(&m.month);
            let toks = if m.project_total_points > 0 {
                (m.total_earned as f64 / m.project_total_points as f64)
                    * m.monthly_token_supply as f64
            } else {
                0.0
            };
            (name.chars().take(3).collect(), m.total_earned, toks)
        })
        .collect();
    let (cur_name, _) = month_parts(&rewards.month);
    combo_points.push((
        cur_name.chars().take(3).collect(),
        rewards.team_points,
        chart_current_tokens,
    ));
    let combo_svg = render_combo(&combo_points);

    // Donut from this cycle's transactions grouped by type
    let mut by_type: std::collections::BTreeMap<String, i64> = std::collections::BTreeMap::new();
    for tx in &tx_list {
        let key = tx.transaction_type.clone();
        *by_type.entry(key).or_insert(0) += tx.amount.max(0);
    }
    let palette = ["#818cf8", "#fcb300", "#a855f7", "#22d3ee", "#fb923c"];
    let mut donut_items: Vec<(String, i64, &str)> = Vec::new();
    for (i, (name, val)) in by_type.iter().enumerate() {
        if i >= palette.len() {
            break;
        }
        donut_items.push((name.clone(), *val, palette[i]));
    }
    let donut_total: i64 = donut_items.iter().map(|(_, v, _)| *v).sum();
    let donut_svg = render_donut(&donut_items, donut_total);

    let claimable_count = past_months.iter().filter(|m| !m.exchanged).count();

    rsx! {
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "rewards-arena",
            div { class: "section-label",
                span { class: "section-label__dash" }
                span { class: "section-label__title",
                    "{tr.section_label_prefix} "
                    strong { "{tr.section_label_strong}" }
                }
                span { class: "section-label__dash" }
            }
            div { class: "page",
                HeroCard {
                    tr: tr.clone(),
                    points: hero_points,
                    total_points: hero_total_raw,
                    share_percent_str: share_percent_str.clone(),
                    share_fill_pct,
                    estimated_tokens,
                    monthly_supply: hero_supply,
                    token_symbol: token_symbol.clone(),
                    month_pretty: month_pretty.clone(),
                }

                div { class: "charts",
                    div { class: "chart-card",
                        div { class: "chart-card__head",
                            div {
                                div { class: "chart-card__title", "{tr.chart_points_tokens}" }
                                div { class: "chart-card__subtitle", "{tr.chart_subtitle}" }
                            }
                            div { class: "chart-legends",
                                span { class: "chart-legend",
                                    span { class: "legend-swatch legend-swatch--bar" }
                                    "{tr.chart_legend_points}"
                                }
                                span { class: "chart-legend",
                                    span { class: "legend-swatch legend-swatch--line" }
                                    "{token_symbol}"
                                }
                            }
                        }
                        div {
                            class: "combo-chart",
                            dangerous_inner_html: combo_svg,
                        }
                    }
                    div { class: "chart-card",
                        div { class: "chart-card__head",
                            div {
                                div { class: "chart-card__title", "{tr.source_breakdown}" }
                                div { class: "chart-card__subtitle", "{tr.source_subtitle}" }
                            }
                        }
                        div { class: "donut-wrap",
                            div { dangerous_inner_html: donut_svg }
                            div { class: "donut-legend",
                                if donut_items.is_empty() {
                                    div { class: "empty-desc", "{tr.activity_empty}" }
                                } else {
                                    for (name, value, color) in donut_items.iter() {
                                        div {
                                            class: "legend-item",
                                            key: "{name}",
                                            span {
                                                class: "legend-dot",
                                                style: "background:{color}",
                                            }
                                            span { class: "legend-name", "{name}" }
                                            span { class: "legend-value", {format_points(*value)} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Activity
                div {
                    div { class: "section-head",
                        span { class: "section-head__title", "{tr.activity_title}" }
                        span { class: "section-head__count",
                            strong { "{tx_count}" }
                            " {tr.entries}"
                        }
                    }
                    div { class: "activity",
                        if tx_list.is_empty() {
                            div { class: "empty",
                                div { class: "empty__icon",
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "1.6",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        circle { cx: "12", cy: "12", r: "10" }
                                        path { d: "M12 6v12" }
                                        path { d: "M16 10H8" }
                                    }
                                }
                                div { class: "empty__desc", "{tr.activity_empty}" }
                            }
                        } else {
                            for tx in tx_list.iter() {
                                ActivityRow {
                                    key: "{tx.created_at}-{tx.transaction_type}",
                                    tx: tx.clone(),
                                    pts_unit: tr.pts_unit.to_string(),
                                }
                            }
                            if has_next {
                                button {
                                    class: "activity__loadmore",
                                    disabled: is_fetching_next_value,
                                    onclick: on_load_more,
                                    if is_fetching_next_value {
                                        "{tr.loading}"
                                    } else {
                                        "{tr.load_more}"
                                    }
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        polyline { points: "6 9 12 15 18 9" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Past cycles. The Swap All action is gated off (the
                // claim signature + on-chain swap pipeline isn't ready)
                // — the inline `section-note` below explains that, and
                // each `CycleCard` renders the Swap All button greyed
                // out with a "Coming soon" hover hint.
                div {
                    div { class: "section-head",
                        span { class: "section-head__title", "{tr.past_cycles}" }
                        span { class: "section-head__count",
                            strong { "{claimable_count}" }
                            " {tr.claimable}"
                        }
                    }
                    div {
                        class: "section-note",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M12 16v-4" }
                            path { d: "M12 8h.01" }
                        }
                        span { "{tr.swap_coming_soon_note}" }
                    }
                    div { class: "cycles",
                        if past_months.is_empty() {
                            div { class: "empty",
                                div { class: "empty__icon",
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "1.6",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        circle { cx: "12", cy: "12", r: "10" }
                                        path { d: "M12 6v12" }
                                        path { d: "M16 10H8" }
                                    }
                                }
                                div {
                                    class: "empty__title",
                                    "{tr.past_empty_title}"
                                }
                                div {
                                    class: "empty__desc",
                                    "{tr.past_empty_desc}"
                                }
                            }
                        } else {
                            for item in past_months.iter() {
                                CycleCard {
                                    key: "{item.month}",
                                    team_pk: team_pk_signal,
                                    item: item.clone(),
                                    token_symbol: token_symbol.clone(),
                                    tr: tr.clone(),
                                    swap_enabled: false,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HeroCard(
    tr: TeamRewardsTranslate,
    points: i64,
    total_points: i64,
    share_percent_str: String,
    share_fill_pct: f64,
    estimated_tokens: f64,
    monthly_supply: i64,
    token_symbol: String,
    month_pretty: String,
) -> Element {
    let points_fmt = format_points(points);
    let total_fmt = format_points(total_points);
    let tokens_fmt = format_tokens(estimated_tokens);
    let supply_fmt = format_points(monthly_supply);
    rsx! {
        div { class: "hero",
            div { class: "hero__main",
                div { class: "hero__eyebrow",
                    span { class: "pulse" }
                    span { "{tr.earning_this_cycle}" }
                    span { style: "color:var(--text-dim)", "·" }
                    strong { "{month_pretty}" }
                }
                div { class: "hero__points",
                    span { class: "hero__points-value", "{points_fmt}" }
                    span { class: "hero__points-unit", "{tr.points}" }
                }
                div { class: "hero__share",
                    div { class: "hero__share-row",
                        span { class: "hero__share-label", "{tr.share_of_pool}" }
                        span { class: "hero__share-value",
                            strong { "{share_percent_str}%" }
                            small { "{tr.of_total} {total_fmt} {tr.total_points_unit}" }
                        }
                    }
                    div { class: "hero__share-bar",
                        div {
                            class: "hero__share-bar-fill",
                            style: "width:{share_fill_pct}%",
                        }
                    }
                    div { class: "hero__share-meta",
                        span { "{tr.your_position}" }
                        span { "{tr.rank_of} —" }
                    }
                }
            }
            div { class: "hero__side",
                div {
                    div { class: "hero__token-label",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M12 6v12" }
                            path { d: "M16 10H8" }
                        }
                        "{tr.estimated_tokens}"
                    }
                    div { class: "hero__token-value",
                        strong { "{tokens_fmt}" }
                        small { "{token_symbol}" }
                    }
                }
                div { class: "hero__token-formula",
                    "({points_fmt} ÷ {total_fmt}) × {supply_fmt} = "
                    code { "{tokens_fmt} {token_symbol}" }
                }
                div { class: "hero__countdown",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "10" }
                        polyline { points: "12 6 12 12 16 14" }
                    }
                    div { class: "hero__countdown-text",
                        "{tr.cycle_locks_in} "
                        strong { "—" }
                        " {tr.claim_opens}"
                    }
                }
            }
        }
    }
}

#[component]
fn ActivityRow(tx: PointTransactionResponse, pts_unit: String) -> Element {
    let amount_class = if tx.amount >= 0 {
        "activity__amount--in"
    } else {
        "activity__amount--out"
    };
    let icon_class = if tx.amount >= 0 {
        "activity__icon--in"
    } else {
        "activity__icon--out"
    };
    let sign = if tx.amount >= 0 { "+" } else { "" };
    let title = tx
        .description
        .clone()
        .unwrap_or_else(|| tx.transaction_type.clone());
    let amount_fmt = format_points(tx.amount);
    let ago = time_ago(tx.created_at);
    rsx! {
        div { class: "activity__row",
            div { class: "activity__icon {icon_class}",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M18 20V10" }
                    path { d: "M12 20V4" }
                    path { d: "M6 20v-6" }
                }
            }
            div { class: "activity__body",
                span { class: "activity__title", "{title}" }
                span { class: "activity__source", "{tx.transaction_type}" }
            }
            div { class: "activity__amount {amount_class}",
                "{sign}{amount_fmt} "
                small { "{pts_unit}" }
            }
            div { class: "activity__time", "{ago}" }
        }
    }
}

fn render_combo(points: &[(String, i64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let width: f64 = 520.0;
    let height: f64 = 240.0;
    let margin_l = 50.0;
    let margin_r = 50.0;
    let margin_t = 18.0;
    let margin_b = 32.0;
    let inner_w = width - margin_l - margin_r;
    let inner_h = height - margin_t - margin_b;

    let max_p = points.iter().map(|(_, p, _)| *p).max().unwrap_or(1).max(1) as f64;
    let max_t = points
        .iter()
        .map(|(_, _, t)| *t)
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let n = points.len() as f64;
    let slot = inner_w / n;
    let bar_w = slot * 0.45;

    let mut bars = String::new();
    let mut line_d = String::new();
    let mut dots = String::new();
    let mut labels = String::new();
    for (i, (name, p, t)) in points.iter().enumerate() {
        let cx = margin_l + slot * (i as f64 + 0.5);
        let bx = cx - bar_w / 2.0;
        let bh = (*p as f64 / max_p) * inner_h;
        let by = margin_t + inner_h - bh;
        bars.push_str(&format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"5\" fill=\"url(#g-bar)\"/>",
            bx, by, bar_w, bh
        ));
        let ly = margin_t + inner_h - (t / max_t) * inner_h;
        if i == 0 {
            line_d.push_str(&format!("M {:.2} {:.2}", cx, ly));
        } else {
            line_d.push_str(&format!(" L {:.2} {:.2}", cx, ly));
        }
        dots.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"4\" fill=\"#06060e\" stroke=\"#6eedd8\" stroke-width=\"2\"/>",
            cx, ly
        ));
        labels.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" text-anchor=\"middle\" font-family=\"Orbitron,sans-serif\" font-size=\"10\" font-weight=\"600\" letter-spacing=\"1\" fill=\"#55556a\">{}</text>",
            cx,
            margin_t + inner_h + 20.0,
            name.to_uppercase()
        ));
    }

    format!(
        "<svg viewBox=\"0 0 {w} {h}\" preserveAspectRatio=\"none\" width=\"100%\" height=\"240\"><defs><linearGradient id=\"g-bar\" x1=\"0\" x2=\"0\" y1=\"0\" y2=\"1\"><stop offset=\"0%\" stop-color=\"#ffd24a\"/><stop offset=\"100%\" stop-color=\"#fcb300\" stop-opacity=\"0.85\"/></linearGradient></defs>{bars}<path d=\"{line}\" fill=\"none\" stroke=\"#6eedd8\" stroke-width=\"2.5\" stroke-linecap=\"round\"/>{dots}{labels}</svg>",
        w = width,
        h = height,
        bars = bars,
        line = line_d,
        dots = dots,
        labels = labels
    )
}

fn render_donut(items: &[(String, i64, &str)], total: i64) -> String {
    if total <= 0 || items.is_empty() {
        return "<svg class=\"donut\" width=\"160\" height=\"160\" viewBox=\"0 0 200 200\"><circle cx=\"100\" cy=\"100\" r=\"80\" fill=\"none\" stroke=\"rgba(255,255,255,0.05)\" stroke-width=\"21\"/></svg>".to_string();
    }
    let cx = 100.0_f64;
    let cy = 100.0_f64;
    let r_outer = 91.0_f64;
    let r_inner = 70.0_f64;

    let mut offset = -std::f64::consts::FRAC_PI_2;
    let mut paths = String::new();
    for (_, v, color) in items.iter() {
        let frac = *v as f64 / total as f64;
        let start = offset;
        let end = offset + frac * std::f64::consts::TAU;
        offset = end;
        let large = if (end - start) > std::f64::consts::PI {
            1
        } else {
            0
        };
        let x0 = cx + r_outer * start.cos();
        let y0 = cy + r_outer * start.sin();
        let x1 = cx + r_outer * end.cos();
        let y1 = cy + r_outer * end.sin();
        let x2 = cx + r_inner * end.cos();
        let y2 = cy + r_inner * end.sin();
        let x3 = cx + r_inner * start.cos();
        let y3 = cy + r_inner * start.sin();
        paths.push_str(&format!(
            "<path d=\"M {x0:.2} {y0:.2} A {r_outer:.2} {r_outer:.2} 0 {large} 1 {x1:.2} {y1:.2} L {x2:.2} {y2:.2} A {r_inner:.2} {r_inner:.2} 0 {large} 0 {x3:.2} {y3:.2} Z\" fill=\"{color}\"/>"
        ));
    }

    format!(
        "<svg class=\"donut\" width=\"160\" height=\"160\" viewBox=\"0 0 200 200\"><circle cx=\"100\" cy=\"100\" r=\"80.5\" fill=\"none\" stroke=\"rgba(255,255,255,0.05)\" stroke-width=\"21\"/>{paths}<text x=\"100\" y=\"98\" text-anchor=\"middle\" font-family=\"Orbitron,sans-serif\" font-size=\"24\" font-weight=\"700\" fill=\"#f0f0f5\">{total_fmt}</text><text x=\"100\" y=\"116\" text-anchor=\"middle\" font-family=\"Orbitron,sans-serif\" font-size=\"8.5\" font-weight=\"600\" letter-spacing=\"2\" fill=\"#55556a\">POINTS</text></svg>",
        paths = paths,
        total_fmt = format_points(total)
    )
}

/// Cycle history row for the team rewards page. Mirrors the user-side
/// `CycleCard` — same layout, same Swap All gating — but routes the
/// claim signature request through the team-scoped endpoint
/// (`request_team_claim_signature_handler`) so the on-chain claim
/// goes against the team's pk space when the swap pipeline opens.
#[component]
fn CycleCard(
    team_pk: ReadSignal<TeamPartition>,
    item: MonthlySummaryItem,
    token_symbol: String,
    tr: TeamRewardsTranslate,
    /// Gate the Swap All CTA off while the claim/swap pipeline is
    /// still being built. Defaults to true so future activations
    /// don't need a prop change at every call site.
    #[props(default = true)]
    swap_enabled: bool,
) -> Element {
    let mut is_claiming = use_signal(|| false);
    let mut claimed = use_signal(move || item.exchanged);

    let (month_name, year) = month_parts(&item.month);
    let share_pct = if item.project_total_points > 0 {
        (item.total_earned as f64 / item.project_total_points as f64) * 100.0
    } else {
        0.0
    };
    let tokens = if item.project_total_points > 0 {
        (item.total_earned as f64 / item.project_total_points as f64)
            * item.monthly_token_supply as f64
    } else {
        0.0
    };

    let is_claimed = claimed();
    let card_class = if is_claimed {
        "cycle-card cycle-card--claimed"
    } else {
        "cycle-card cycle-card--available"
    };
    let share_pct_str = format!("{:.2}", share_pct);
    let earned_fmt = format_points(item.total_earned);
    let tokens_fmt = format_tokens(tokens);

    let item_month = item.month.clone();
    let on_claim = move |_| {
        if is_claiming() || claimed() {
            return;
        }
        let month = item_month.clone();
        let pk = team_pk();
        is_claiming.set(true);
        spawn(async move {
            let res = request_team_claim_signature_handler(
                pk,
                TeamClaimSignatureRequest {
                    month,
                    wallet_address: String::new(),
                },
            )
            .await;
            if res.is_ok() {
                claimed.set(true);
            }
            is_claiming.set(false);
        });
    };

    rsx! {
        div { class: "{card_class}",
            div { class: "cycle-card__month",
                span { class: "cycle-card__month-label", "{month_name}" }
                span { class: "cycle-card__month-year", "{year}" }
            }
            div { class: "cycle-card__stats",
                div { class: "cycle-stat",
                    span { class: "cycle-stat__label", "{tr.stat_points}" }
                    span { class: "cycle-stat__value", "{earned_fmt}" }
                }
                div { class: "cycle-stat",
                    span { class: "cycle-stat__label", "{tr.stat_share}" }
                    span { class: "cycle-stat__value", "{share_pct_str}%" }
                }
                div { class: "cycle-stat cycle-stat--token",
                    span { class: "cycle-stat__label", "{tr.stat_tokens}" }
                    span { class: "cycle-stat__value",
                        "{tokens_fmt} "
                        small { "{token_symbol}" }
                    }
                }
            }
            div { class: "cycle-card__action",
                if is_claimed {
                    span { class: "cycle-card__claimed",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                        "{tr.claimed}"
                    }
                } else {
                    button {
                        class: "cycle-card__claim",
                        disabled: is_claiming() || !swap_enabled,
                        "aria-disabled": (!swap_enabled).to_string(),
                        title: if !swap_enabled { tr.swap_coming_soon.to_string() } else { String::new() },
                        onclick: on_claim,
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M5 6h14l-1 14H6L5 6z" }
                        }
                        "{tr.swap_all}"
                    }
                }
            }
        }
    }
}
