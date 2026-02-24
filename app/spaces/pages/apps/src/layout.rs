use crate::*;

#[component]
pub fn SpaceAppsLayout(space_id: SpacePartition) -> Element {
    let apps_menus = use_loader(move || get_space_apps(space_id.clone()))?;

    rsx! {
        div { "Ok" }
    }
}
