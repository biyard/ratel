mod space_label;
mod space_participant_button;
mod space_title;

pub use space_label::*;
pub use space_participant_button::*;
pub use space_title::*;

use crate::{controllers::get_space::get_space, *};

#[component]
pub fn SpaceTop(space_id: SpacePartition) -> Element {
    let space_pk = space_id.clone();
    let mut space = use_loader(move || get_space(space_id.clone()))?;
    let space_data = space();

    let show_participate = matches!(space_data.status, Some(common::SpaceStatus::InProgress))
        && !space_data.participated
        && space_data.can_participate;

    rsx! {
        div { class: "flex flex-row justify-between items-center px-[12px] py-[17.5px] min-h-[65px]",
            div { class: "flex flex-row w-full justify-start items-center gap-2.5",
                SpaceLabel { status: space_data.status }
                SpaceTitle { title: space_data.title.clone() }
            }

            if show_participate {
                SpaceParticipantButton {
                    space_id: space_pk.clone(),
                    on_participated: move |_| {
                        space.restart();
                    },
                }
            }
        }
    }
}
