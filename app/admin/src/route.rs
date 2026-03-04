use crate::*;

use views::AdminMainPage;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(layout::AppLayout)]
        #[route("/")]
        AdminMainPage {},
    #[end_layout]

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}
