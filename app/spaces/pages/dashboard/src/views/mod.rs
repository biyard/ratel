use crate::{api::fetch_dashboard_extensions, *};

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let extension_loader = use_loader({
        let sid = space_id.clone();
        move || fetch_dashboard_extensions(sid.clone())
    })?;

    let extensions = extension_loader.read().clone();

    if extensions.is_empty() {
        rsx! {
            div { class: "flex items-center justify-center w-full h-full text-web-font-neutral",
                "No dashboard data available."
            }
        }
    } else {
        rsx! {
            div { class: "w-full h-full min-h-0",
                DashboardGrid { extensions }
            }
        }
    }
}
