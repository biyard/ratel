use crate::*;

use crate::features::essence::EssenceSourcesPage;
use crate::features::me::pages::MyAiPage;
use crate::features::my_follower::MyFollowerPage;

use crate::features::spaces::pages::dashboard::SpaceDashboardPage;
use crate::features::spaces::pages::overview::SpaceOverviewPage;
use crate::features::spaces::pages::report::SpaceReportPage;
use crate::features::spaces::SpaceLayout;

// Space Rewards
use crate::features::spaces::pages::apps::apps::rewards::views::HomePage as SpaceRewardsHomePage;

// Space Apps
use crate::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeCreatePage;
use crate::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeDetailPage;
use crate::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeDiscussionPage;
use crate::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeRecordsPage;
use crate::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeReportPage;
use crate::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzesAppPage;
use crate::features::spaces::pages::apps::apps::file::SpaceFileAppPage;
use crate::features::spaces::pages::apps::apps::general::SpaceGeneralAppPage;
use crate::features::spaces::pages::apps::apps::incentive_pool::SpaceIncentivePoolAppPage;
use crate::features::spaces::pages::apps::apps::panels::SpacePanelsAppPage;
use crate::features::spaces::pages::apps::Layout as SpaceAppsLayout;
use crate::features::spaces::pages::apps::SpaceAppsPage;
use crate::features::spaces::pages::index::action_pages::{
    SpaceDiscussionCommentPage, SpaceDiscussionPage,
};
use crate::features::spaces::pages::index::SpaceIndexPage;

// Space Actions
use crate::features::spaces::pages::actions::actions::discussion::DiscussionActionEditorPage;
use crate::features::spaces::pages::actions::actions::follow::FollowActionPage;
use crate::features::spaces::pages::actions::actions::meet::MeetActionPage;
use crate::features::spaces::pages::actions::actions::poll::PollActionPage;
use crate::features::spaces::pages::actions::actions::quiz::QuizActionPage;
use crate::features::spaces::pages::actions::SpaceActionsPage;

use crate::features::admin::{AdminLayout, AdminMainPage};

// Sub-team governance pages — placeholders wired up now; content lives
// under `features::sub_team::pages::*` and will be fleshed out by the
// UI-implementation dispatches.
use crate::features::sub_team::pages::{
    TeamBylawsPage, TeamLeaveParentPage, TeamSubTeamApplicationStatusPage,
    TeamSubTeamApplyPage, TeamSubTeamBroadcastComposePage, TeamSubTeamBroadcastEditPage,
    TeamSubTeamDeregisterPage, TeamSubTeamDetailPage, TeamSubTeamDocComposePage,
    TeamSubTeamDocEditPage, TeamSubTeamManagementPage,
};

use crate::features::posts::{Index as PostIndex, PostDetail, PostEdit};

use crate::views::{Index, PrivacyPolicyPage, TermsOfServicePage};
use layout::AppLayout;
use membership::Home as MembershipHome;
use root_layout::RootLayout;

/// Top-level `/credentials` entry point. The underlying `CredentialPage`
/// component shows the *current user's* credentials and ignores its
/// `username` prop, so we wrap it here with an empty placeholder to
/// expose a username-less route.
#[component]
fn CredentialsHome() -> Element {
    rsx! {
        CredentialPage { username: String::new() }
    }
}

// Team pages
use crate::features::social::layout::SocialLayout;
use crate::features::social::pages::dao::Home as TeamDao;
use crate::features::social::pages::draft::Home as TeamDraft;
use crate::features::social::pages::home::Home as TeamHome;
use crate::features::social::pages::member::Home as TeamMember;
use crate::features::social::pages::reward::Home as TeamReward;
use crate::features::social::pages::setting::Home as TeamSetting;
use crate::features::social::pages::setting::ManagementPage as TeamSettingMember;
use crate::features::social::pages::setting::SubscriptionPage as TeamSettingSubscription;
use crate::features::social::pages::team_arena::TeamArenaLayout;

