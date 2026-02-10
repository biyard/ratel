use crate::*;
use actions::Route as ActionsRoute;
use apps::Route as AppsRoute;
use dashboard::Route as DashboardRoute;
use overview::Route as OverviewRoute;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces")]
        #[nest("/:space_id")]
            #[layout(SpaceLayout)]
                #[route("/dashboard/:..rest")]
                DashboardApp { space_id: SpacePartition, rest: Vec<String> },
                #[route("/overview/:..rest")]
                OverviewApp { space_id: SpacePartition, rest: Vec<String> },
                #[route("/actions/:..rest")]
                ActionsApp { space_id: SpacePartition, rest: Vec<String> },
                #[route("/apps/:..rest")]
                AppsApp { space_id: SpacePartition, rest: Vec<String> },
                
                #[redirect("/", |space_id: SpacePartition| Route::DashboardApp { space_id, rest : vec![] })]
                #[redirect("/:..rest", |space_id: SpacePartition, rest: Vec<String>| Route::DashboardApp { space_id, rest })]
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

define_app_wrapper!(DashboardApp, DashboardRoute);
define_app_wrapper!(OverviewApp, OverviewRoute);
define_app_wrapper!(ActionsApp, ActionsRoute);
define_app_wrapper!(AppsApp, AppsRoute);
