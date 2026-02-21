use crate::*;

use crate::views::Index;
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
            rsx! {
                Router::<$route_module> {}
            }
        }
    };
}

define_app_wrapper!(Auth, AuthRoute);
define_app_wrapper!(Space, SpaceRoute);
define_app_wrapper!(Post, PostRoute);

#[component]
pub fn UserHome(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! {
        Router::<UserRoute> {}
    }
}

#[component]
pub fn TeamHome(teamname: String, rest: Vec<String>) -> Element {
    let _ = (teamname, rest);
    rsx! {
        Router::<TeamRoute> {}
    }
}
