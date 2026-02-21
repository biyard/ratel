use crate::*;

use crate::views::Index;
use dioxus::router::components::child_router::ChildRouter;
use layout::AppLayout;
use ratel_auth::Route as AuthRoute;
use ratel_post::Route as PostRoute;
use ratel_team_shell::Route as TeamRoute;
use ratel_user_shell::Route as UserRoute;
use space_shell::Route as SpaceRoute;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Index { },

        #[route("/auth/:..rest")]
        Auth { rest: Vec<String> },

        #[route("/posts/:..rest")]
        Post { rest: Vec<String> },

        #[route("/:username/:..rest")]
        UserHome { username: String, rest: Vec<String> },

        #[route("/teams/:teamname/:..rest")]
        TeamHome { teamname: String, rest: Vec<String> },
    #[end_layout]

    #[route("/spaces/:..rest")]
    Space { rest: Vec<String> },
}

macro_rules! define_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(rest: Vec<String>) -> Element {
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

define_app_wrapper!(Auth, AuthRoute);
define_app_wrapper!(Space, SpaceRoute);
define_app_wrapper!(Post, PostRoute);

#[component]
pub fn UserHome(username: String, rest: Vec<String>) -> Element {
    let router = use_context::<dioxus::router::RouterContext>();
    let route: UserRoute = router.current();

    rsx! {
        ChildRouter::<UserRoute> {
            route,
            format_route_as_root_route: |r: UserRoute| r.to_string(),
            parse_route_from_root_route: |url: &str| { <UserRoute as std::str::FromStr>::from_str(url).ok() },

        }
    }
}

#[component]
pub fn TeamHome(teamname: String, rest: Vec<String>) -> Element {
    let router = use_context::<dioxus::router::RouterContext>();
    let route: TeamRoute = router.current();

    rsx! {
        ChildRouter::<TeamRoute> {
            route,
            format_route_as_root_route: |r: TeamRoute| r.to_string(),
            parse_route_from_root_route: |url: &str| { <TeamRoute as std::str::FromStr>::from_str(url).ok() },

        }
    }
}
