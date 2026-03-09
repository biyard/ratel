mod i18n;
use crate::features::spaces::actions::poll::*;
use i18n::PollViewerTranslate;

#[component]
pub fn PollViewerPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let tr: PollViewerTranslate = use_translate();
    let nav = navigator();
    let space_id_clone = space_id.clone();

    rsx! {
        div { class: "flex flex-col gap-5 items-center justify-center w-full min-h-[300px]",
            div { class: "flex flex-col items-center gap-3",
                span { class: "text-lg text-neutral-400", {tr.no_access} }
            }
            button {
                class: "px-4 py-2 rounded-lg bg-neutral-700 text-white hover:bg-neutral-600 transition-colors",
                onclick: move |_| {
                    nav.go_back();
                },
                {tr.btn_back}
            }
        }
    }
}
