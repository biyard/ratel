use crate::features::social::*;

use crate::features::social::layout::{SocialLayout, UserLayout};
use crate::features::social::pages::setting::layout::TeamSettingLayout;

// Team pages
use crate::features::social::pages::dao::Home as DaoPage;
use crate::features::social::pages::draft::Home as DraftPage;
use crate::features::social::pages::group::Home as GroupPage;
use crate::features::social::pages::home::Home as HomePage;
use crate::features::social::pages::member::Home as MemberPage;
use crate::features::social::pages::reward::Home as RewardPage;
use crate::features::social::pages::setting::Home as SettingPage;
use crate::features::social::pages::setting::ManagementPage as SettingManagementPage;

// User pages
use crate::features::social::user_views::Home as UserViewHome;
use crate::features::social::pages::credentials::Home as CredentialPage;
use crate::features::social::pages::user_draft::Home as UserDraftPage;
use crate::features::social::pages::user_membership::Home as MembershipPage;
use crate::features::social::pages::post::Home as PostPage;
use crate::features::social::pages::user_reward::Home as UserRewardPage;
use crate::features::social::pages::user_setting::Home as UserSettingPage;
use crate::features::social::pages::space::Home as SpacePage;

macro_rules! define_team_app_page {
    ($wrapper_name:ident, $page_component:ident) => {
        #[component]
        pub fn $wrapper_name(username: String) -> Element {
            rsx! {
                $page_component { username }
            }
        }
    };
}

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
            let user_ctx = crate::features::auth::hooks::use_user_context();
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

            let el: Element = rsx! {
                $page_component { username }
            };
            el
        }
    };
}

define_team_app_page!(TeamDao, DaoPage);
define_team_app_page!(TeamDraft, DraftPage);
define_team_app_page!(TeamGroup, GroupPage);
define_team_app_page!(TeamHome, HomePage);
define_team_app_page!(TeamMember, MemberPage);
define_team_app_page!(TeamReward, RewardPage);
define_team_app_page!(TeamSetting, SettingPage);

#[component]
pub fn TeamSettingMember(username: String) -> Element {
    rsx! {
        SettingManagementPage { username }
    }
}

// User page wrappers
define_user_app_page!(UserPosts, PostPage);
define_owner_only_page!(UserRewards, UserRewardPage);
define_owner_only_page!(UserSettings, UserSettingPage);
define_owner_only_page!(UserMemberships, MembershipPage);
define_owner_only_page!(UserDrafts, UserDraftPage);
define_owner_only_page!(UserSpaces, SpacePage);

#[component]
pub fn UserHomeRoot(username: String) -> Element {
    rsx! {
        UserViewHome { username }
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
            #[route("/memberships")]
            UserMemberships { username: String },
            #[route("/drafts")]
            UserDrafts { username: String },
            #[route("/credentials")]
            CredentialPage { username: String },
            #[route("/spaces")]
            UserSpaces { username: String },
        #[end_layout]
        #[layout(SocialLayout)]
            #[route("/home")]
            TeamHome { username: String },
            #[route("/team-drafts")]
            TeamDraft { username: String },
            #[route("/groups")]
            TeamGroup { username: String },
            #[route("/dao")]
            TeamDao { username: String },
            #[route("/members")]
            TeamMember { username: String },
            #[route("/team-rewards")]
            TeamReward { username: String },
        #[end_layout]
        #[layout(TeamSettingLayout)]
            #[route("/settings")]
            TeamSetting { username: String },
            #[route("/settings/members")]
            TeamSettingMember { username: String },
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
