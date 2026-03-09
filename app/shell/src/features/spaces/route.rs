use crate::features::spaces::*;
use crate::features::spaces::actions::shell::Route as ActionsRoute;
use crate::features::spaces::apps::shell::Route as AppsRoute;
use crate::features::spaces::pages::dashboard::Route as DashboardRoute;
use dioxus::router::components::child_router::ChildRouter;
use crate::features::spaces::pages::overview::Route as OverviewRoute;
use crate::features::spaces::pages::report::Route as ReportRoute;
#[cfg(not(feature = "layout_test"))]
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces")]
        #[nest("/:space_id")]
                #[layout(SpaceLayout)]
                    #[route("/dashboard/:..rest")]
                    Dashboard { space_id: SpacePartition, rest: Vec<String> },
                    #[route("/overview/:..rest")]
                    Overview { space_id: SpacePartition, rest: Vec<String> },
                    #[route("/actions/:..rest")]
                    Actions { space_id: SpacePartition, rest: Vec<String> },
                    #[route("/report/:..rest")]
                    Report { space_id: SpacePartition, rest: Vec<String> },
                    #[route("/apps/:..rest")]
                    Apps { space_id: SpacePartition, rest: Vec<String> },
                    #[redirect("/", |space_id: SpacePartition| Route::Dashboard { space_id, rest : vec![] })]
                #[end_layout]
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[route("/")]
    PageNotFound { route: Vec<String> },

}

#[cfg(feature = "layout_test")]
use crate::features::spaces::views::LoginTest;
#[cfg(feature = "layout_test")]
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    LoginTest {},
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}

/// Defines a wrapper component for a space sub-app using ChildRouter.
///
/// Uses `ChildRouter` instead of `Router` to share the parent's `RouterContext`,
/// avoiding nested router conflicts. Each sub-package's Route enum already contains
/// full paths (e.g., `/spaces/:space_id/actions/...`), so no prefix stripping is needed.
///
/// - Arg[0]: Component name to register in the Route (e.g., `Dashboard`)
/// - Arg[1]: Actual route enum for the sub-app (e.g., `DashboardRoute`)
macro_rules! define_space_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(space_id: SpacePartition, rest: Vec<String>) -> Element {
            let router = use_context::<dioxus::router::RouterContext>();
            let route: $route_module = router.current();
            rsx! {
                ChildRouter::<$route_module> {
                    route,
                    format_route_as_root_route: |r: $route_module| r.to_string(),
                    parse_route_from_root_route: |url: &str| {
                        <$route_module as std::str::FromStr>::from_str(url).ok()
                    },
                }
            }
        }
    };
}

// #[component]
// fn ChildApp(space_id: SpacePartition, rest: Vec<String>) -> Element {
//     let router = consume_context::<RouterContext>();
//     let route: ChildRoute = router.current();
//     rsx! {
//         ChildRouter::<ChildRoute> {
//             route,
//             format_route_as_root_route: |r: ChildRoute| r.to_string(),
//             parse_route_from_root_route: |url: &str| { <ChildRoute as std::str::FromStr>::from_str(url).ok() },
//         }
//     }
// }

define_space_app_wrapper!(Dashboard, DashboardRoute);
define_space_app_wrapper!(Overview, OverviewRoute);
define_space_app_wrapper!(Actions, ActionsRoute);
define_space_app_wrapper!(Apps, AppsRoute);
define_space_app_wrapper!(Report, ReportRoute);
