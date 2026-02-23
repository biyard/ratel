use crate::*;

const BASE_APP_MENU_NAMES: [SpaceAppName; 2] = [SpaceAppName::AllApps, SpaceAppName::General];
const APP_ICON_SIZE: &str = "20";
const APPS_ICON_CLASS: &str = "text-icon-primary [&>path]:stroke-current";
const GENERAL_ICON_CLASS: &str = "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current";
const INCENTIVE_POOL_ICON_CLASS: &str = "text-icon-primary [&>path]:fill-none [&>path]:stroke-current";

pub fn get_nav_item(
    space_id: SpacePartition,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    if role != SpaceUserRole::Creator {
        return None;
    }
    Some((
        all_apps_icon(),
        SpacePage::Apps,
        Route::from_app_name(&space_id, SpaceAppName::AllApps),
    ))
}

pub fn get_app_menu_items(
    space_id: SpacePartition,
    installed_apps: impl IntoIterator<Item = SpaceAppName>,
) -> Vec<(Element, SpaceAppName, NavigationTarget)> {
    let mut app_names = BASE_APP_MENU_NAMES
        .into_iter()
        .chain(installed_apps)
        .collect::<Vec<_>>();
    app_names.sort_unstable_by_key(|app_name| app_name.as_str());

    app_names
        .into_iter()
        .map(|app_name| app_menu_item(&space_id, app_name))
        .collect()
}

fn app_menu_item(
    space_id: &SpacePartition,
    app_name: SpaceAppName,
) -> (Element, SpaceAppName, NavigationTarget) {
    let spec = app_spec(app_name);
    (
        (spec.icon)(),
        app_name,
        ((spec.route)(space_id)).into(),
    )
}

#[derive(Clone, Copy)]
struct AppSpec {
    icon: fn() -> Element,
    route: fn(&SpacePartition) -> Route,
}

fn app_spec(app_name: SpaceAppName) -> AppSpec {
    match app_name {
        SpaceAppName::AllApps => AppSpec {
            icon: all_apps_icon,
            route: route_all_apps,
        },
        SpaceAppName::General => AppSpec {
            icon: general_icon,
            route: route_general,
        },
        SpaceAppName::IncentivePool => AppSpec {
            icon: incentive_pool_icon,
            route: route_incentive_pool,
        },
    }
}

#[component]
pub fn all_apps_icon() -> Element {
    rsx! {
        icons::layouts::Apps {
            width: APP_ICON_SIZE,
            height: APP_ICON_SIZE,
            class: APPS_ICON_CLASS,
        }
    }
}

fn general_icon() -> Element {
    rsx! {
        icons::settings::Settings2 {
            width: APP_ICON_SIZE,
            height: APP_ICON_SIZE,
            class: GENERAL_ICON_CLASS,
        }
    }
}

fn incentive_pool_icon() -> Element {
    rsx! {
        icons::ratel::Chest {
            width: APP_ICON_SIZE,
            height: APP_ICON_SIZE,
            class: INCENTIVE_POOL_ICON_CLASS,
        }
    }
}

fn route_all_apps(space_id: &SpacePartition) -> Route {
    Route::AllApps {
        space_id: space_id.clone(),
        rest: vec![],
    }
}

fn route_general(space_id: &SpacePartition) -> Route {
    Route::General {
        space_id: space_id.clone(),
        rest: vec![],
    }
}

fn route_incentive_pool(space_id: &SpacePartition) -> Route {
    Route::IncentivePool {
        space_id: space_id.clone(),
        rest: vec![],
    }
}

impl Route {
    fn to_app_route(space_id: &SpacePartition, app_name: SpaceAppName) -> Self {
        (app_spec(app_name).route)(space_id)
    }

    fn from_app_name(space_id: &SpacePartition, app_name: SpaceAppName) -> NavigationTarget {
        Self::to_app_route(space_id, app_name).into()
    }
}
