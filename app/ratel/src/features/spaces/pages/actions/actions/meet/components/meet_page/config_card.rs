use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::components::{
    ActionDeleteButton, ActionDependencySelector, ActionRewardSetting, ActionStatusControl,
    PrerequisiteTile,
};
use crate::*;

#[component]
pub fn MeetConfigCard() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet {
        space_id,
        meet_id,
        meet,
        ..
    } = use_context::<UseMeet>();
    let current = meet().space_action.clone();
    let saved_credits = current.credits;
    let action_status = current.status.clone();
    let initial_prereq = current.prerequisite;
    let initial_depends = current.depends_on.clone();

    let action_id_str = meet_id().to_string();
    let action_id_signal: ReadSignal<String> = use_signal(move || action_id_str.clone()).into();

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.settings_label}" }
            }

            ActionDependencySelector {
                space_id,
                action_id: action_id_signal,
                initial_depends_on: initial_depends,
            }

            ActionRewardSetting {
                space_id,
                action_id: action_id_signal,
                saved_credits,
                action_status: action_status.clone(),
            }

            PrerequisiteTile {
                space_id,
                action_id: action_id_signal,
                initial_prerequisite: initial_prereq,
                on_changed: move |_| {},
            }

            ActionStatusControl {
                space_id,
                action_id: action_id_signal,
                initial_status: action_status.clone(),
                on_changed: move |_| {},
            }

            ActionDeleteButton { space_id: space_id(), action_id: meet_id().to_string() }
        }
    }
}
