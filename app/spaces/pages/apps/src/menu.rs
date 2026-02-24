use crate::*;

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
    pub name: SpaceAppType,
    pub icon: Element,
    pub route: Route,
}

pub fn get_app_menu_items(space_id: SpacePartition, installed: Vec<SpaceApp>) -> Vec<AppMenuItem> {
    installed
        .into_iter()
        .map(|app| match app.app_type {
            SpaceAppType::General => AppMenuItem {
                name: SpaceAppType::General,
                icon: rsx! {
                    icons::settings::Settings2 { class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current" }
                },
                route: Route::General {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
            SpaceAppType::IncentivePool => AppMenuItem {
                name: SpaceAppType::IncentivePool,
                icon: rsx! {
                    icons::ratel::Chest { class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current" }
                },
                route: Route::IncentivePool {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
            SpaceAppType::File => AppMenuItem {
                name: SpaceAppType::File,
                icon: rsx! {
                    icons::file::File { class: "" }
                },
                route: Route::IncentivePool {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
        })
        .collect()
}
