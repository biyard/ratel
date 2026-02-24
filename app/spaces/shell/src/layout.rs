use crate::*;

#[component]
pub fn SpaceLayout(space_id: SpacePartition) -> Element {
    let role = use_loader(get_user_role)?;

    use_context_provider(|| role);
    let role_value = role.read();

    let menus = vec![
        dashboard::get_nav_item(space_id.clone(), role_value.clone()),
        overview::get_nav_item(space_id.clone(), role_value.clone()),
        actions::get_nav_item(space_id.clone(), role_value.clone()),
        apps::get_nav_item(space_id.clone(), role_value.clone()),
        report::get_nav_item(space_id.clone(), role_value.clone()),
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
        Layover {}
    }
}
