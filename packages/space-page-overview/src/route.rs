use crate::*;

use crate::views::HomePage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/overview")]
        #[route("/")]
        HomePage { space_id: SpacePartition },
}
