use crate::features::spaces::apps::main::*;

use crate::features::spaces::apps::main::views::AppMainPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps")]
        #[route("/")]
        AppMainPage { space_id: SpacePartition },
}
