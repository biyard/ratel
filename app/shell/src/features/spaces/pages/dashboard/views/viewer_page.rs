use crate::features::spaces::pages::dashboard::controllers::list_dashboard_data_handler;

use super::*;

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let extension_loader = use_loader({
        let sid = space_id.clone();
        move || list_dashboard_data_handler(sid.clone())
    })?;

    let components = extension_loader.read().clone();

    if components.is_empty() {
        rsx! {
            div { class: "flex items-center justify-center w-full h-full text-web-font-neutral",
                "No dashboard data available."
            }
        }
    } else {
        rsx! {
            div { class: "w-full h-full min-h-0",
                DashboardGrid { components, space_id, is_creator: false }
            }
        }
    }
}
