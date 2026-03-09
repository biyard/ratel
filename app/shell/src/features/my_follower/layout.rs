use crate::features::my_follower::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col w-full min-h-screen bg-space-bg text-font-primary",
            div { class: "flex flex-col grow p-5",
                Outlet::<Route> {}
            }
        }
    }
}
