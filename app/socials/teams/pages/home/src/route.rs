use crate::*;

use views::Home;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/teams/:teamname/home")]
        #[route("/")]
        Main { teamname: String },
}

#[component]
pub fn Main(teamname: String) -> Element {
    rsx! {
        views::Home { teamname }
    }
}
