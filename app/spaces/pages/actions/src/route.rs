use crate::*;

use crate::views::MainPage;
use space_action_poll::Route as PollRoute;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions")]
        #[route("/")]
        MainPage { space_id: SpacePartition },

        #[route("/polls/:..rest")]
        PollApp { space_id: SpacePartition, rest: Vec<String> },
}

#[component]
fn PollApp(space_id: SpacePartition, rest: Vec<String>) -> Element {
    rsx! {
        Router::<PollRoute> {}
    }
}
