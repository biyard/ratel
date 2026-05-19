use super::super::controllers::{list_reward_history_handler, RewardHistoryItem};
use super::super::*;
use crate::common::*;

fn format_yyyy_mm_dd(ts_millis: i64) -> String {
    use chrono::{DateTime, Utc};
    DateTime::<Utc>::from_timestamp_millis(ts_millis)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

translate! {
    RewardHistorySectionTranslate;

    title: { en: "Reward History", ko: "리워드 히스토리" },
    col_space: { en: "Space", ko: "스페이스" },
    col_action: { en: "Action", ko: "액션" },
    col_point: { en: "Points", ko: "획득 포인트" },
    col_date: { en: "Earned at", ko: "획득 시간" },
    empty: { en: "No reward history yet.", ko: "아직 리워드 히스토리가 없습니다." },
    pts_unit: { en: "pts", ko: "pts" },
    load_more: { en: "Load more", ko: "더보기" },
    loading: { en: "Loading…", ko: "불러오는 중…" },
}

#[component]
pub fn RewardHistorySection(username: ReadSignal<String>) -> Element {
    let tr: RewardHistorySectionTranslate = use_translate();

    let mut items = use_signal(Vec::<RewardHistoryItem>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut is_fetching = use_signal(|| false);
    let mut has_loaded_once = use_signal(|| false);

    use_hook(|| {
        spawn(async move {
            match list_reward_history_handler(username.peek().clone(), None).await {
                Ok(res) => {
                    items.set(res.items);
                    next_bookmark.set(res.bookmark);
                }
                Err(e) => {
                    crate::error!("initial reward history fetch failed: {e}");
                }
            }
            has_loaded_once.set(true);
        });
    });

    let list = items.read().clone();
    let has_more = next_bookmark.read().is_some();
    let fetching = *is_fetching.read();
    let loaded = has_loaded_once();
    let is_empty = loaded && list.is_empty();

    let on_load_more = move |_| {
        if *is_fetching.peek() {
            return;
        }
        let Some(bm) = next_bookmark.read().clone() else {
            return;
        };
        let name = username();
        is_fetching.set(true);
        spawn(async move {
            match list_reward_history_handler(name, Some(bm)).await {
                Ok(res) => {
                    items.with_mut(|v| v.extend(res.items));
                    next_bookmark.set(res.bookmark);
                }
                Err(e) => {
                    crate::error!("load_more reward history failed: {e}");
                }
            }
            is_fetching.set(false);
        });
    };

    rsx! {
        div {
            div { class: "section-head",
                span { class: "section-head__title", "{tr.title}" }
            }
            div { class: "reward-history",
                if is_empty {
                    div { class: "reward-history__empty",
                        div { class: "empty__desc", "{tr.empty}" }
                    }
                } else {
                    // Desktop: table header. Mobile media query collapses this
                    // into a stacked card layout where each row repeats the
                    // labels inline (see main.css `.reward-history__row`).
                    div { class: "reward-history__head",
                        span { class: "reward-history__col reward-history__col--space",
                            "{tr.col_space}"
                        }
                        span { class: "reward-history__col reward-history__col--action",
                            "{tr.col_action}"
                        }
                        span { class: "reward-history__col reward-history__col--point",
                            "{tr.col_point}"
                        }
                        span { class: "reward-history__col reward-history__col--date",
                            "{tr.col_date}"
                        }
                    }
                    div { class: "reward-history__list",
                        for item in list.iter() {
                            RewardHistoryRow {
                                key: "{item.transaction_id.clone().unwrap_or_default()}-{item.created_at}",
                                item: item.clone(),
                                pts_unit: tr.pts_unit.to_string(),
                                label_space: tr.col_space.to_string(),
                                label_action: tr.col_action.to_string(),
                                label_point: tr.col_point.to_string(),
                                label_date: tr.col_date.to_string(),
                            }
                        }
                    }
                }

                if has_more {
                    button {
                        class: "reward-history__loadmore",
                        disabled: fetching,
                        onclick: on_load_more,
                        if fetching {
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
}

#[component]
fn RewardHistoryRow(
    item: RewardHistoryItem,
    pts_unit: String,
    label_space: String,
    label_action: String,
    label_point: String,
    label_date: String,
) -> Element {
    let space_title = item.space_title.clone().unwrap_or_else(|| "—".to_string());
    let action_name = item.action_name.clone().unwrap_or_else(|| "—".to_string());
    let date = format_yyyy_mm_dd(item.created_at);
    let point_fmt = format_with_commas(item.point, None);
    let sign = if item.point >= 0 { "+" } else { "" };
    let amount_class = if item.point >= 0 {
        "reward-history__point reward-history__point--in"
    } else {
        "reward-history__point reward-history__point--out"
    };

    rsx! {
        div { class: "reward-history__row",
            div { class: "reward-history__cell reward-history__cell--space",
                span { class: "reward-history__cell-label", "{label_space}" }
                span { class: "reward-history__cell-value", "{space_title}" }
            }
            div { class: "reward-history__cell reward-history__cell--action",
                span { class: "reward-history__cell-label", "{label_action}" }
                span { class: "reward-history__cell-value", "{action_name}" }
            }
            div { class: "reward-history__cell reward-history__cell--point",
                span { class: "reward-history__cell-label", "{label_point}" }
                span { class: "{amount_class}",
                    "{sign}{point_fmt} "
                    small { "{pts_unit}" }
                }
            }
            div { class: "reward-history__cell reward-history__cell--date",
                span { class: "reward-history__cell-label", "{label_date}" }
                span { class: "reward-history__cell-value reward-history__cell-value--date",
                    "{date}"
                }
            }
        }
    }
}
