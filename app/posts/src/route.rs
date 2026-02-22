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
        #[route("/:post_pk/edit")]
        PostEdit { post_pk: String },
        #[route("/:post_pk")]
        PostDetail { post_pk: String },
}
