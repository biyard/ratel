use crate::*;
use actions::Route as ActionsRoute;
use apps::Route as AppsRoute;
use dashboard::Route as DashboardRoute;
use overview::Route as OverviewRoute;

/*
## https://github.com/ealmloff/dioxus/blob/master/packages/router/src/components/child_router.rs
For now, Child Router only support simple static prefixes
 */
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
                #[route("/apps/:..rest")]
                Apps { space_id: SpacePartition, rest: Vec<String> },
                
                #[redirect("/", |space_id: SpacePartition| Route::Dashboard { space_id, rest : vec![] })]
                #[redirect("/:..rest", |space_id: SpacePartition, rest: Vec<String>| Route::Dashboard { space_id, rest })]
            #[end_layout]
        #[redirect("/:..rest", |rest: Vec<String>| Route::PageNotFound{ route: rest })]
        #[end_nest]
    #[end_nest]
    #[route("/")]
    PageNotFound { route: Vec<String> },

}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}

/// Defines a wrapper component for a space sub-app.
///
/// - Arg[0]: Component name to register in the Route (e.g., `Dashboard`)
/// - Arg[1]: Actual route enum for the sub-app (e.g., `DashboardRoute`)
macro_rules! define_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(space_id: SpacePartition, rest: Vec<String>) -> Element {
            rsx! {
                Router::<$route_module> {}
            }
        }
    };
}

define_app_wrapper!(Dashboard, DashboardRoute);
define_app_wrapper!(Overview, OverviewRoute);
define_app_wrapper!(Actions, ActionsRoute);
define_app_wrapper!(Apps, AppsRoute);
