use crate::features::spaces::actions::main::*;

use crate::features::spaces::actions::main::views::MainPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions")]
        #[route("/")]
        MainPage { space_id: SpacePartition },
}
