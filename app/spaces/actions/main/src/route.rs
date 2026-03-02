use crate::*;

use crate::views::MainPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions")]
        #[route("/")]
        MainPage { space_id: SpacePartition },
}
