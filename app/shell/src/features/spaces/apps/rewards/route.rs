use crate::features::spaces::apps::rewards::views::HomePage;
use crate::features::spaces::apps::rewards::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/rewards")]
        #[route("/")]
        HomePage { space_id: SpacePartition },

}
