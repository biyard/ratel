use crate::*;

use crate::views::{HomePage, NewActionPage};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions")]
        #[route("/")]
        HomePage { space_id: SpacePartition },

        #[route("/new")]
        NewActionPage { space_id: SpacePartition },
}
