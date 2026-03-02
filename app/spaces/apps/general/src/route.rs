use crate::*;

use crate::views::AppGeneralPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/apps/general")]
        #[route("/")]
        AppGeneralPage { space_id: SpacePartition },

}
