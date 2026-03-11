use crate::*;

use crate::features::admin::Route as AdminRoute;
use crate::features::auth::Route as AuthRoute;
use crate::features::my_follower::Route as MyFollowerRoute;
use crate::features::posts::Route as PostRoute;

#[cfg(feature = "spaces")]
use crate::features::spaces::pages::dashboard::SpaceDashboardPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::overview::SpaceOverviewPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::report::SpaceReportPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::SpaceLayout;

// Space Apps
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::file::SpaceFileAppPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::general::SpaceGeneralAppPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::incentive_pool::SpaceIncentivePoolAppPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::Layout as SpaceAppsLayout;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::SpaceAppsPage;

// Space Actions
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::actions::discussion::{
    DiscussionActionEditorPage, DiscussionActionPage,
};
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::actions::poll::PollActionPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::actions::quiz::QuizActionPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::actions::subscription::FollowActionPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::SpaceActionsPage;

#[cfg(feature = "teams")]
use crate::features::teams::Route as TeamRoute;
#[cfg(feature = "users")]
use crate::features::users::Route as UserRoute;
use crate::views::Index;
use dioxus::router::components::child_router::ChildRouter;
use layout::AppLayout;
use membership::Home as MembershipHome;

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


    #[cfg_attr(feature="spaces", nest("/spaces/:space_id"))]
        #[cfg_attr(feature="spaces", layout(SpaceLayout))]
            #[cfg_attr(feature="spaces", route("/dashboard"))]
            #[cfg(feature = "spaces")]
            SpaceDashboardPage { space_id: SpacePartition },
            #[cfg_attr(feature="spaces", route("/overview"))]
            #[cfg(feature = "spaces")]
            SpaceOverviewPage { space_id: SpacePartition },

            #[cfg_attr(feature="spaces", nest("/actions"))]
                // #[cfg_attr(feature="spaces", layout(SpaceAppsLayout))]
                    #[cfg_attr(feature="spaces", route("/"))]
                    #[cfg(feature = "spaces")]
                    SpaceActionsPage { space_id: SpacePartition },

                    #[cfg_attr(feature="spaces", route("/discussions/:discussion_id/edit"))]
                    #[cfg(feature = "spaces")]
                    DiscussionActionEditorPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },

                    #[cfg_attr(feature="spaces", route("/discussions/:discussion_id"))]
                    #[cfg(feature = "spaces")]
                    DiscussionActionPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },

                    #[cfg_attr(feature="spaces", route("/polls/:poll_id"))]
                    #[cfg(feature = "spaces")]
                    PollActionPage { space_id: SpacePartition, poll_id: SpacePollEntityType },

                    #[cfg_attr(feature="spaces", route("/quizzes/:quiz_id"))]
                    #[cfg(feature = "spaces")]
                    QuizActionPage { space_id: SpacePartition, quiz_id: SpaceQuizEntityType },

                    #[cfg_attr(feature="spaces", route("/follows"))]
                    #[cfg(feature = "spaces")]
                    FollowActionPage { space_id: SpacePartition },

                // #[cfg_attr(feature="spaces", end_layout)]
            #[cfg_attr(feature="spaces", end_nest)]

    // #[nest("/spaces/:space_id/actions")]
    //     #[route("/polls/:..rest")]
    //     Poll { space_id: SpacePartition, rest: Vec<String> },
    //     #[route("/quizzes/:..rest")]
    //     Quiz { space_id: SpacePartition, rest: Vec<String> },
    //     #[route("/discussions/:..rest")]
    //     Discussion { space_id: SpacePartition, rest: Vec<String> },
    //     #[route("/subscriptions/:..rest")]
    //     Subscription { space_id: SpacePartition, rest: Vec<String> },
    //     #[route("/:..rest")]
    //     Main { space_id: SpacePartition, rest: Vec<String> },


            #[cfg_attr(feature="spaces", route("/report"))]
            #[cfg(feature = "spaces")]
            SpaceReportPage { space_id: SpacePartition },

            // Space Apps
            #[cfg_attr(feature="spaces", nest("/apps"))]
                #[cfg_attr(feature="spaces", layout(SpaceAppsLayout))]
                    #[cfg_attr(feature="spaces", route("/"))]
                    #[cfg(feature = "spaces")]
                    SpaceAppsPage { space_id: SpacePartition },

                    #[cfg_attr(feature="spaces", route("/general"))]
                    #[cfg(feature = "spaces")]
                    SpaceGeneralAppPage { space_id: SpacePartition },

                    #[cfg_attr(feature="spaces", route("/files"))]
                    #[cfg(feature = "spaces")]
                    SpaceFileAppPage { space_id: SpacePartition },

                    #[cfg_attr(feature="spaces", route("/panels"))]
                    #[cfg(feature = "spaces")]
                    SpacePanelsAppPage { space_id: SpacePartition },

                    #[cfg_attr(feature="spaces", route("/incentive-pool"))]
                    #[cfg(feature = "spaces")]
                    SpaceIncentivePoolAppPage { space_id: SpacePartition },
                #[cfg_attr(feature="spaces", end_layout)]
            #[cfg_attr(feature="spaces", end_nest)]

            #[cfg_attr(feature="spaces", redirect("/", |space_id: SpacePartition| Route::SpaceDashboardPage { space_id }))]
        #[cfg_attr(feature="spaces", end_layout)]
    #[cfg_attr(feature="spaces", end_nest)]

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {rest:?}" }
    }
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

#[cfg(feature = "spaces")]
#[component]
pub fn SpaceAppGeneralPage(space_id: SpacePartition) -> Element {
    rsx! {
        crate::features::spaces::pages::apps::apps::general::SpaceGeneralAppPage { space_id }
    }
}

#[cfg(feature = "spaces")]
#[component]
pub fn SpaceAppFilePage(space_id: SpacePartition) -> Element {
    rsx! {
        crate::features::spaces::pages::apps::apps::file::SpaceFileAppPage { space_id }
    }
}

#[cfg(feature = "spaces")]
#[component]
pub fn SpacePanelsAppPage(space_id: SpacePartition) -> Element {
    rsx! {
        crate::features::spaces::pages::apps::apps::panels::SpacePanelsAppPage { space_id }
    }
}

#[cfg(feature = "spaces")]
#[component]
pub fn SpaceAppIncentivePoolPage(space_id: SpacePartition) -> Element {
    rsx! {
        crate::features::spaces::pages::apps::apps::incentive_pool::SpaceIncentivePoolAppPage { space_id }
    }
}
