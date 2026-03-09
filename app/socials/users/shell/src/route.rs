use crate::layout::UserLayout;
use crate::*;
use views::Home;

use credentials::Home as CredentialPage;
use draft::Home as DraftPage;
use membership::Home as MembershipPage;
use post::Home as PostPage;
use reward::Home as RewardPage;
use setting::Home as SettingPage;
use space::Home as SpacePage;

macro_rules! define_user_app_page {
    ($wrapper_name:ident, $page_component:ident) => {
        #[component]
        pub fn $wrapper_name(username: String) -> Element {
            rsx! {
                $page_component { username }
            }
        }
    };
}

macro_rules! define_owner_only_page {
    ($wrapper_name:ident, $page_component:ident) => {
        #[component]
        pub fn $wrapper_name(username: String) -> Element {
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

            rsx! {
                $page_component { username }
            }
        }
    };
}

define_user_app_page!(UserPosts, PostPage);
define_owner_only_page!(UserRewards, RewardPage);
define_owner_only_page!(UserSettings, SettingPage);
define_owner_only_page!(UserMemberships, MembershipPage);
define_owner_only_page!(UserDrafts, DraftPage);
define_owner_only_page!(UserSpaces, SpacePage);

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
            #[route("/posts")]
            UserPosts { username: String },
            #[route("/rewards")]
            UserRewards { username: String },
            #[route("/settings")]
            UserSettings { username: String },
            #[route("/memberships")]
            UserMemberships { username: String },
            #[route("/drafts")]
            UserDrafts { username: String },
            #[route("/credentials")]
            CredentialPage { username: String },
            #[route("/spaces")]
            UserSpaces { username: String },
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
