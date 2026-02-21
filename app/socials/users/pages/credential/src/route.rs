use crate::*;

use views::Home;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:username/credentials")]
        #[route("/")]
        Main { username: String },
}

#[component]
pub fn Main(username: String) -> Element {
    rsx! {
        views::Home { username }
    }
}
