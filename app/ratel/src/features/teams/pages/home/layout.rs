use super::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col w-full",
            Outlet::<crate::features::teams::Route> {}
        }
    }
}
