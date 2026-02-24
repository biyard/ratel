use crate::*;
use dioxus::router::components::child_router::ChildRouter;
use space_app_all_apps::Route as AllAppsRoute;
use space_app_general::Route as GeneralRoute;
use space_app_incentive_pool::Route as IncentivePoolRoute;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps")]
        #[route("/all_apps/:..rest")]
        AllApps { space_id: SpacePartition, rest: Vec<String> },
        #[route("/general/:..rest")]
        General { space_id: SpacePartition, rest: Vec<String> },
        #[route("/incentive_pool/:..rest")]
        IncentivePool { space_id: SpacePartition, rest: Vec<String> },
}

macro_rules! define_apps_route_wrapper {
    ($wrapper_name:ident, $route_ty:ty) => {
        #[component]
        fn $wrapper_name(space_id: SpacePartition, rest: Vec<String>) -> Element {
            let router = use_context::<dioxus::router::RouterContext>();
            let route: $route_ty = router.current();
            let installed_apps = use_loader({
                let space_id = space_id.clone();
                move || get_space_apps(space_id.clone())
            })?;
            let app_ids = installed_apps()
                .apps
                .into_iter()
                .map(|app| app.name)
                .collect::<Vec<_>>();
            use_sync_apps_nav(app_ids);
            rsx! {
                ChildRouter::<$route_ty> {
                    route,
                    format_route_as_root_route: |r: $route_ty| r.to_string(),
                    parse_route_from_root_route: |url: &str| {
                        <$route_ty as std::str::FromStr>::from_str(url).ok()
                    },
                }
            }
        }
    };
}

define_apps_route_wrapper!(AllApps, AllAppsRoute);
define_apps_route_wrapper!(General, GeneralRoute);
define_apps_route_wrapper!(IncentivePool, IncentivePoolRoute);

fn use_sync_apps_nav(app_names: Vec<SpaceAppName>) {
    let nav_override = try_use_context::<Signal<Option<Vec<SpaceAppName>>>>();

    use_effect(move || {
        if let Some(mut nav_override) = nav_override.clone() {
            if nav_override.read().as_ref() != Some(&app_names) {
                nav_override.set(Some(app_names.clone()));
            }
        }
    });

    use_drop(move || {
        if let Some(mut nav_override) = nav_override {
            nav_override.set(None);
        }
    });
}
