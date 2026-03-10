use super::*;

#[component]
pub fn PanelCreatorPage(space_id: SpacePartition) -> Element {
    let _ = space_id;

    rsx! {
        div { class: "flex w-full flex-col gap-5", "Panel Creator Page" }
    }
}
