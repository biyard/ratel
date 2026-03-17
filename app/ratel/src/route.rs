use crate::*;

use crate::features::my_follower::MyFollowerPage;

#[cfg(feature = "spaces")]
use crate::features::spaces::pages::dashboard::SpaceDashboardPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::overview::SpaceOverviewPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::report::SpaceReportPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::SpaceLayout;

// Space Rewards
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::rewards::views::HomePage as SpaceRewardsHomePage;

// Space Apps
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::file::SpaceFileAppPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::general::SpaceGeneralAppPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::incentive_pool::SpaceIncentivePoolAppPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::apps::apps::panels::SpacePanelsAppPage;
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
use crate::features::spaces::pages::actions::actions::follow::FollowActionPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::actions::poll::PollActionPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::actions::quiz::QuizActionPage;
#[cfg(feature = "spaces")]
use crate::features::spaces::pages::actions::SpaceActionsPage;

use crate::features::admin::{AdminLayout, AdminMainPage};

use crate::features::posts::{Index as PostIndex, PostDetail, PostEdit};

use crate::views::Index;
use layout::AppLayout;
use membership::Home as MembershipHome;
use root_layout::RootLayout;

// Team pages
#[cfg(feature = "social")]
use crate::features::social::layout::SocialLayout;
#[cfg(feature = "social")]
use crate::features::social::pages::dao::Home as TeamDao;
#[cfg(feature = "social")]
use crate::features::social::pages::draft::Home as TeamDraft;
#[cfg(feature = "social")]
use crate::features::social::pages::group::Home as TeamGroup;
#[cfg(feature = "social")]
use crate::features::social::pages::home::Home as TeamHome;
#[cfg(feature = "social")]
use crate::features::social::pages::member::Home as TeamMember;
#[cfg(feature = "social")]
use crate::features::social::pages::reward::Home as TeamReward;
#[cfg(feature = "social")]
use crate::features::social::pages::setting::layout::TeamSettingLayout;
#[cfg(feature = "social")]
use crate::features::social::pages::setting::Home as TeamSetting;
#[cfg(feature = "social")]
use crate::features::social::pages::setting::ManagementPage as TeamSettingMember;

// User pages
#[cfg(feature = "social")]
use crate::features::social::pages::credentials::Home as CredentialPage;
#[cfg(feature = "social")]
use crate::features::social::pages::post::Home as UserPosts;
#[cfg(feature = "social")]
use crate::features::social::pages::space::Home as UserSpaces;
#[cfg(feature = "social")]
use crate::features::social::pages::user_draft::Home as UserDrafts;
#[cfg(feature = "social")]
use crate::features::social::pages::user_membership::Home as UserMemberships;
#[cfg(feature = "social")]
use crate::features::social::pages::user_reward::Home as UserRewards;
#[cfg(feature = "social")]
use crate::features::social::pages::user_setting::Home as UserSettingPage;
#[cfg(feature = "social")]
use crate::features::social::user_views::Home as UserHomeRoot;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[layout(AppLayout)]
            #[route("/")]
            Index { },

            #[cfg_attr(feature="membership", route("/membership"))]
            #[cfg(feature="membership")]
            MembershipHome {  },

            #[nest("/posts")]
                #[route("/")]
                PostIndex { },
                #[route("/:post_id/edit")]
                PostEdit { post_id: FeedPartition },
                #[route("/:post_id")]
                PostDetail { post_id: FeedPartition },
            #[end_nest]

            #[route("/my-follower")]
            MyFollowerPage { },

            #[layout(AdminLayout)]
                #[route("/")]
                AdminMainPage {},
            #[end_layout]

            #[cfg(feature = "social")]
            #[nest("/:username")]
                #[layout(SocialLayout)]
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
        #[end_layout]

        #[cfg_attr(feature="spaces", nest("/spaces/:space_id"))]
            #[cfg_attr(feature="spaces", layout(SpaceLayout))]
                #[cfg_attr(feature="spaces", route("/dashboard"))]
                #[cfg(feature = "spaces")]
                SpaceDashboardPage { space_id: SpacePartition },
                #[cfg_attr(feature="spaces", route("/overview"))]
                #[cfg(feature = "spaces")]
                SpaceOverviewPage { space_id: SpacePartition },
                #[cfg_attr(feature="spaces", route("/report"))]
                #[cfg(feature = "spaces")]
                SpaceReportPage { space_id: SpacePartition },

                #[cfg_attr(feature="spaces", nest("/actions"))]
                    #[cfg_attr(feature="spaces", route("/"))]
                    #[cfg(feature = "spaces")]
                    SpaceActionsPage { space_id: SpacePartition },

                    #[cfg_attr(feature="spaces", route("/discussions/:discussion_id"))]
                    #[cfg(feature = "spaces")]
                    DiscussionActionPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },

                    #[cfg_attr(feature="spaces", route("/discussions/:discussion_id/edit"))]
                    #[cfg(feature = "spaces")]
                    DiscussionActionEditorPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },
                    #[cfg_attr(feature="spaces", route("/polls/:poll_id"))]
                    #[cfg(feature = "spaces")]
                    PollActionPage { space_id: SpacePartition, poll_id: SpacePollEntityType },

                    #[cfg_attr(feature="spaces", route("/quizzes/:quiz_id"))]
                    #[cfg(feature = "spaces")]
                    QuizActionPage { space_id: SpacePartition, quiz_id: SpaceQuizEntityType },

                    #[cfg_attr(feature="spaces", route("/follows/:follow_id"))]
                    #[cfg(feature = "spaces")]
                    FollowActionPage { space_id: SpacePartition, follow_id: SpaceActionFollowEntityType },
                #[cfg_attr(feature="spaces", end_nest)]

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
