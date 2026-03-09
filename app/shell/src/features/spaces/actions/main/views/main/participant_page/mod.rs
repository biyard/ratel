use crate::features::spaces::actions::main::components::ActionCard;
use crate::features::spaces::actions::main::controllers::list_actions;
use crate::features::spaces::actions::main::*;
use i18n::ParticipantActionPageTranslate;

mod i18n;

#[component]
pub fn ParticipantActionPage(space_id: SpacePartition) -> Element {
    let tr: ParticipantActionPageTranslate = use_translate();

    let actions = use_loader({
        let space_id = space_id.clone();
        move || list_actions(space_id.clone())
    })?;

    rsx! {
        div {
            id: "participant-action-page",
            class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

            div { class: "flex flex-col gap-2.5 w-full max-w-[1024px] mx-auto",
                h3 { {tr.title} }

                // Action cards grid
                if !actions.is_empty() {
                    div { class: "grid grid-cols-1 gap-2.5 w-full tablet:grid-cols-2",
                        for action in actions.iter() {
                            ActionCard {
                                key: "{action.action_id}",
                                action: action.clone(),
                                space_id: space_id.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
