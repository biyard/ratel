use space_app_main::{SpaceApp, SpaceAppType};

use crate::i18n::SpaceAppLayoutTranslate;
use crate::*;

// Space Layout Menu
pub fn get_nav_item(
    space_id: SpacePartition,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    let is_admin = SpaceApp::can_view(role).is_ok();
    match is_admin {
        true => Some((
            rsx! {
                icons::layouts::Apps {
                    width: "20",
                    height: "20",
                    class: "text-icon-primary [&>path]:stroke-current",
                }
            },
            SpacePage::Apps,
            Route::Main {
                space_id,
                rest: vec![],
            }
            .into(),
        )),
        false => None,
    }
}

// Space App Layout Menu
pub struct AppMenuItem {
    pub name: String,
    pub icon: Element,
    pub route: Route,
}

pub fn get_app_menu_items(
    space_id: SpacePartition,
    installed: &Vec<SpaceApp>,
    tr: &SpaceAppLayoutTranslate,
) -> Vec<AppMenuItem> {
    installed
        .iter()
        .map(|app| match app.app_type {
            SpaceAppType::General => AppMenuItem {
                name: tr.general.to_string(),
                icon: rsx! {
                    icons::settings::Settings2 {
                        width: "20",
                        height: "20",
                        class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current",
                    }
                },
                route: Route::General {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
            SpaceAppType::IncentivePool => AppMenuItem {
                name: tr.incentive_pool.to_string(),
                icon: rsx! {
                    icons::ratel::Chest {
                        width: "20",
                        height: "20",
                        class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current",
                    }
                },
                route: Route::IncentivePool {
                    space_id: space_id.clone(),
                    rest: vec![],
                },
            },
        })
        .collect()
}
