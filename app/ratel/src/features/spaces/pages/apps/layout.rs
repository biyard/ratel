use super::*;

#[component]
pub fn Layout(space_id: ReadSignal<SpacePartition>) -> Element {
    rsx! {
        ContextProvider { space_id, Outlet::<Route> {} }
    }
}
