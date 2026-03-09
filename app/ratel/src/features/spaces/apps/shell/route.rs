use crate::features::spaces::apps::shell::*;
use dioxus::router::components::child_router::ChildRouter;

use crate::features::spaces::apps::file::Route as FileRoute;
use crate::features::spaces::apps::general::Route as GeneralRoute;
use crate::features::spaces::apps::incentive_pool::Route as IncentivePoolRoute;
use crate::features::spaces::apps::main::Route as MainRoute;
use crate::features::spaces::apps::rewards::Route as RewardsRoute;

use layout::SpaceAppsLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps")]
        #[layout(SpaceAppsLayout)]
            #[route("/general/:..rest")]
            General { space_id: SpacePartition, rest: Vec<String> },
            #[route("/incentive-pool/:..rest")]
            IncentivePool { space_id: SpacePartition, rest: Vec<String> },
            #[route("/file/:..rest")]
            File { space_id: SpacePartition, rest: Vec<String> },
            #[route("/rewards/:..rest")]
            Rewards { space_id: SpacePartition, rest: Vec<String> },
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
define_apps_route_wrapper!(Rewards, RewardsRoute);
