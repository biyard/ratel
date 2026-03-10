use crate::features::spaces::pages::apps::apps::file::views::HomePage;
use crate::features::spaces::pages::apps::apps::file::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/file")]
        #[route("/")]
        HomePage { space_id: SpacePartition },

}
