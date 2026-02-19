use crate::*;
mod i18n;
use i18n::ListActionPageTranslate;
#[component]
pub fn ListActionPage(space_id: SpacePartition) -> Element {
    let tr: ListActionPageTranslate = use_translate();

    rsx! {
        div {
            id: "new-action-page",
            class: "flex flex-col gap-5 items-start w-full",
            h3 { "{tr.title}" }
        }
    }
}
