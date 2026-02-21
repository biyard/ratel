use crate::*;

use crate::views::Index;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/posts")]
        #[route("/")]
        Index { },
}
