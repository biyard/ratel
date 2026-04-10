use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::controllers::SpaceResponse;
use crate::features::spaces::space_common::hooks::use_space;

const DEFAULT_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

#[component]
pub fn ActionDashboard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let space = use_space()();
    let actions = use_loader(move || async move { list_actions(space_id()).await })?;
    let actions = actions();

    let logo = if space.logo.is_empty() {
        DEFAULT_LOGO.to_string()
    } else {
        space.logo.clone()
    };
    let status_text = match space.status {
        Some(SpaceStatus::Open) => tr.status_open.to_string(),
        Some(SpaceStatus::Ongoing) => tr.status_ongoing.to_string(),
        Some(SpaceStatus::Finished) => tr.status_finished.to_string(),
        _ => tr.status_open.to_string(),
    };

    let participant_count = space.quota - space.remains;
    let participants = format_number(participant_count);
    let remaining = format_number(space.remains);
    let rewards_str = space
        .rewards
        .map(|r| format_number(r))
        .unwrap_or_else(|| "0".to_string());

    let incomplete: Vec<_> = actions
        .iter()
        .filter(|a| !a.user_participated)
        .cloned()
        .collect();
    let completed: Vec<_> = actions
        .iter()
        .filter(|a| a.user_participated)
        .cloned()
        .collect();
    let total = actions.len();
    let done = completed.len();
    let progress_pct = if total > 0 {
        (done as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    let mut show_archive = use_signal(|| false);
    let mut active_panel = use_signal(|| ActivePanel::None);
    let lang = use_language();

    let overview_open = active_panel() == ActivePanel::Overview;
    let settings_open = active_panel() == ActivePanel::Settings;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "arena", "data-testid": "action-dashboard",
            // Top bar
            div { class: "arena-topbar",
                div { class: "arena-topbar__brand",
                    img {
                        class: "arena-topbar__logo",
                        src: "{logo}",
                        alt: "Space logo",
                    }
                    span { class: "arena-topbar__title", "{space.title}" }
                    span { class: "arena-topbar__status", "{status_text}" }
                }
                div { class: "arena-topbar__actions",
                    button {
                        aria_label: "{tr.overview}",
                        aria_pressed: overview_open,
                        class: "hud-btn",
                        "data-testid": "btn-overview",
                        onclick: move |_| {
                            if active_panel() == ActivePanel::Overview {
                                active_panel.set(ActivePanel::None);
                            } else {
                                active_panel.set(ActivePanel::Overview);
                            }
                        },
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                            polyline { points: "14 2 14 8 20 8" }
                            line {
                                x1: "16",
                                x2: "8",
                                y1: "13",
                                y2: "13",
                            }
                            line {
                                x1: "16",
                                x2: "8",
                                y1: "17",
                                y2: "17",
                            }
                            line {
                                x1: "10",
                                x2: "8",
                                y1: "9",
                                y2: "9",
                            }
                        }
                    }
                    button {
                        aria_label: "{tr.settings}",
                        aria_pressed: settings_open,
                        class: "hud-btn",
                        "data-testid": "btn-settings",
                        onclick: move |_| {
                            if active_panel() == ActivePanel::Settings {
                                active_panel.set(ActivePanel::None);
                            } else {
                                active_panel.set(ActivePanel::Settings);
                            }
                        },
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                            circle { cx: "12", cy: "12", r: "3" }
                        }
                    }
                }
            }

            div { class: "quest-label",
                span { class: "quest-label__title", "{tr.your_quests}" }
            }

            if incomplete.is_empty() {
                div { class: "quest-empty",
                    div { class: "quest-empty__icon",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                            polyline { points: "22 4 12 14.01 9 11.01" }
                        }
                    }
                    div { class: "quest-empty__text", "{tr.all_quests_done}" }
                }
            } else {
                // Carousel — same IDs as space-participant.html
                div { class: "carousel-wrapper",
                    div { class: "carousel-track", id: "carousel-track",
                        for action in incomplete.iter() {
                            QuestCard {
                                key: "{action.action_id}",
                                action: action.clone(),
                                space_id,
                            }
                        }
                    }
                }

                // Dots
                div { class: "carousel-dots", id: "carousel-dots",
                    for action in incomplete.iter() {
                        button {
                            class: "carousel-dot",
                            "data-type": quest_type_css(&action.action_type),
                        }
                    }
                }
            }

            // Bottom bar
            div { class: "bottom-bar",
                div { class: "quest-progress",
                    span { class: "quest-progress__label", "{tr.quest_progress}" }
                    div { class: "quest-progress__bar-wrap",
                        div {
                            class: "quest-progress__bar",
                            width: "{progress_pct}%",
                        }
                    }
                    span { class: "quest-progress__fraction", "{done} / {total}" }
                }

                button {
                    class: "archive-btn",
                    "data-testid": "btn-archive",
                    aria_label: "Completed quests",
                    onclick: move |_| {
                        show_archive.set(!show_archive());
                    },
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                    }
                    if done > 0 {
                        span { class: "archive-btn__count", "{done}" }
                    }
                }
            }

            // Archive panel
            div {
                class: "archive-panel",
                "data-open": show_archive(),
                "data-testid": "archive-panel",
                div { class: "archive-panel__header",
                    span { class: "archive-panel__title", "{tr.completed}" }
                    button {
                        class: "archive-panel__close",
                        onclick: move |_| {
                            show_archive.set(false);
                        },
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
                div { class: "archive-panel__list",
                    if completed.is_empty() {
                        div { class: "archive-panel__empty", "{tr.no_completed_yet}" }
                    } else {
                        for action in completed.iter() {
                            div { class: "archive-item",
                                div { class: "archive-item__info",
                                    div { class: "archive-item__title", "{action.title}" }
                                    div { class: "archive-item__meta",
                                        "{action.action_type.translate(&lang())} · {action.credits} CR"
                                    }
                                }
                                div { class: "archive-item__check",
                                    svg {
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        view_box: "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                                        polyline { points: "22 4 12 14.01 9 11.01" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Panels
            OverviewPanel {
                open: overview_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
                space: space.clone(),
                participants: participants.clone(),
                remaining: remaining.clone(),
                rewards: rewards_str.clone(),
            }

            SettingsPanel {
                open: settings_open,
                on_close: move |_| {
                    active_panel.set(ActivePanel::None);
                },
            }
        }
    }
}

/// Quest card using the same class names as space-participant.html
#[component]
fn QuestCard(action: SpaceActionSummary, space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();
    let type_css = quest_type_css(&action.action_type);
    let card_modifier = format!("quest-card--{type_css}");
    let type_badge_class = match action.action_type {
        SpaceActionType::Poll => "quest-card__type--poll",
        SpaceActionType::TopicDiscussion => "quest-card__type--discussion",
        SpaceActionType::Quiz => "quest-card__type--quiz",
        SpaceActionType::Follow => "quest-card__type--follow",
    };

    rsx! {
        div {
            class: "quest-card {card_modifier}",
            "data-type": type_css,
            "data-prerequisite": action.prerequisite,
            "data-testid": "quest-card-{action.action_id}",
            onclick: move |_| {
                let route = action.get_url(&space_id());
                nav.push(route);
            },

            div { class: "quest-card__top",
                span { class: "quest-card__type {type_badge_class}",
                    "{action.action_type.translate(&lang())}"
                }
                div { class: "quest-card__badges",
                    if action.prerequisite {
                        span { class: "quest-card__badge quest-card__badge--prerequisite",
                            "{tr.required_label}"
                        }
                    }
                    if action.credits > 0 {
                        span { class: "quest-card__badge quest-card__badge--credits",
                            "+{action.credits} CR"
                        }
                    }
                }
            }

            div { class: "quest-card__body",
                div { class: "quest-card__title", "{action.title}" }
                if !action.description.is_empty() {
                    div { class: "quest-card__desc", "{action.description}" }
                }
            }

            div { class: "quest-card__footer",
                div { class: "quest-card__reward",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M12 6v12" }
                        path { d: "M16 10H8" }
                    }
                    "{action.credits} CR"
                }
                button { class: "quest-card__cta", "{tr.start_quest}" }
            }
        }
    }
}

fn quest_type_css(t: &SpaceActionType) -> &'static str {
    match t {
        SpaceActionType::Poll => "poll",
        SpaceActionType::TopicDiscussion => "discuss",
        SpaceActionType::Quiz => "quiz",
        SpaceActionType::Follow => "follow",
    }
}
