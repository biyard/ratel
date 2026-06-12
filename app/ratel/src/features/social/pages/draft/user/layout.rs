use super::*;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        // No `p-5` frame — the drafts arena is full-bleed (owns its own
        // background + sticky topbar). The padding pushed the arena in and
        // left a gap above the header.
        div { class: "flex flex-col w-full min-h-screen bg-space-bg text-font-primary",
            div { class: "flex flex-col grow",
                Outlet::<crate::features::social::Route> {}
            }
        }
    }
}
