use crate::features::spaces::pages::overview::*;

use crate::features::spaces::pages::overview::views::HomePage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/overview")]
        #[route("/")]
        HomePage { space_id: SpacePartition },
}
