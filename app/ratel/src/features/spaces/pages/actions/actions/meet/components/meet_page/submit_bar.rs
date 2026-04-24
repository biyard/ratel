use crate::features::spaces::pages::actions::SpaceActionStatus;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::*;

#[component]
pub fn MeetSubmitBar() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet {
        meet, mut publish, ..
    } = use_context::<UseMeet>();
    let current = meet();
    let mode = current.mode.clone();
    let status = current
        .space_action
        .status
        .clone()
        .unwrap_or(SpaceActionStatus::Designing);
    let is_published = !matches!(status, SpaceActionStatus::Designing);
    let label = if mode == MeetMode::Instant {
        tr.submit_start_now.to_string()
    } else {
        tr.submit_schedule.to_string()
    };

    rsx! {
        div { class: "create-bar",
            Button {
                "data-testid": "meet-submit-button",
                disabled: is_published,
                onclick: move |_| publish.call(),
                "{label}"
            }
        }
    }
}
