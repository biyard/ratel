use crate::*;

use crate::views::Index;
use crate::views::PostDetail;
use crate::views::PostEdit;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/posts")]
        #[route("/")]
        Index { },
        #[route("/:post_id/edit")]
        PostEdit { post_id: FeedPartition },
        #[route("/:post_id")]
        PostDetail { post_id: FeedPartition },
}
