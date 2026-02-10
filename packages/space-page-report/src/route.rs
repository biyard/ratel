use crate::*;

use crate::views::HomePage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/spaces/:space_id/report")]
    HomePage { space_id: SpacePartition },
}
