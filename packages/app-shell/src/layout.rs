use crate::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "antialiased bg-bg",
            AppMenu {}
            Outlet::<Route> {}
        }
    }
}
