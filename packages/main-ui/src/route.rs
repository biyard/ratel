use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SocialLayout)]
        #[route("/social")]
        IndexPage {},
        #[route("/social/:id")]
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
    #[end_layout]

    #[layout(LandingLayout)]
        #[route("/")]
        LandingPage {},
        #[route("/politicians")]
        PoliticiansPage {},
        #[route("/politicians/:id")]
        PoliticiansByIdPage { id: i64 },
        #[route("/presidential-election")]
        PresidentialElectionPage {},
        #[route("/privacy-policy")]
        PrivacyPolicyPage {},
        #[route("/preparing")]
        PreparingPage {},
        #[route("/become-sponsor")]
        BecomeSponsorPage {},
        #[route("/quizzes")]
        QuizzesPage {},

        #[route("/quizzes/results/:id")]
        ResultsPage {id: String},
    #[end_layout]

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
