mod user_context;

use crate::common::dioxus::fullstack::Loading;
pub use user_context::*;

use crate::features::auth::{controllers::get_me_handler, *};

#[derive(Clone, Copy)]
pub struct Context {
    pub user_context: Store<UserContext>,
}

impl Context {
    pub fn init() -> Result<Self, Loading> {
        let user_ctx = use_loader(move || async move {
            Ok::<_, Error>(match get_me_handler().await {
                Ok(resp) => UserContext {
                    user: resp.user,
                    refresh_token: None,
                    #[cfg(feature = "membership")]
                    membership: resp.membership,
                },
                Err(_) => UserContext::default(),
            })
        })?();

        let ctx = Self {
            user_context: use_store(move || user_ctx),
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }
}
