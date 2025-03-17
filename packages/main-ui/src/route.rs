use crate::pages::*;
use bdk::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
    #[layout(RootLayout)]
    #[route("/")]
    HomePage { lang: Language, class:String },

    #[route("/politicians")]
    PoliticiansPage { lang: Language },
    #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::default(), class: String::new() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
