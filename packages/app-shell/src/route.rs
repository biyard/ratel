use crate::*;

use crate::views::Index;
use layout::AppLayout;
use ratel_auth::Route as AuthRoute;
use ratel_post::Route as PostRoute;
use ratel_user_credential::Route as UserCredentialRoute;
use ratel_user_draft::Route as UserDraftRoute;
use ratel_user_membership::Route as UserMembershipRoute;
use ratel_user_post::Route as UserPostRoute;
use ratel_user_reward::Route as UserRewardRoute;
use ratel_user_setting::Route as UserSettingRoute;
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

    #[route("/:username/posts/:..rest")]
    UserPosts { username: String, rest: Vec<String> },
    #[route("/:username/rewards/:..rest")]
    UserRewards { username: String, rest: Vec<String> },
    #[route("/:username/settings/:..rest")]
    UserSettings { username: String, rest: Vec<String> },
    #[route("/:username/memberships/:..rest")]
    UserMemberships { username: String, rest: Vec<String> },
    #[route("/:username/drafts/:..rest")]
    UserDrafts { username: String, rest: Vec<String> },
    #[route("/:username/credentials/:..rest")]
    UserCredentials { username: String, rest: Vec<String> },

    #[redirect("/:username", |username: String| Route::UserPosts { username, rest: vec![] })]

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
pub fn UserPosts(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! { Router::<UserPostRoute> {} }
}

#[component]
pub fn UserRewards(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! { Router::<UserRewardRoute> {} }
}

#[component]
pub fn UserSettings(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! { Router::<UserSettingRoute> {} }
}

#[component]
pub fn UserMemberships(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! { Router::<UserMembershipRoute> {} }
}

#[component]
pub fn UserDrafts(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! { Router::<UserDraftRoute> {} }
}

#[component]
pub fn UserCredentials(username: String, rest: Vec<String>) -> Element {
    let _ = (username, rest);
    rsx! { Router::<UserCredentialRoute> {} }
}
