use crate::*;
use crate::i18n::DashboardTranslate;

// ─── Card Header (shared across all card types) ─────────────

#[component]
fn CardHeader(icon: DashboardIcon, main_value: String, main_label: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-3 max-mobile:gap-2",

            // Left: Icon box
            div { class: "flex h-11 w-11 shrink-0 items-center justify-center max-mobile:h-9 max-mobile:w-9 rounded-[10px] {icon.bg_class()}",
                {render_icon(&icon)}
            }

            // Right: value + label
            div { class: "flex-1 min-w-0 text-right",
                div { class: "text-[24px] leading-[24px] max-mobile:text-[20px] max-mobile:leading-[22px] font-bold text-text-primary font-inter",
                    "{main_value}"
                }
                div { class: "mt-1 text-[15px] leading-[18px] max-mobile:text-[13px] max-mobile:leading-[16px] tracking-[-0.16px] font-semibold text-web-font-neutral font-raleway",
                    "{main_label}"
                }
            }
        }
    }
}

fn render_icon(icon: &DashboardIcon) -> Element {
    match icon {
        DashboardIcon::BarChart => rsx! {
            icons::graph::BarChart {
                width: "24", height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::Action => rsx! {
            icons::ratel::Thunder {
                width: "24", height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5",
            }
        },
        DashboardIcon::Participants => rsx! {
            icons::user::UserGroup {
                width: "24", height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::IncentivePool => rsx! {
            icons::ratel::Chest {
                width: "24", height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::Rewards => rsx! {
            icons::ratel::Clock {
                width: "24", height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>circle]:fill-none",
            }
        },
    }
}

// ─── DashboardCard (unified entry point) ─────────────────

#[component]
pub fn DashboardCard(ext: DashboardExtension) -> Element {
    match ext.data {
        DashboardComponentData::RankingTable(data) => rsx! {
            RankingTable { data }
        },
        DashboardComponentData::StatSummary(data) => {
            rsx! {
                div { class: "flex h-full w-full min-h-0 flex-col gap-5 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: data.main_value.clone(),
                        main_label: data.main_label.clone(),
                    }
                    StatSummaryContent { data }
                }
            }
        }
        DashboardComponentData::StatCard(data) => {
            let tr: DashboardTranslate = use_translate();
            rsx! {
                div { class: "flex h-full w-full min-h-0 flex-col gap-2.5 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: data.value.clone(),
                        main_label: data.trend_label.clone(),
                    }
                    StatCardContent { data, tr }
                }
            }
        }
        DashboardComponentData::ProgressList(data) => {
            let tr: DashboardTranslate = use_translate();
            rsx! {
                div { class: "flex h-full w-full min-h-0 flex-col gap-5 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: data.main_value.clone(),
                        main_label: tr.participation_action.to_string(),
                    }
                    ProgressListContent { data, tr }
                }
            }
        }
        DashboardComponentData::TabChart(data) => {
            let tr: DashboardTranslate = use_translate();
            rsx! {
                div { class: "grid h-full w-full min-h-0 grid-rows-[auto_auto_minmax(0,_1fr)] gap-4 max-mobile:gap-3 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: data.main_value.clone(),
                        main_label: tr.total_participants.to_string(),
                    }
                    TabChartContent { data }
                }
            }
        }
        DashboardComponentData::InfoCard(data) => {
            let tr: DashboardTranslate = use_translate();
            rsx! {
                div { class: "flex h-full w-full min-h-0 flex-col gap-2.5 p-[30px] max-tablet:p-5 max-mobile:p-4 rounded-2xl max-mobile:rounded-xl bg-web-card-bg",
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: data.main_value.clone(),
                        main_label: tr.points_available.to_string(),
                    }
                    InfoCardContent { data }
                }
            }
        }
    }
}

// ─── DashboardGrid ─────────────────────────────────

#[component]
pub fn DashboardGrid(extensions: Vec<DashboardExtension>) -> Element {
    let mut sorted = extensions;
    sorted.sort_by_key(|e| e.order());

    // Separate into: stacked (col 1), tall cards (cols 2-4), full-width (table)
    let mut stacked: Vec<DashboardExtension> = Vec::new();
    let mut tall_cards: Vec<DashboardExtension> = Vec::new();
    let mut full_width: Vec<DashboardExtension> = Vec::new();

    for ext in sorted {
        match &ext.data {
            DashboardComponentData::InfoCard(_) | DashboardComponentData::StatCard(_) => {
                stacked.push(ext);
            }
            DashboardComponentData::RankingTable(_) => {
                full_width.push(ext);
            }
            _ => {
                tall_cards.push(ext);
            }
        }
    }

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full h-full min-h-0 overflow-y-auto",
            // Top cards row: 4 columns
            div { class: "grid grid-cols-4 max-tablet:grid-cols-2 max-mobile:grid-cols-1 gap-2.5 shrink-0",
                // Column 1: stacked InfoCard + StatCard
                div { class: "flex flex-col gap-2.5",
                    for ext in stacked.into_iter() {
                        { let id = ext.id.clone(); rsx! { div { key: "{id}", DashboardCard { ext } } } }
                    }
                }
                // Columns 2-4: tall cards
                for ext in tall_cards.into_iter() {
                    { let id = ext.id.clone(); rsx! { div { key: "{id}", DashboardCard { ext } } } }
                }
            }
            // Bottom: full-width ranking table
            for ext in full_width.into_iter() {
                { let id = ext.id.clone(); rsx! { div { key: "{id}", class: "min-h-0", DashboardCard { ext } } } }
            }
        }
    }
}

