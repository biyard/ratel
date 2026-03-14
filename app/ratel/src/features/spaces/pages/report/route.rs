use crate::features::spaces::pages::report::*;

use crate::features::spaces::pages::report::views::SpaceReportPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/report")]
        #[route("/")]
        HomePage { space_id: SpacePartition },
}
