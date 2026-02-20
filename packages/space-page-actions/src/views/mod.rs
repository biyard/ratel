use crate::{controllers::list_actions, *};
mod i18n;
use i18n::ListActionPageTranslate;
#[component]
pub fn ListActionPage(space_id: SpacePartition) -> Element {
    let tr: ListActionPageTranslate = use_translate();
    let actions = use_loader(move || list_actions(space_id.clone()))?;
    rsx! {
        div {
            id: "new-action-page",
            class: "flex flex-col gap-5 items-start w-full",
            h3 { "{tr.title}" }
            h3 { "{actions.len()}" }
        }
    }
}
