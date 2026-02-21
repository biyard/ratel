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
        div { class: "grid overflow-hidden grid-cols-7 w-full h-screen bg-space-bg text-font-primary",
            SpaceNav {
                logo: "https://metadata.ratel.foundation/logos/logo.png",
                menus,
            }
            div { class: "flex flex-col col-span-6 col-start-2 min-h-0",
                SpaceTop { space_id }
                div { class: "flex overflow-auto p-5 w-full top-[65px] grow bg-space-body-bg rounded-tl-[10px] h-[calc(100%-65px)]",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