// ─── StatSummary Content ─────────────────────────────

#[component]
fn StatSummaryContent(data: StatSummaryData) -> Element {
    rsx! {
        div { class: "flex-1 min-h-0 pr-1 space-y-5 max-mobile:space-y-3 overflow-y-auto",

            for item in data.items.iter() {
                div { class: "flex flex-col gap-0.5",

                    // Label + Icon + Value row
                    div { class: "flex items-center justify-between gap-2",
                        span { class: "truncate text-text-primary text-xs leading-4 font-medium font-raleway",
                            "{item.label}"
                        }
                        div { class: "flex items-center gap-1",
                            div { class: "h-[18px] w-[18px] shrink-0",
                                {render_summary_item_icon(&item.label, &item.icon)}
                            }
                            span { class: "text-text-primary text-xs leading-4 font-semibold font-inter",
                                "{item.value}"
                            }
                        }
                    }

                    // Trend row
                    div { class: "flex items-center gap-1 text-xs leading-4 font-medium font-inter",
                        if item.trend > 0.0 {
                            div { class: "h-[18px] w-[18px] shrink-0",
                                icons::arrows::ShapeArrowUp {
                                    width: "18", height: "18",
                                    class: "h-[18px] w-[18px] text-icon-primary [&>path]:fill-current",
                                }
                            }
                            if !item.trend_label.is_empty() {
                                span { class: "text-web-font-neutral", "{item.trend_label}" }
                            } else {
                                span { class: "text-web-font-neutral", "+{item.trend:.0}%" }
                            }
                        } else if item.trend < 0.0 {
                            span { class: "text-red-600", "↓ {item.trend:.0}%" }
                            if !item.trend_label.is_empty() {
                                span { class: "text-web-font-neutral", "{item.trend_label}" }
                            }
                        } else {
                            span { class: "text-text-primary", "→ 0%" }
                        }
                    }
                }
            }
        }
    }
}

fn render_summary_item_icon(label: &str, fallback_icon: &str) -> Element {
    match label {
        "Total Participants" => rsx! {
            icons::user::UserCheck {
                width: "18", height: "18",
                class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
            }
        },
        "Total Likes" => rsx! {
            icons::emoji::ThumbsUp {
                width: "18", height: "18",
                class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
            }
        },
        "Total Comments" => rsx! {
            icons::chat::RoundBubble {
                width: "18", height: "18",
                class: "h-[18px] w-[18px] [&>path]:fill-none text-icon-primary [&>path]:stroke-current [&>line]:stroke-current",
            }
        },
        "Total Actions" => rsx! {
            icons::ratel::Thunder {
                width: "18", height: "18",
                class: "h-[18px] w-[18px] text-icon-primary",
            }
        },
        _ if !fallback_icon.is_empty() => rsx! {
            span { class: "text-sm text-text-primary leading-[18px]", "{fallback_icon}" }
        },
        _ => rsx! {},
    }
}

// ─── StatCard Content ─────────────────────────────────

#[component]
fn StatCardContent(data: StatCardData, tr: DashboardTranslate) -> Element {
    rsx! {
        div { class: "flex-1 min-h-0 pr-1 space-y-0.5 overflow-y-auto",
            if !data.total_winners.is_empty() {
                div { class: "flex items-center justify-between text-text-primary",
                    span { class: "text-xs leading-4 font-medium font-raleway",
                        "{tr.total_winners}"
                    }
                    span { class: "text-xs leading-4 font-semibold font-inter",
                        "{data.total_winners}"
                    }
                }
            }

            if !data.rank_rate.is_empty() {
                p { class: "text-web-font-neutral text-xs leading-4 font-medium font-inter",
                    "{tr.rank_rate}"
                }
            }
        }

        if !data.incentive_pool.is_empty() {
            div { class: "border-t border-separator pt-2",
                div { class: "flex items-center justify-between gap-2",
                    span { class: "shrink-0 text-text-primary text-xs leading-4 font-medium font-raleway",
                        "{tr.incentive_pool}"
                    }
                    span { class: "min-w-0 max-w-[60%] truncate text-right text-text-primary text-xs leading-4 font-semibold font-inter",
                        "{data.incentive_pool}"
                    }
                }
            }
        }
    }
}

// ─── ProgressList Content ────────────────────────────

