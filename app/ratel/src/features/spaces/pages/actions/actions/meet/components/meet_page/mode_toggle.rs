use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::*;

#[component]
pub fn MeetModeToggle() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet {
        meet,
        mut update_mode,
        ..
    } = use_context::<UseMeet>();
    let current = meet().mode.clone();

    let pick_scheduled = move |_| update_mode.call(MeetMode::Scheduled);
    let pick_instant = move |_| update_mode.call(MeetMode::Instant);

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.mode_label}" }
            }
            div {
                class: "mode-toggle",
                role: "tablist",
                "data-testid": "meet-mode-toggle",
                div {
                    class: "mode-option",
                    role: "tab",
                    "aria-selected": current == MeetMode::Scheduled,
                    "data-testid": "meet-mode-scheduled",
                    onclick: pick_scheduled,
                    span { class: "mode-option__title", "{tr.mode_scheduled}" }
                    p { class: "mode-option__desc", "{tr.mode_scheduled_desc}" }
                }
                div {
                    class: "mode-option",
                    role: "tab",
                    "aria-selected": current == MeetMode::Instant,
                    "data-testid": "meet-mode-instant",
                    onclick: pick_instant,
                    span { class: "mode-option__title", "{tr.mode_instant}" }
                    p { class: "mode-option__desc", "{tr.mode_instant_desc}" }
                }
            }
        }
    }
}