// User pages
use crate::features::social::pages::credentials::Home as CredentialPage;
use crate::features::social::pages::post::Home as UserPosts;
use crate::features::social::pages::space::Home as UserSpaces;
use crate::features::social::pages::team_membership::Home as TeamMemberships;
use crate::features::social::pages::user_draft::Home as UserDrafts;
use crate::features::social::pages::user_membership::Home as UserMemberships;
use crate::features::social::pages::user_reward::Home as UserRewards;
use crate::features::social::pages::user_setting::Home as UserSettingPage;
use crate::features::social::user_views::Home as UserHomeRoot;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        Index { },

        #[route("/privacy")]
        PrivacyPolicyPage { },

        #[route("/terms")]
        TermsOfServicePage { },

        #[route("/membership")]
        MembershipHome {  },

        #[route("/credentials")]
        CredentialsHome {  },

        #[route("/essence")]
        EssenceSourcesPage {},

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

        #[route("/my-ai")]
        MyAiPage { },

        #[nest("/admin")]
            #[layout(AdminLayout)]
                #[route("/")]
                AdminMainPage {},
            #[end_layout]
        #[end_nest]

        #[nest("/:username")]
            #[route("/rewards")]
            UserRewards { username: String },
            #[route("/settings")]
            UserSettingPage { username: String },
            #[route("/drafts")]
            UserDrafts { username: String },
            #[layout(SocialLayout)]
                #[route("/")]
                UserHomeRoot { username: String },
                #[route("/posts")]
                UserPosts { username: String },
                #[route("/memberships")]
                UserMemberships { username: String },
                #[route("/credentials")]
                CredentialPage { username: String },
                #[route("/spaces")]
                UserSpaces { username: String },
            #[end_layout]
            #[layout(TeamArenaLayout)]
                #[route("/home")]
                TeamHome { username: String },
                #[route("/team-drafts")]
                TeamDraft { username: String },
                #[route("/dao")]
                TeamDao { username: String },
                #[route("/members")]
                TeamMember { username: String },
                #[route("/team-rewards")]
                TeamReward { username: String },
                #[route("/team-memberships")]
                TeamMemberships { username: String },
                #[route("/team-settings")]
                TeamSetting { username: String },
                #[route("/team-settings/members")]
                TeamSettingMember { username: String },
                #[route("/team-settings/subscription")]
                TeamSettingSubscription { username: String },

                // Sub-team governance — path param inherited from the
                // parent `/:username` nest, so the field is `username`
                // (the team's handle, which identifies the team). The
                // placeholder components map it to `team_id` internally.
                // Follow-up dispatches wire in the design mockups and
                // controller hooks.
                #[route("/sub-teams/manage")]
                TeamSubTeamManagementPage { username: String },
                #[route("/sub-teams/apply")]
                TeamSubTeamApplyPage { username: String },
                #[route("/sub-teams/application")]
                TeamSubTeamApplicationStatusPage { username: String },
                #[route("/sub-teams/docs/compose")]
                TeamSubTeamDocComposePage { username: String },
                #[route("/sub-teams/docs/:doc_id/edit")]
                TeamSubTeamDocEditPage { username: String, doc_id: String },
                #[route("/sub-teams/announcements/compose")]
                TeamSubTeamBroadcastComposePage { username: String },
                #[route("/sub-teams/announcements/:announcement_id/edit")]
                TeamSubTeamBroadcastEditPage { username: String, announcement_id: String },
                #[route("/sub-teams/:sub_team_id")]
                TeamSubTeamDetailPage { username: String, sub_team_id: String },
                #[route("/sub-teams/:sub_team_id/deregister")]
                TeamSubTeamDeregisterPage { username: String, sub_team_id: String },
                #[route("/parent/leave")]
                TeamLeaveParentPage { username: String },
                #[route("/bylaws")]
                TeamBylawsPage { username: String },
            #[end_layout]
        #[end_nest]

        #[nest("/spaces/:space_id")]
            #[layout(SpaceLayout)]
                #[route("/")]
                SpaceIndexPage { space_id: SpacePartition },
                #[route("/dashboard")]
                SpaceDashboardPage { space_id: SpacePartition },
                #[route("/overview")]
                SpaceOverviewPage { space_id: SpacePartition },
                #[route("/report")]
                SpaceReportPage { space_id: SpacePartition },

                // Deep link to a discussion (standalone page, no arena overlay).
                #[route("/discussions/:discussion_id")]
                SpaceDiscussionPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },

                // Deep link with a specific comment to scroll to + highlight.
                // The comment id lives in the path (not query/fragment) because
                // Dioxus Router strips both query strings and fragments during
                // URL normalization on hydration.
                #[route("/discussions/:discussion_id/comments/:comment_id")]
                SpaceDiscussionCommentPage { space_id: SpacePartition, discussion_id: SpacePostEntityType, comment_id: String },

                #[nest("/actions")]
                    #[route("/")]
                    SpaceActionsPage { space_id: SpacePartition },

                    #[route("/discussions/:discussion_id/edit")]
                    DiscussionActionEditorPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },
                    #[route("/polls/:poll_id")]
                    PollActionPage { space_id: SpacePartition, poll_id: SpacePollEntityType },

                    #[route("/quizzes/:quiz_id")]
                    QuizActionPage { space_id: SpacePartition, quiz_id: SpaceQuizEntityType },

                    #[route("/follows/:follow_id")]
                    FollowActionPage { space_id: SpacePartition, follow_id: SpaceActionFollowEntityType },

                    #[route("/meets/:meet_id")]
                    MeetActionPage { space_id: SpacePartition, meet_id: SpaceMeetEntityType },
                #[end_nest]

                // Space Apps
                #[nest("/apps")]
                    #[layout(SpaceAppsLayout)]
                        #[route("/")]
                        SpaceAppsPage { space_id: SpacePartition },

                        #[route("/general")]
                        SpaceGeneralAppPage { space_id: SpacePartition },

                        #[route("/files")]
                        SpaceFileAppPage { space_id: SpacePartition },

                        #[route("/analyzes")]
                        SpaceAnalyzesAppPage { space_id: SpacePartition },

                        #[route("/analyzes/create")]
                        SpaceAnalyzeCreatePage { space_id: SpacePartition },

                        #[route("/analyzes/report/:report_id")]
                        SpaceAnalyzeReportPage { space_id: SpacePartition, report_id: String },

                        #[route("/analyzes/report/:report_id/records")]
                        SpaceAnalyzeRecordsPage { space_id: SpacePartition, report_id: String },

                        #[route("/analyzes/poll/:poll_id")]
                        SpaceAnalyzeDetailPage { space_id: SpacePartition, poll_id: SpacePollEntityType },

                        #[route("/analyzes/discussion/:discussion_id")]
                        SpaceAnalyzeDiscussionPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },

                        #[route("/panels")]
                        SpacePanelsAppPage { space_id: SpacePartition },

                        #[route("/incentive-pool")]
                        SpaceIncentivePoolAppPage { space_id: SpacePartition },
                    #[end_layout]
                #[end_nest]
            #[end_layout]
        #[end_nest]

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
