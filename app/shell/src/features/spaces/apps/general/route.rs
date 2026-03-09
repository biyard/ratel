use crate::features::spaces::apps::general::*;

use crate::features::spaces::apps::general::views::AppGeneralPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/general")]
        #[route("/")]
        AppGeneralPage { space_id: SpacePartition },

}
