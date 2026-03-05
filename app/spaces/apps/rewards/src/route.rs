use crate::views::HomePage;
use crate::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/rewards")]
        #[route("/")]
        HomePage { space_id: SpacePartition },

}
