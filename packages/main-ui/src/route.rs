use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SocialLayout)]
        #[layout(MyPageLayout)]
            #[route("/social")]
            IndexPage {},
        #[end_layout]

        #[route("/threads/:id")]
        ThreadPage { id: i64 },
        #[route("/explore")]
        ExplorePage {},
        #[route("/my-network")]
        MyNetworkPage {},
        #[route("/message")]
        MessagesPage {},
        #[route("/notifications")]
        NotificationsPage {},
        #[route("/my-profile")]
        MyProfilePage {},
        #[route("/politicians")]
        PoliticiansPage {},
        #[route("/politicians/:id")]
        PoliticiansByIdPage { id: i64 },
        #[route("/presidential-election")]
        PresidentialElectionPage {},

        #[nest("/teams/:teamname")]
    #[layout(TeamsByIdLayout)]
    #[route("/")]
    TeamsByIdPage { teamname: String },
    #[end_layout]
    #[end_nest]
    #[end_layout]

    #[layout(LandingLayout)]
        #[route("/")]
        LandingPage {},
        #[route("/privacy-policy")]
        PrivacyPolicyPage {},
        #[route("/preparing")]
        PreparingPage {},
        #[route("/become-sponsor")]
        BecomeSponsorPage {},
        #[route("/quizzes")]
        QuizzesPage {},

        #[route("/politician")]
        PoliticiansPageForLanding {},
        #[route("/presidential-elections")]
        PresidentialElectionPageForLanding {},

        #[route("/advocacy-campaigns/:id")]
        AdvocacyCampaignsByIdPage { id: i64},

        #[route("/quizzes/results/:id")]
        ResultsPage {id: String},
    #[end_layout]

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
