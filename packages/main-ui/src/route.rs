use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[nest("/")]
    #[layout(SocialLayout)]
    #[route("/")]
    IndexPage {},
    #[route("/explore")]
    ExplorePage {},
    #[route("/my-network")]
    MyNetworkPage {},
    #[route("/notifications")]
    NotificationsPage {},
    #[route("/my-profile")]
    MyProfilePage {},
    #[end_nest]
    #[nest("/landing")]
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
    #[end_nest]
    #[redirect("/", || Route::LandingPage {  })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
