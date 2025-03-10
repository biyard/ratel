use crate::pages::*;
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/")]
            HomePage { lang: Language },

            // #[route("/politicians")]
            // PoliticianStatusPage { lang: Language },

            // #[nest("/topics")]
            //     #[route("/")]
            //     TopicsPage { lang: Language },
            //     #[route("/new?:step&:legislation_id")]
            //     NewTopicPage { lang: Language, step: NewTopicStep, legislation_id: QueryParam<String> },
            //     #[route("/:id")]
            //     TopicsByIdPage { id: String, lang: Language },
            // #[end_nest]

            // #[nest("/patrons")]
            //     #[route("/")]
            //     PatronsPage { lang: Language },
            //     #[route("/new")]
            //     NewPatronPage { lang: Language },
            //     #[route("/new_login")]
            //     PatronsLoginPage { id: String, lang: Language },
            //     #[route("/:id")]
            //     PatronsByIdPage { id: String, lang: Language },
            // #[end_nest]
        #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
