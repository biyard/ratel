use crate::features::spaces::apps::panels::views::HomePage;
use crate::features::spaces::apps::panels::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/panels")]
        #[route("/")]
        HomePage { space_id: SpacePartition },

}
