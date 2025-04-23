use crate::pages::*;
use bdk::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
    #[layout(RootLayout)]
    #[route("/")]
    HomePage { lang: Language },

    #[route("/politicians")]
    PoliticiansPage { lang: Language },
    #[route("/politicians/:id")]
    PoliticiansByIdPage { lang: Language, id: i64 },

    #[route("/presidential-election")]
    PresidentialElectionPage { lang: Language },

    #[route("/privacy-policy")]
    PrivacyPolicyPage { lang: Language },

    #[route("/preparing")]
    PreparingPage { lang: Language },

    #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
