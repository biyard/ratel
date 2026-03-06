use crate::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col w-full min-h-screen bg-component-bg text-web-font-primary",
            div { class: "flex flex-col grow p-5",
                Outlet::<Route> {}
            }
        }
    }
}
