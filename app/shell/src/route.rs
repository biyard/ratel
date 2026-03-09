use crate::*;

#[cfg(feature = "teams")]
use crate::features::teams::Route as TeamRoute;
#[cfg(feature = "users")]
use crate::features::users::Route as UserRoute;
use crate::views::Index;
use dioxus::router::components::child_router::ChildRouter;
use layout::AppLayout;
use membership::Home as MembershipHome;
use crate::features::admin::Route as AdminRoute;
use crate::features::auth::Route as AuthRoute;
use crate::features::my_follower::Route as MyFollowerRoute;
use crate::features::posts::Route as PostRoute;
use crate::features::spaces::Route as SpaceRoute;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Index { },

        #[route("/auth/:..rest")]
        Auth { rest: Vec<String> },

        #[cfg_attr(feature="membership", route("/membership"))]
        #[cfg(feature="membership")]
        MembershipHome {  },

        #[route("/posts/:..rest")]
        Post { rest: Vec<String> },

        #[route("/my-follower/:..rest")]
        MyFollower { rest: Vec<String> },

        #[route("/admin/:..rest")]
        Admin { rest: Vec<String> },

        #[cfg(feature = "users")]
        #[route("/:username/:..rest")]
        #[cfg(feature = "users")]
        UserHome { username: String, rest: Vec<String> },

        #[cfg(feature = "teams")]
        #[route("/teams/:teamname/:..rest")]
        #[cfg(feature = "teams")]
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

define_app_wrapper!(Admin, AdminRoute);
define_app_wrapper!(Auth, AuthRoute);
define_app_wrapper!(Space, SpaceRoute);
define_app_wrapper!(Post, PostRoute);
define_app_wrapper!(MyFollower, MyFollowerRoute);

#[cfg(feature = "users")]
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

#[cfg(feature = "teams")]
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
