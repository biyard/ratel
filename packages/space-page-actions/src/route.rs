use crate::*;

use crate::views::ListActionPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions")]
        #[route("/")]
        ListActionPage { space_id: SpacePartition },        
}
