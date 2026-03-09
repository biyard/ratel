use crate::i18n::DashboardTranslate;
use crate::*;

// ─── Card Header (shared across all card types) ─────────────

#[component]
fn CardHeader(icon: DashboardIcon, main_value: String, main_label: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-3 max-mobile:gap-2",

            // Left: Icon box
            div { class: "flex h-11 w-11 shrink-0 items-center justify-center max-mobile:h-9 max-mobile:w-9 rounded-[10px] {icon.class()}",
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
                width: "24",
                height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::Action => rsx! {
            icons::ratel::Thunder {
                width: "24",
                height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::Participants => rsx! {
            icons::user::UserGroup {
                width: "24",
                height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::IncentivePool => rsx! {
            icons::ratel::Chest {
                width: "24",
                height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5 [&>path]:fill-none",
            }
        },
        DashboardIcon::Rewards => rsx! {
            icons::ratel::Reward {
                width: "24",
                height: "24",
                class: "h-6 w-6 max-mobile:h-5 max-mobile:w-5",
            }
        },
    }
}

// ─── DashboardCard (unified entry point) ─────────────────

#[component]
pub fn DashboardCard(
    data: DashboardComponentData,
    #[props(default = false)] is_creator: bool,
    space_id: SpacePartition,
) -> Element {
    let tr: DashboardTranslate = use_translate();

    match data {
        DashboardComponentData::RankingTable(data) => rsx! {
            RankingTable { data }
        },
        DashboardComponentData::StatSummary(data) => {
            rsx! {
                SpaceCard {
                    fill_height: true,
                    class: "flex flex-col gap-5".to_string(),
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: format!("{}", data.participants),
                        main_label: tr.space_views.to_string(),
                    }
                    StatSummaryContent { data, tr }
                }
            }
        }
        DashboardComponentData::StatCard(data) => {
            rsx! {
                SpaceCard {
                    fill_height: true,
                    class: "flex flex-col gap-2.5".to_string(),
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: data.value.clone(),
                        main_label: tr.incentive_pool.to_string(),
                    }
                    StatCardContent {
                        data,
                        tr,
                        is_creator,
                        space_id,
                    }
                }
            }
        }
        DashboardComponentData::ProgressList(data) => {
            rsx! {
                SpaceCard {
                    fill_height: true,
                    class: "flex flex-col gap-5".to_string(),
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: format!("{}", data.poll_count + data.post_count),
                        main_label: tr.participation_action.to_string(),
                    }
                    ProgressListContent { data, tr }
                }
            }
        }
        DashboardComponentData::TabChart(data) => {
            let card_class = if data.participants == 0 {
                "flex flex-col gap-2.5"
            } else {
                "grid grid-rows-[auto_auto_minmax(0,_1fr)] gap-4 max-mobile:gap-3"
            };

            rsx! {
                SpaceCard { fill_height: true, class: card_class.to_string(),
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: format!("{}", data.participants),
                        main_label: tr.total_participants.to_string(),
                    }
                    TabChartContent { data, tr }
                }
            }
        }
        DashboardComponentData::InfoCard(data) => {
            rsx! {
                SpaceCard {
                    fill_height: true,
                    class: "flex flex-col gap-2.5".to_string(),
                    CardHeader {
                        icon: data.icon.clone(),
                        main_value: format!("{}", data.total_points),
                        main_label: tr.points_available.to_string(),
                    }
                    InfoCardContent { data, tr }
                }
            }
        }
    }
}

// ─── DashboardGrid ─────────────────────────────────

