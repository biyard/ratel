use crate::*;

pub fn get_nav_item(
    space_id: SpacePartition,
    role: SpaceUserRole,
    has_admin_access: bool,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    if role != SpaceUserRole::Creator && !has_admin_access {
        return None;
    }
    Some((
        rsx! {
            icons::layouts::Apps { class: "text-icon-primary [&>path]:stroke-current" }
        },
        SpacePage::Apps,
        Route::AllApps {
            space_id,
            rest: vec![],
        }
        .into(),
    ))
}

pub fn get_not_installed_menus(installed: Vec<SpaceApp>) -> Vec<SpaceAppType> {
    let installed_types: Vec<SpaceAppType> =
        installed.into_iter().map(|app| app.app_type).collect();

    let variants = SpaceAppType::VARIANTS;
    variants
        .into_iter()
        .copied()
        .filter(|variant| !installed_types.contains(variant))
        .collect()
}

pub struct AppMenuItem {
    pub name: String,
    pub icon: Element,
    pub route: Route,
}

pub fn get_app_menu_items(space_id: SpacePartition, installed: &Vec<SpaceApp>) -> Vec<AppMenuItem> {
    installed
        .iter()
        .map(|app| match app.app_type {
            SpaceAppType::General => AppMenuItem {
                name: SpaceAppType::General.to_string(),
                icon: rsx! {
                    icons::settings::Settings2 { class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current" }
                },
                route: Route::General {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
            SpaceAppType::IncentivePool => AppMenuItem {
                name: SpaceAppType::IncentivePool.to_string(),
                icon: rsx! {
                    icons::ratel::Chest { class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current" }
                },
                route: Route::IncentivePool {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
        })
        .collect()
}
