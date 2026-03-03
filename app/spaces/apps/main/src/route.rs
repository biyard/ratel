use crate::*;

use crate::views::AppMainPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps")]
        #[route("/")]
        AppMainPage { space_id: SpacePartition },
}
