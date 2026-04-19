use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::use_space;

#[component]
pub fn WaitingCard(prereqs: Vec<SpaceActionSummary>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let space = use_space()();
    let participants = format_number(space.participants);

    let started_at = space.started_at;
    let mut remaining_secs = use_signal(|| 0i64);

    use_effect(move || {
        #[cfg(feature = "web")]
        if let Some(start_ms) = started_at {
            spawn(async move {
                loop {
                    let now_ms = js_sys::Date::now() as i64;
                    let diff = (start_ms - now_ms) / 1000;
                    remaining_secs.set(diff.max(0));
                    if diff <= 0 {
                        break;
                    }
                    gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        }
    });

    let days = remaining_secs() / 86400;
    let hours = (remaining_secs() % 86400) / 3600;
    let minutes = (remaining_secs() % 3600) / 60;
    let seconds = remaining_secs() % 60;

    rsx! {
        div { class: "waiting-card", "data-testid": "card-waiting",

            // Success icon
            div { class: "waiting-card__icon",
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

            span { class: "waiting-card__heading", "{tr.waiting_heading}" }

            // Countdown timer
            if started_at.is_some() && remaining_secs() > 0 {
                div { class: "waiting-card__countdown",
                    span { class: "waiting-card__countdown-label", "{tr.waiting_starts_in}" }
                    div { class: "waiting-card__countdown-timer",
                        if days > 0 {
                            div { class: "waiting-card__countdown-unit",
                                span { class: "waiting-card__countdown-value", "{days}" }
                                span { class: "waiting-card__countdown-suffix", "{tr.waiting_days}" }
                            }
                        }
                        div { class: "waiting-card__countdown-unit",
                            span { class: "waiting-card__countdown-value", "{hours:02}" }
                            span { class: "waiting-card__countdown-suffix", "{tr.waiting_hours}" }
                        }
                        div { class: "waiting-card__countdown-unit",
                            span { class: "waiting-card__countdown-value", "{minutes:02}" }
                            span { class: "waiting-card__countdown-suffix", "{tr.waiting_minutes}" }
                        }
                        div { class: "waiting-card__countdown-unit",
                            span { class: "waiting-card__countdown-value", "{seconds:02}" }
                            span { class: "waiting-card__countdown-suffix", "{tr.waiting_seconds}" }
                        }
                    }
                }
            }

            // Participant count
            div { class: "waiting-card__participants",
                span { class: "waiting-card__participants-count", "{participants}" }
                span { class: "waiting-card__participants-label", "{tr.waiting_participants}" }
            }

            // Completed checklist summary
            if !prereqs.is_empty() {
                div { class: "waiting-card__list",
                    for action in prereqs.iter() {
                        div { key: "{action.action_id}", class: "waiting-item",
                            div { class: "waiting-item__icon",
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
                            div { class: "waiting-item__info",
                                span { class: "waiting-item__title", "{action.title}" }
                                span { class: "waiting-item__type",
                                    "{action.action_type.translate(&lang())}"
                                }
                            }
                        }
                    }
                }
            }

            // Status badge
            div { class: "waiting-card__status",
                div { class: "waiting-card__pulse" }
                "{tr.waiting_status}"
            }
        }
    }
}
