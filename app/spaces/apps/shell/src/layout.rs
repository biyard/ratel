use crate::*;
#[component]
pub fn SpaceAppsLayout(space_id: SpacePartition) -> Element {
    let _ = space_id;
    rsx! {
        div { class: "flex flex-col gap-2.5 w-full max-w-[1024px] mx-auto", Outlet::<Route> {} }

    }
}
