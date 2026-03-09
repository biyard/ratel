use crate::features::auth::*;

use crate::features::auth::views::{ForgotPassword, Index};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/auth")]
        #[route("/")]
        Index { },

        #[route("/forgot-password")]
        ForgotPassword { },
}