#[component]
pub fn DashboardGrid(
    components: Vec<DashboardComponentData>,
    #[props(default = false)] is_creator: bool,
    space_id: SpacePartition,
) -> Element {
    let mut stacked: Vec<DashboardComponentData> = Vec::new();
    let mut tall_cards: Vec<DashboardComponentData> = Vec::new();
    let mut full_width: Vec<DashboardComponentData> = Vec::new();

    for c in components {
        match &c {
            DashboardComponentData::InfoCard(_) | DashboardComponentData::StatCard(_) => {
                stacked.push(c);
            }
            DashboardComponentData::RankingTable(_) => {
                full_width.push(c);
            }
            _ => {
                tall_cards.push(c);
            }
        }
    }
    let stacked_len = stacked.len();
    let stacked_container_class = if stacked_len == 1 {
        "flex h-full flex-col gap-2.5"
    } else {
        "flex flex-col gap-2.5"
    };
    let top_level_count = tall_cards.len() + usize::from(stacked_len > 0);
    let top_grid_class = match top_level_count {
        1 => "grid grid-cols-1 max-tablet:grid-cols-1 max-mobile:grid-cols-1 gap-2.5 shrink-0",
        2 => "grid grid-cols-2 max-tablet:grid-cols-2 max-mobile:grid-cols-1 gap-2.5 shrink-0",
        3 => "grid grid-cols-3 max-tablet:grid-cols-2 max-mobile:grid-cols-1 gap-2.5 shrink-0",
        _ => "grid grid-cols-4 max-tablet:grid-cols-2 max-mobile:grid-cols-1 gap-2.5 shrink-0",
    };

    rsx! {
        div { class: "flex flex-col gap-2.5 w-full h-full min-h-0 overflow-y-auto",
            div { class: top_grid_class,
                if stacked_len > 0 {
                    div { class: stacked_container_class,
                        for (idx , data) in stacked.into_iter().enumerate() {
                            {
                                let sid = space_id.clone();
                                let item_class = if stacked_len == 1 { "h-full" } else { "" };
                                rsx! {
                                    div { key: "{idx}", class: item_class,
                                        DashboardCard { data, is_creator, space_id: sid }
                                    }
                                }
                            }
                        }
                    }
                }
                for (idx , data) in tall_cards.into_iter().enumerate() {
                    {
                        let sid = space_id.clone();
                        rsx! {
                            div { key: "{idx}",
                                DashboardCard { data, is_creator, space_id: sid }
                            }
                        }
                    }
                }
            }
            for (idx , data) in full_width.into_iter().enumerate() {
                {
                    let sid = space_id.clone();
                    rsx! {
                        div { key: "{idx}", class: "min-h-0",
                            DashboardCard { data, is_creator, space_id: sid }
                        }
                    }
                }
            }
        }
    }
}

// ─── StatSummary Content ─────────────────────────────

