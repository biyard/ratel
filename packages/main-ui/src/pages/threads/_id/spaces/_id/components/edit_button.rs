use bdk::prelude::*;
use dto::by_components::icons::edit::Edit1;

#[component]
pub fn SpaceEditButton(isedit: bool, onedit: EventHandler<bool>) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-row w-194 h-46 justify-start items-center px-16 py-12 bg-white rounded-l-[100px] rounded-r-[4px] gap-4",
            onclick: move |_| {
                onedit.call(!isedit);
            },
            Edit1 {
                class: "[&>path]:stroke-neutral-500 w-16 h-16",
                width: "16",
                height: "16",
                fill: "none",
            }

            div { class: "font-bold text-neutral-900 text-base/22",
                {if isedit { "Save" } else { "Edit" }}
            }
        }
    }
}
