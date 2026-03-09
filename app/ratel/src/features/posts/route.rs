use crate::features::posts::*;

use crate::features::posts::views::Index;
use crate::features::posts::views::PostDetail;
use crate::features::posts::views::PostEdit;

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
