mod space_label;
mod space_participant_button;
mod space_title;

pub use space_label::*;
pub use space_participant_button::*;
pub use space_title::*;

use crate::{controllers::get_space::get_space, *};

#[component]
pub fn SpaceTop(space_id: SpacePartition) -> Element {
    let space = use_loader(move || get_space(space_id.clone()))?();
    debug!("space data: {:?}", space);

    rsx! {
        div { class: "flex flex-row justify-between items-center px-[12px] py-[17.5px] min-h-[65px]",
            div { class: "flex flex-row w-full justify-start items-center gap-2.5",
                SpaceLabel {}
                SpaceTitle {}
            }

            SpaceParticipantButton {}
        }
    }
}
