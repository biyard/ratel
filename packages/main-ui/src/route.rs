use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SocialLayout)]
        #[route("/social")]
        IndexPage {},
        #[route("/explore")]
        ExplorePage {},
        #[route("/my-network")]
        MyNetworkPage {},
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
    #[end_layout]

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
