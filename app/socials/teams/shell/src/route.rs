use crate::*;

use views::Home;

use crate::layout::TeamLayout;
use dioxus::router::components::child_router::ChildRouter;
use ratel_team_dao::Route as TeamDaoRoute;
use ratel_team_draft::Route as TeamDraftRoute;
use ratel_team_group::Route as TeamGroupRoute;
use ratel_team_home::Route as TeamHomeRoute;
use ratel_team_member::Route as TeamMemberRoute;
use ratel_team_reward::Route as TeamRewardRoute;
use ratel_team_setting::Route as TeamSettingRoute;

macro_rules! define_team_app_wrapper {
    ($wrapper_name:ident, $route_module:ident) => {
        #[component]
        pub fn $wrapper_name(teamname: String, rest: Vec<String>) -> Element {
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

define_team_app_wrapper!(TeamDao, TeamDaoRoute);
define_team_app_wrapper!(TeamDraft, TeamDraftRoute);
define_team_app_wrapper!(TeamGroup, TeamGroupRoute);
define_team_app_wrapper!(TeamHome, TeamHomeRoute);
define_team_app_wrapper!(TeamMember, TeamMemberRoute);
define_team_app_wrapper!(TeamReward, TeamRewardRoute);
define_team_app_wrapper!(TeamSetting, TeamSettingRoute);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/teams/:teamname")]
        #[layout(TeamLayout)]
            #[route("/home/:..rest")]
            TeamHome { teamname: String, rest: Vec<String> },
            #[route("/drafts/:..rest")]
            TeamDraft { teamname: String, rest: Vec<String> },
            #[route("/groups/:..rest")]
            TeamGroup { teamname: String, rest: Vec<String> },
            #[route("/dao/:..rest")]
            TeamDao { teamname: String, rest: Vec<String> },
            #[route("/members/:..rest")]
            TeamMember { teamname: String, rest: Vec<String> },
            #[route("/rewards/:..rest")]
            TeamReward { teamname: String, rest: Vec<String> },
            #[route("/settings/:..rest")]
            TeamSetting { teamname: String, rest: Vec<String> },
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
