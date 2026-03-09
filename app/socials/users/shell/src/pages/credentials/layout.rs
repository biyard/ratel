use super::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col w-full min-h-screen bg-space-bg text-font-primary",
            div { class: "flex flex-col p-5 grow", Outlet::<crate::Route> {} }
        }
    }
}
