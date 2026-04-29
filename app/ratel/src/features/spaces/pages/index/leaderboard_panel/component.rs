use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::{use_my_score, use_ranking};

#[component]
pub fn LeaderboardPanel(
    space_id: ReadSignal<SpacePartition>,
    open: bool,
    on_close: EventHandler<()>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    rsx! {

        div {
            class: "leaderboard-panel",
            "data-testid": "leaderboard-panel",
            "data-open": open,

            // Header
            div { class: "leaderboard-panel__header",
                span { class: "leaderboard-panel__title", "{tr.leaderboard}" }
                button {
                    aria_label: "Close leaderboard",
                    class: "leaderboard-panel__close",
                    onclick: move |_| on_close.call(()),
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        line {
                            x1: "18",
                            x2: "6",
                            y1: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            x2: "18",
                            y1: "6",
                            y2: "18",
                        }
                    }
                }
            }

            if open {
                SuspenseBoundary {
                    LeaderboardContent { space_id }
                }
            }
        }
    }
}

#[component]
fn LeaderboardContent(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    let ranking_loader = use_ranking();
    let my_score_loader = use_my_score();
    let user = use_space_user()?;

    let ranking = ranking_loader();
    let my_score = my_score_loader();

    let entries = &ranking.items;
    let top3: Vec<_> = entries.iter().take(3).cloned().collect();
    let rest: Vec<_> = entries.iter().skip(3).cloned().collect();

    let user = &user;
    let user_pk = user.pk.to_string();

    rsx! {
        // My Rank Card
        if my_score.rank > 0 {
            div { class: "leaderboard-my-rank",
                div { class: "leaderboard-my-rank__position", "#{my_score.rank}" }
                div { class: "leaderboard-my-rank__info",
                    div { class: "leaderboard-my-rank__label", "{tr.your_rank}" }
                    div { class: "leaderboard-my-rank__name", {user.username.clone()} }
                }
                div {
                    div { class: "leaderboard-my-rank__score", "{my_score.total_score}" }
                    div { class: "leaderboard-my-rank__score-label", "{tr.total_xp}" }
                }
            }

            // Score Breakdown
            div { class: "leaderboard-breakdown",
                div { class: "leaderboard-breakdown__item leaderboard-breakdown__item--poll",
                    span { class: "leaderboard-breakdown__value", "{my_score.poll_score}" }
                    span { class: "leaderboard-breakdown__label", "{tr.poll_label}" }
                }
                div { class: "leaderboard-breakdown__item leaderboard-breakdown__item--quiz",
                    span { class: "leaderboard-breakdown__value", "{my_score.quiz_score}" }
                    span { class: "leaderboard-breakdown__label", "{tr.quiz_label}" }
                }
                div { class: "leaderboard-breakdown__item leaderboard-breakdown__item--discussion",
                    span { class: "leaderboard-breakdown__value", "{my_score.discussion_score}" }
                    span { class: "leaderboard-breakdown__label", "{tr.discussion_label}" }
                }
                div { class: "leaderboard-breakdown__item leaderboard-breakdown__item--follow",
                    span { class: "leaderboard-breakdown__value", "{my_score.follow_score}" }
                    span { class: "leaderboard-breakdown__label", "{tr.follow_label}" }
                }
            }
        }

        // Scrollable body
        div { class: "leaderboard-panel__body",
            if entries.is_empty() {
                div { class: "leaderboard-empty",
                    span { class: "leaderboard-empty__text", "{tr.no_ranking_yet}" }
                }
            } else {
                span { class: "leaderboard-section-label", "{tr.top_participants}" }

                // Podium (top 3)
                if top3.len() >= 3 {
                    div { class: "leaderboard-podium",
                        PodiumEntry { entry: top3[1].clone(), place: 2 }
                        PodiumEntry { entry: top3[0].clone(), place: 1 }
                        PodiumEntry { entry: top3[2].clone(), place: 3 }
                    }
                    div { class: "leaderboard-divider" }
                }

                // Ranking list (4th onward, or all if <3 entries)
                div { class: "leaderboard-list",
                    if top3.len() < 3 {
                        for entry in entries.iter() {
                            LeaderboardEntry {
                                key: "{entry.rank}",
                                entry: entry.clone(),
                                is_me: &user_pk == &entry.user_pk,
                                you_label: tr.you.to_string(),
                            }
                        }
                    } else {
                        for entry in rest.iter() {
                            LeaderboardEntry {
                                key: "{entry.rank}",
                                entry: entry.clone(),
                                is_me: &user_pk == &entry.user_pk,
                                you_label: tr.you.to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PodiumEntry(
    entry: crate::features::activity::controllers::RankingEntryResponse,
    place: u32,
) -> Element {
    let place_class = match place {
        1 => "leaderboard-podium__entry--1st",
        2 => "leaderboard-podium__entry--2nd",
        _ => "leaderboard-podium__entry--3rd",
    };

    rsx! {
        div { class: "leaderboard-podium__entry {place_class}",
            div { class: "leaderboard-podium__avatar-wrap",
                if entry.avatar.is_empty() {
                    div { class: "leaderboard-podium__avatar-placeholder",
                        "{entry.name.chars().next().unwrap_or('?')}"
                    }
                } else {
                    img {
                        class: "leaderboard-podium__avatar",
                        src: "{entry.avatar}",
                        alt: "{entry.name}",
                    }
                }
                div { class: "leaderboard-podium__medal", "{place}" }
            }
            span { class: "leaderboard-podium__name", "{entry.name}" }
            span { class: "leaderboard-podium__score", "{entry.total_score} XP" }
        }
    }
}

#[component]
fn LeaderboardEntry(
    entry: crate::features::activity::controllers::RankingEntryResponse,
    is_me: bool,
    you_label: String,
) -> Element {
    let row_class = if is_me {
        "leaderboard-entry leaderboard-entry--me"
    } else {
        "leaderboard-entry"
    };

    rsx! {
        div { class: "{row_class}",
            span { class: "leaderboard-entry__rank", "{entry.rank}" }
            if entry.avatar.is_empty() {
                div { class: "leaderboard-entry__avatar-placeholder",
                    "{entry.name.chars().next().unwrap_or('?')}"
                }
            } else {
                img {
                    class: "leaderboard-entry__avatar",
                    src: "{entry.avatar}",
                    alt: "{entry.name}",
                }
            }
            span { class: "leaderboard-entry__name", "{entry.name}" }
            if is_me {
                span { class: "leaderboard-entry__me-badge", "{you_label}" }
            }
            span { class: "leaderboard-entry__score", "{entry.total_score} XP" }
        }
    }
}
