use crate::features::spaces::apps::file::views::HomePage;
use crate::features::spaces::apps::file::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/file")]
        #[route("/")]
        HomePage { space_id: SpacePartition },

}
