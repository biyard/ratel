use crate::*;
use dioxus::router::components::child_router::ChildRouter;

use space_app_file::Route as FileRoute;
use space_app_general::Route as GeneralRoute;
use space_app_incentive_pool::Route as IncentivePoolRoute;
use space_app_main::Route as MainRoute;

use layout::SpaceAppsLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps")]
        #[layout(SpaceAppsLayout)]
            #[route("/general/:..rest")]
            General { space_id: SpacePartition, rest: Vec<String> },
            #[route("/incentive_pool/:..rest")]
            IncentivePool { space_id: SpacePartition, rest: Vec<String> },
            #[route("/file/:..rest")]
            File { space_id: SpacePartition, rest: Vec<String> },
            #[route("/:..rest")]
            Main { space_id: SpacePartition, rest: Vec<String> },

}

macro_rules! define_apps_route_wrapper {
    ($wrapper_name:ident, $route_ty:ty) => {
        #[component]
        fn $wrapper_name(space_id: SpacePartition, rest: Vec<String>) -> Element {
            let router = use_context::<dioxus::router::RouterContext>();
            let route: $route_ty = router.current();
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

define_apps_route_wrapper!(Main, MainRoute);
define_apps_route_wrapper!(General, GeneralRoute);
define_apps_route_wrapper!(IncentivePool, IncentivePoolRoute);
define_apps_route_wrapper!(File, FileRoute);
