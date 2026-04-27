use crate::features::spaces::pages::apps::apps::analyzes::*;

#[derive(Clone, PartialEq)]
struct FollowRow {
    name: &'static str,
    handle: &'static str,
    avatar_seed: &'static str,
    rate_pct: &'static str,
    rate_label: &'static str,
    count_text: &'static str,
}

/// Follow panel — one card with a list of follow targets, each with a
/// horizontal bar showing completion %. Mockup data 1:1.
#[component]
pub fn FollowPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let rows: Vec<FollowRow> = vec![
        FollowRow {
            name: "김변호 변호사",
            handle: "@kimlaw",
            avatar_seed: "lawyer1",
            rate_pct: "90.5%",
            rate_label: "90.5%",
            count_text: "38 / 42",
        },
        FollowRow {
            name: "박판사 판사",
            handle: "@parkjudge",
            avatar_seed: "lawyer2",
            rate_pct: "76.2%",
            rate_label: "76.2%",
            count_text: "32 / 42",
        },
        FollowRow {
            name: "이검사 검사",
            handle: "@leeprosec",
            avatar_seed: "lawyer3",
            rate_pct: "64.3%",
            rate_label: "64.3%",
            count_text: "27 / 42",
        },
        FollowRow {
            name: "최교수 법학교수",
            handle: "@choiprof",
            avatar_seed: "lawyer4",
            rate_pct: "52.4%",
            rate_label: "52.4%",
            count_text: "22 / 42",
        },
        FollowRow {
            name: "정법사 변호사",
            handle: "@jeonglaw",
            avatar_seed: "lawyer5",
            rate_pct: "45.2%",
            rate_label: "45.2%",
            count_text: "19 / 42",
        },
        FollowRow {
            name: "한기자 법조 기자",
            handle: "@hanreporter",
            avatar_seed: "lawyer6",
            rate_pct: "28.6%",
            rate_label: "28.6%",
            count_text: "12 / 42",
        },
    ];

    rsx! {
        section { class: "panel", "data-panel": "follow", "data-active": "false",
            h1 { class: "main-title",
                span { class: "main-title__chip main-title__chip--follow",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path { d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                        circle { cx: "8.5", cy: "7", r: "4" }
                        line {
                            x1: "20",
                            y1: "8",
                            x2: "20",
                            y2: "14",
                        }
                        line {
                            x1: "23",
                            y1: "11",
                            x2: "17",
                            y2: "11",
                        }
                    }
                    "{tr.detail_panel_chip_follow}"
                }
                span { "data-follow-title": true, "{tr.detail_follow_title}" }
            }

            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_follow_card_title}" }
                    span { class: "card__count", "{tr.detail_follow_count_text}" }
                }
                div { class: "follow-legend",
                    span { class: "follow-legend__item",
                        span { class: "follow-legend__swatch follow-legend__swatch--done" }
                        "{tr.detail_follow_legend_done}"
                    }
                    span { class: "follow-legend__item",
                        span { class: "follow-legend__swatch follow-legend__swatch--miss" }
                        "{tr.detail_follow_legend_miss}"
                    }
                }
                div { class: "follow-list",
                    for (idx, row) in rows.iter().enumerate() {
                        FollowRowEl { key: "follow-{idx}", row: row.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn FollowRowEl(row: FollowRow) -> Element {
    let avatar = format!(
        "https://api.dicebear.com/7.x/personas/svg?seed={}",
        row.avatar_seed
    );
    let bar_style = format!("width: {};", row.rate_pct);
    rsx! {
        div { class: "follow-row",
            div { class: "follow-row__user",
                img { class: "follow-row__avatar", src: "{avatar}", alt: "" }
                div { class: "follow-row__meta",
                    span { class: "follow-row__name", "{row.name}" }
                    span { class: "follow-row__handle", "{row.handle}" }
                }
            }
            div { class: "follow-row__bar",
                div { class: "follow-row__bar-done", style: "{bar_style}" }
            }
            div { class: "follow-row__stats",
                span { class: "follow-row__rate", "{row.rate_label}" }
                span { class: "follow-row__count", "{row.count_text}" }
            }
        }
    }
}
