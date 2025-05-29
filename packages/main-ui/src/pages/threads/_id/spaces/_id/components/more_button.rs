use bdk::prelude::*;
use dto::by_components::icons::arrows::ShapeArrowDown;

#[component]
pub fn SpaceMoreButton() -> Element {
    rsx! {
        div { class: "flex flex-row w-48 h-46 justify-center items-center bg-neutral-500 rounded-l-[4px] rounded-r-[100px]",
            ShapeArrowDown { size: 16 }
        }
    }
}