#[component]
fn ProgressListContent(data: ProgressListData, tr: DashboardTranslate) -> Element {
    rsx! {
        div { class: "flex-1 min-h-0 pr-1 space-y-5 max-mobile:space-y-3 overflow-y-auto",

            for item in data.items.iter() {
                div { class: "space-y-2 max-mobile:space-y-1.5",

                    // Label and Value Row
                    div { class: "flex items-center justify-between gap-2",
                        span { class: "min-w-0 truncate text-text-primary text-xs leading-4 font-medium font-raleway",
                            "{item.label}"
                        }
                        span { class: "shrink-0 text-text-primary text-xs leading-4 font-semibold font-inter",
                            "{item.current:.0}"
                        }
                    }

                    // Progress Bar
                    div { class: "h-2 w-full overflow-hidden rounded-full bg-popover",
                        div {
                            class: "h-full rounded-full transition-all duration-300",
                            style: "width: {(item.current / item.total * 100.0).min(100.0):.1}%; background-color: {item.color};",
                        }
                    }

                    // Completed Text
                    div { class: "flex items-center gap-1 text-xs text-web-font-neutral",
                        span { class: "text-xs leading-4 font-medium font-inter",
                            "{item.current:.0} / {item.total:.0}"
                        }
                        span { class: "text-xs leading-4 font-medium font-raleway",
                            "{tr.completed}"
                        }
                    }
                }
            }
        }
    }
}

// ─── TabChart Content ─────────────────────────────────

#[component]
fn TabChartContent(data: TabChartData) -> Element {
    let mut selected_tab = use_signal(|| 0usize);

    rsx! {
        // Tab Buttons
        div { class: "flex items-start justify-end overflow-hidden rounded-lg",

            for (idx, tab) in data.tabs.iter().enumerate() {
                {
                    let is_active = selected_tab() == idx;
                    let is_first = idx == 0;
                    let is_last = idx == data.tabs.len() - 1;

                    let base = "flex-1 px-4 py-1.5 text-sm font-bold font-raleway transition-all cursor-pointer text-center";
                    let active_class = if is_active {
                        " bg-web-btn-bg text-web-font-btn-b-w"
                    } else {
                        " border border-web-btn-storke text-text-primary"
                    };
                    let round = if is_first {
                        " rounded-l-lg"
                    } else if is_last {
                        " rounded-r-lg"
                    } else {
                        ""
                    };

                    let class = format!("{base}{active_class}{round}");

                    rsx! {
                        button {
                            class: "{class}",
                            onclick: move |_| selected_tab.set(idx),
                            "{tab.label}"
                        }
                    }
                }
            }
        }

        // Chart Content
        div { class: "min-h-0",
            if let Some(tab) = data.tabs.get(selected_tab()) {
                div { class: "h-full min-h-0 pr-1 space-y-3 max-mobile:space-y-2 overflow-y-auto",

                    for cat in tab.categories.iter() {
                        div { class: "flex flex-col gap-0.5",
                            // Label + Percentage row
                            div { class: "flex items-center justify-between text-xs leading-4 text-text-primary",
                                span { class: "font-medium font-inter", "{cat.name}" }
                                span { class: "font-semibold font-inter",
                                    if !cat.percentage.is_empty() {
                                        "{cat.percentage}"
                                    } else {
                                        "{cat.value:.1}%"
                                    }
                                }
                            }

                            // Progress Bar
                            div { class: "h-2 w-full bg-popover rounded-full overflow-hidden",
                                div {
                                    class: "h-full rounded-full transition-all duration-300",
                                    style: "width: {cat.value.min(100.0):.1}%; background-color: {cat.color};",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── InfoCard Content ─────────────────────────────────

#[component]
fn InfoCardContent(data: InfoCardData) -> Element {
    rsx! {
        div { class: "flex flex-1 min-h-0 flex-col justify-end gap-2 mt-4 max-mobile:mt-2 pr-1 overflow-y-auto",

            for item in data.items.iter() {
                {
                    let raw_label = item.label.trim();
                    let boost_label = raw_label.strip_prefix("✕ ").map(str::trim);

                    rsx! {
                        div { class: "flex items-center justify-between gap-2 text-text-primary",
                            if let Some(text) = boost_label {
                                div { class: "flex min-w-0 items-center gap-1",
                                    div { class: "h-[18px] w-[18px] shrink-0 [transform:rotate(-90deg)]",
                                        icons::validations::Clear {
                                            width: "18", height: "18",
                                            class: "h-full w-full text-icon-primary [&>path]:stroke-current",
                                        }
                                    }
                                    span { class: "truncate text-xs leading-4 font-medium font-inter",
                                        "{text}"
                                    }
                                }
                            } else {
                                span { class: "min-w-0 truncate text-xs leading-4 font-medium font-raleway", "{item.label}" }
                            }
                            span { class: "shrink-0 text-xs leading-4 font-semibold font-inter",
                                "{item.value}"
                            }
                        }
                    }
                }
            }
        }
    }
}
