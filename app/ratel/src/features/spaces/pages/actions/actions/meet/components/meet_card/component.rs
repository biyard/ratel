use crate::features::spaces::pages::actions::actions::meet::components::meet_page::MeetActionTranslate;
use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::actions::SpaceActionStatus;
use crate::*;

#[derive(Clone, Copy, PartialEq)]
enum MeetPhase {
    Draft,
    Live,
    Ended,
}

fn derive_phase(action: &SpaceActionSummary) -> MeetPhase {
    let status = action
        .status
        .clone()
        .unwrap_or(SpaceActionStatus::Designing);
    match status {
        SpaceActionStatus::Designing => MeetPhase::Draft,
        SpaceActionStatus::Ongoing => MeetPhase::Live,
        SpaceActionStatus::Finish => MeetPhase::Ended,
    }
}

fn phase_str(phase: MeetPhase) -> &'static str {
    match phase {
        MeetPhase::Draft => "draft",
        MeetPhase::Live => "live",
        MeetPhase::Ended => "ended",
    }
}

#[component]
pub fn MeetActionCard(
    action: SpaceActionSummary,
    space_id: ReadSignal<SpacePartition>,
    #[props(default)] is_admin: bool,
) -> Element {
    let tr: MeetActionTranslate = use_translate();
    let nav = use_navigator();
    let phase = derive_phase(&action);
    let phase_string = phase_str(phase);
    let meet_id: SpaceMeetEntityType = action.action_id.clone().into();
    let meet_id_signal: ReadSignal<SpaceMeetEntityType> = use_signal(move || meet_id.clone()).into();

    let title_display = if action.title.is_empty() {
        "새 회의".to_string()
    } else {
        action.title.clone()
    };
    let _ = is_admin;

    let go = move |_| {
        nav.push(crate::Route::MeetActionPage {
            space_id: space_id(),
            meet_id: meet_id_signal(),
        });
    };

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div {
            class: "quest-card quest-card--meet",
            "data-testid": "action-card-meet",
            "data-kind": "meet",
            "data-type": "meet",
            "data-phase": phase_string,
            "data-prerequisite": action.prerequisite,
            onclick: go,

            // Hero icon (video camera for Meet)
            svg {
                class: "quest-card__hero",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "0.5",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polygon { points: "23 7 16 12 23 17 23 7" }
                rect {
                    x: "1",
                    y: "5",
                    width: "15",
                    height: "14",
                    rx: "2",
                    ry: "2",
                }
            }

            div { class: "quest-card__top",
                span { class: "quest-card__type quest-card__type--meet",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        polygon { points: "23 7 16 12 23 17 23 7" }
                        rect {
                            x: "1",
                            y: "5",
                            width: "15",
                            height: "14",
                            rx: "2",
                            ry: "2",
                        }
                    }
                    "MEET"
                }
                div { class: "quest-card__top-actions",
                    match phase {
                        MeetPhase::Draft => rsx! {
                            span { class: "quest-card__badge quest-card__badge--draft", "설정 중" }
                        },
                        MeetPhase::Live => rsx! {
                            span { class: "quest-card__badge quest-card__badge--live", "{tr.live_label}" }
                        },
                        MeetPhase::Ended => rsx! {
                            span { class: "quest-card__badge quest-card__badge--ended", "{tr.ended_label}" }
                        },
                    }
                    if action.credits > 0 {
                        span { class: "quest-card__badge quest-card__badge--credits",
                            "+{action.credits} CR"
                        }
                    }
                }
            }

            div { class: "quest-card__body",
                div { class: "quest-card__title", "{title_display}" }
                if !action.description.is_empty() {
                    div {
                        class: "quest-card__desc",
                        dangerous_inner_html: "{action.description}",
                    }
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
                match phase {
                    MeetPhase::Draft => rsx! {
                        span { class: "quest-card__cta quest-card__cta--start", "재진입" }
                    },
                    MeetPhase::Live => rsx! {
                        Button { disabled: true, "data-testid": "meet-card-join",
                            span { "{tr.live_cta} · " }
                            span { class: "quest-card__badge quest-card__badge--coming-soon", "{tr.coming_soon_badge}" }
                        }
                    },
                    MeetPhase::Ended => rsx! {
                        Button { disabled: true, "data-testid": "meet-card-archive",
                            span { "{tr.ended_cta} · " }
                            span { class: "quest-card__badge quest-card__badge--coming-soon", "{tr.coming_soon_badge}" }
                        }
                    },
                }
            }
        }
    }
}
