use crate::layout::UserLayout;
use crate::*;
use views::Home;

use ratel_user_credential::Route as CredentialRoute;
use ratel_user_draft::Route as DraftRoute;
use ratel_user_membership::Route as MembershipRoute;
use ratel_user_post::Route as PostRoute;
use ratel_user_reward::Route as RewardRoute;
use ratel_user_setting::Route as SettingRoute;

macro_rules! define_user_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(username: String, rest: Vec<String>) -> Element {
            let _ = (username, rest);
            rsx! {
                Router::<$route_module> {}
            }
        }
    };
}

define_user_app_wrapper!(UserPosts, PostRoute);
define_user_app_wrapper!(UserRewards, RewardRoute);
define_user_app_wrapper!(UserSettings, SettingRoute);
define_user_app_wrapper!(UserMemberships, MembershipRoute);
define_user_app_wrapper!(UserDrafts, DraftRoute);
define_user_app_wrapper!(UserCredentials, CredentialRoute);

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