#[component]
fn StatSummaryContent(data: StatSummaryData, tr: DashboardTranslate) -> Element {
    let items: Vec<(&str, i64, Element)> = vec![
        (
            &tr.total_participants,
            data.participants,
            rsx! {
                icons::user::UserCheck {
                    width: "18", height: "18",
                    class: "size-4.5 [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
                }
            },
        ),
        (
            &tr.total_likes,
            data.likes,
            rsx! {
                icons::emoji::ThumbsUp {
                    width: "18", height: "18",
                    class: "size-4.5 [&>path]:fill-none text-icon-primary [&>path]:stroke-current",
                }
            },
        ),
        (
            &tr.total_comments,
            data.comments,
            rsx! {
                icons::chat::RoundBubble {
                    width: "18", height: "18",
                    class: "size-4.5 [&>path]:fill-none text-icon-primary [&>path]:stroke-current [&>line]:stroke-current",
                }
            },
        ),
        (
            &tr.total_actions,
            data.total_actions,
            rsx! {
                icons::ratel::Thunder {
                    width: "18", height: "18",
                    class: "size-4.5 text-icon-primary [&>path]:stroke-icon-primary [&>path]:fill-none",
                }
            },
        ),
    ];

    rsx! {
        div { class: "flex-1 min-h-0 pr-1 space-y-5 max-mobile:space-y-3 overflow-y-auto",
            for (label , value , icon) in items.into_iter() {
                div { class: "flex flex-col gap-0.5",
                    div { class: "flex items-center justify-between gap-2",
                        span { class: "truncate text-text-primary text-xs leading-4 font-medium font-raleway",
                            {label}
                        }
                        div { class: "flex items-center gap-1",
                            div { class: "size-4.5 shrink-0", {icon} }
                            span { class: "text-text-primary text-xs leading-4 font-semibold font-inter",
                                "{value}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── StatCard Content ─────────────────────────────────

#[component]
fn StatCardContent(
    data: StatCardData,
    tr: DashboardTranslate,
    #[props(default = false)] is_creator: bool,
    space_id: SpacePartition,
) -> Element {
    let incentive_not_setup = data.incentive_pool.is_empty();
    let apps_url = format!("/spaces/{}/apps/", space_id);

    rsx! {
        div { class: "flex-1 min-h-0 pr-1 space-y-0.5 overflow-y-auto",
            if !data.total_winners.is_empty() {
                div { class: "flex items-center justify-between text-text-primary",
                    span { class: "text-xs leading-4 font-medium font-raleway", "{tr.total_winners}" }
                    span { class: "text-xs leading-4 font-semibold font-inter", "{data.total_winners}" }
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

        if is_creator && incentive_not_setup {
            Link {
                to: NavigationTarget::Internal(apps_url),
                class: "flex w-full items-center justify-center rounded-[10px] bg-violet-500 px-5 py-3 text-sm font-bold font-raleway text-web-font-ab-bk transition-colors hover:bg-violet-600",
                "{tr.setup_now}"
            }
        }
    }
}

// ─── ProgressList Content ────────────────────────────

#[component]
fn ProgressListContent(data: ProgressListData, tr: DashboardTranslate) -> Element {
    let total = (data.poll_count + data.post_count).max(1) as f64;

    rsx! {
        div {
            class: "flex-1 min-h-0 pr-1 space-y-5 max-mobile:space-y-3 overflow-y-auto",
            // Poll progress
            div { class: "space-y-2 max-mobile:space-y-1.5",
                div { class: "flex items-center justify-between gap-2",
                    span { class: "min-w-0 truncate text-text-primary text-xs leading-4 font-medium font-raleway",
                        "{tr.poll_completion}"
                    }
                    span { class: "shrink-0 text-text-primary text-xs leading-4 font-semibold font-inter",
                        "{data.poll_count}"
                    }
                }
                div { class: "h-2 w-full overflow-hidden rounded-full bg-popover",
                    div {
                        class: "h-full rounded-full transition-all duration-300 bg-primary",
                        style: "width: {(data.poll_count as f64 / total * 100.0).min(100.0):.1}%;",
                    }
                }
                div { class: "flex items-center gap-1 text-xs text-web-font-neutral",
                    span { class: "text-xs leading-4 font-medium font-inter",
                        "{data.poll_count} / {data.poll_count + data.post_count}"
                    }
                    span { class: "text-xs leading-4 font-medium font-raleway", "{tr.completed}" }
                }
            }
        
        // Discussion progress
        // div { class: "space-y-2 max-mobile:space-y-1.5",
        //     div { class: "flex items-center justify-between gap-2",
        //         span { class: "min-w-0 truncate text-text-primary text-xs leading-4 font-medium font-raleway",
        //             "{tr.discussion_completion}"
        //         }
        //         span { class: "shrink-0 text-text-primary text-xs leading-4 font-semibold font-inter",
        //             "{data.post_count}"
        //         }
        //     }
        //     div { class: "h-2 w-full overflow-hidden rounded-full bg-popover",
        //         div {
        //             class: "h-full rounded-full transition-all duration-300 bg-primary",
        //             style: "width: {(data.post_count as f64 / total * 100.0).min(100.0):.1}%;",
        //         }
        //     }
        //     div { class: "flex items-center gap-1 text-xs text-web-font-neutral",
        //         span { class: "text-xs leading-4 font-medium font-inter",
        //             "{data.post_count} / {data.poll_count + data.post_count}"
        //         }
        //         span { class: "text-xs leading-4 font-medium font-raleway", "{tr.completed}" }
        //     }
        // }
        }
    }
}

#[component]
fn EmptyStateLine(message: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-2",
            span { class: "min-w-0 text-text-primary text-xs leading-4 font-medium",
                "{message}"
            }
        }
    }
}

// ─── TabChart Content ─────────────────────────────────

#[component]
fn TabChartContent(data: TabChartData, tr: DashboardTranslate) -> Element {
    let mut selected_tab = use_signal(|| 0usize);
    let has_tabs = data.participants > 0 && !data.tabs.is_empty();

    rsx! {
        if has_tabs {
            div { class: "flex items-start justify-end overflow-hidden rounded-lg",
                for (idx , tab) in data.tabs.iter().enumerate() {
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
                            button { class: "{class}", onclick: move |_| selected_tab.set(idx), "{tab.label}" }
                        }
                    }
                }
            }
        }

        div { class: "min-h-0",
            if data.participants == 0 {
                div { class: "h-full min-h-0 pr-1 overflow-y-auto pt-2.5",
                    EmptyStateLine { message: tr.not_available_participants.to_string() }
                }
            } else if let Some(tab) = data.tabs.get(selected_tab()) {
                div { class: "h-full min-h-0 pr-1 space-y-3 max-mobile:space-y-2 overflow-y-auto",
                    for cat in tab.categories.iter() {
                        div { class: "flex flex-col gap-0.5",
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
fn InfoCardContent(data: InfoCardData, tr: DashboardTranslate) -> Element {
    if data.items.is_empty() {
        return rsx! {
            div { class: "flex flex-1 flex-col min-h-[120px] max-mobile:min-h-[96px] pr-1 overflow-y-auto pt-2.5",
                EmptyStateLine { message: tr.not_available_rewards.to_string() }
            }
        };
    }

    rsx! {
        div { class: "flex flex-1 min-h-[120px] max-mobile:min-h-[96px] flex-col justify-end gap-2 mt-4 max-mobile:mt-2 pr-1 overflow-y-auto",
            for item in data.items.iter() {
                div { class: "flex items-center justify-between gap-2 text-text-primary",
                    span { class: "min-w-0 truncate text-xs leading-4 font-medium font-raleway",
                        "{item.label}"
                    }
                    span { class: "shrink-0 text-xs leading-4 font-semibold font-inter",
                        "{item.value}"
                    }
                }
            }
        }
    }
}
