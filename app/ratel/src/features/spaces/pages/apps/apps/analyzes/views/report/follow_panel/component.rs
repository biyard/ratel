use crate::features::spaces::pages::apps::apps::analyzes::*;

#[component]
pub fn FollowPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let ctrl = use_context::<UseAnalyzeReportDetail>();
    let detail = ctrl.detail.read().clone();
    let result = detail.result.unwrap_or_default();
    let respondent = result.respondent_count.max(1);
    let aggregates = result.follow_aggregates;

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
                    span { class: "card__count", "{respondent}명 {tr.detail_sb_item_meta_participants}" }
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
                if aggregates.is_empty() {
                    div { class: "card__hint", "{tr.detail_panel_empty_follow}" }
                } else {
                    div { class: "follow-list",
                        for (idx, t) in aggregates.iter().enumerate() {
                            FollowTargetRow {
                                key: "follow-{idx}-{t.user_pk}",
                                target: t.clone(),
                                respondent_total: respondent,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FollowTargetRow(target: FollowTargetAggregate, respondent_total: i64) -> Element {
    let total = respondent_total.max(1) as f64;
    let pct = (target.count as f64 / total * 100.0).clamp(0.0, 100.0);
    let bar_style = format!("width: {:.1}%;", pct);
    let display = if target.display_name.is_empty() {
        target.username.clone()
    } else {
        target.display_name.clone()
    };
    let handle = if target.username.is_empty() {
        String::new()
    } else {
        format!("@{}", target.username)
    };
    let avatar = if target.profile_url.is_empty() {
        format!(
            "https://api.dicebear.com/7.x/personas/svg?seed={}",
            target.user_pk
        )
    } else {
        target.profile_url.clone()
    };

    rsx! {
        div { class: "follow-row",
            div { class: "follow-row__user",
                img { class: "follow-row__avatar", src: "{avatar}", alt: "" }
                div { class: "follow-row__meta",
                    span { class: "follow-row__name", "{display}" }
                    span { class: "follow-row__handle", "{handle}" }
                }
            }
            div { class: "follow-row__bar",
                div { class: "follow-row__bar-done", style: "{bar_style}" }
            }
            div { class: "follow-row__stats",
                span { class: "follow-row__rate", "{pct:.1}%" }
                span { class: "follow-row__count", "{target.count} / {respondent_total}" }
            }
        }
    }
}
