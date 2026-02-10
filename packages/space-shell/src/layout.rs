use crate::*;

#[component]
pub fn SpaceLayout(space_id: SpacePartition) -> Element {
    // FIXME: Temporarily set role to Viewer
    let role = SpaceUserRole::Creator;

    let menus = vec![
        dashboard::get_nav_item(space_id.clone(), role),
        overview::get_nav_item(space_id.clone(), role),
        actions::get_nav_item(space_id.clone(), role),
        apps::get_nav_item(space_id.clone(), role),
        report::get_nav_item(space_id.clone(), role),
    ]
    .into_iter()
    .map(|s| s.try_into())
    .filter(|s| s.is_ok())
    .map(|s| s.unwrap())
    .collect::<Vec<SpaceNavItem>>();

    rsx! {
        div { class: "bg-space-bg w-full grid grid-cols-7 h-screen text-font-primary",
            SpaceNav {
                logo: "https://metadata.ratel.foundation/logos/logo.png",
                menus,
            }

            div { class: "col-span-6 flex flex-col",
                SpaceTop {}
                div { class: "bg-space-body-bg flex grow rounded-tl-[10px] overflow-auto p-5",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
