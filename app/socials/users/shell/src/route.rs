use crate::layout::UserLayout;
use crate::*;
use dioxus::router::components::child_router::ChildRouter;
use views::Home;

use ratel_user_credential::Route as CredentialRoute;
use ratel_user_draft::Route as DraftRoute;
use ratel_user_membership::Route as MembershipRoute;
use ratel_user_post::Route as PostRoute;
use ratel_user_reward::Route as RewardRoute;
use ratel_user_setting::Route as SettingRoute;
use ratel_user_space::Route as SpaceRoute;

macro_rules! define_user_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(username: String, rest: Vec<String>) -> Element {
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

macro_rules! define_owner_only_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(username: String, rest: Vec<String>) -> Element {
            let user_ctx = ratel_auth::hooks::use_user_context();
            let is_owner = user_ctx()
                .user
                .as_ref()
                .map(|u| u.username == username)
                .unwrap_or(false);

            if !is_owner {
                let nav = navigator();
                nav.push(format!("/{username}/posts"));
                return rsx! {};
            }

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

define_user_app_wrapper!(UserPosts, PostRoute);
define_owner_only_wrapper!(UserRewards, RewardRoute);
define_owner_only_wrapper!(UserSettings, SettingRoute);
define_owner_only_wrapper!(UserMemberships, MembershipRoute);
define_owner_only_wrapper!(UserDrafts, DraftRoute);
define_owner_only_wrapper!(UserCredentials, CredentialRoute);
define_owner_only_wrapper!(UserSpaces, SpaceRoute);

#[component]
fn UserHomeRoot(username: String) -> Element {
    let _ = username;
    rsx! {
        Home {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:username")]
        #[layout(UserLayout)]
            #[route("/")]
            UserHomeRoot { username: String },
            #[route("/posts/:..rest")]
            UserPosts { username: String, rest: Vec<String> },
            #[route("/rewards/:..rest")]
            UserRewards { username: String, rest: Vec<String> },
            #[route("/settings/:..rest")]
            UserSettings { username: String, rest: Vec<String> },
            #[route("/memberships/:..rest")]
            UserMemberships { username: String, rest: Vec<String> },
            #[route("/drafts/:..rest")]
            UserDrafts { username: String, rest: Vec<String> },
            #[route("/credentials/:..rest")]
            UserCredentials { username: String, rest: Vec<String> },
            #[route("/spaces/:..rest")]
            UserSpaces { username: String, rest: Vec<String> },
        #[end_layout]
    #[end_nest]

    #[route("/:..route")]
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
