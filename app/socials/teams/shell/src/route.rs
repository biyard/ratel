use crate::*;

use views::Home;

use crate::layout::TeamLayout;
use dao::Home as DaoPage;
use draft::Home as DraftPage;
use group::Home as GroupPage;
use home::Home as HomePage;
use member::Home as MemberPage;
use crate::pages::reward::Home as RewardPage;
use setting::Home as SettingPage;

macro_rules! define_team_app_page {
    ($wrapper_name:ident, $page_component:ident) => {
        #[component]
        pub fn $wrapper_name(teamname: String) -> Element {
            rsx! {
                $page_component { teamname }
            }
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

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/teams/:teamname")]
        #[layout(TeamLayout)]
            #[route("/home")]
            TeamHome { teamname: String },
            #[route("/drafts")]
            TeamDraft { teamname: String },
            #[route("/groups")]
            TeamGroup { teamname: String },
            #[route("/dao")]
            TeamDao { teamname: String },
            #[route("/members")]
            TeamMember { teamname: String },
            #[route("/rewards")]
            TeamReward { teamname: String },
            #[route("/settings")]
            TeamSetting { teamname: String },
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
