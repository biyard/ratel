use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[nest("/")]
    #[layout(SocialLayout)]
    #[end_nest]
    #[nest("/landing")]
    #[layout(LandingLayout)]
    #[route("/")]
    HomePage {},

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

    #[route("/my-profile")]
    MyProfilePage {},
    #[route("/become-sponsor")]
    BecomeSponsorPage {},
    #[end_layout]
    #[end_nest]
    #[redirect("/", || Route::HomePage {  })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
