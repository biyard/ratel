use crate::pages::*;
use bdk::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/en")]
    #[layout(RootLayout)]
    #[route("/")]
    HomePage {  },

    #[route("/politicians")]
    PoliticiansPage {  },
    #[route("/politicians/:id")]
    PoliticiansByIdPage { id: i64 },

    #[route("/presidential-election")]
    PresidentialElectionPage {  },

    #[route("/privacy-policy")]
    PrivacyPolicyPage {  },

    #[route("/preparing")]
    PreparingPage {  },

    #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage {  })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
