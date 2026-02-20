use crate::*;

use crate::views::Index;
use layout::AppLayout;
use ratel_auth::Route as AuthRoute;
use ratel_post::Route as PostRoute;
use ratel_user_home::App as UserHomeApp;
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
    #[end_layout]

    #[route("/spaces/:..rest")]
    Space { rest: Vec<String> },

    #[route("/:username/:..rest")]
    UserHomeApp { username: String, rest: Vec<String> },

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
