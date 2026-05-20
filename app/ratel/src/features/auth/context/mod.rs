mod user_context;
use crate::*;

pub use user_context::*;

use crate::features::auth::{controllers::get_me_handler, *};

#[derive(Clone, Copy)]
pub struct Context {
    pub user_context: Store<UserContext>,
}

impl Context {
    pub fn init() -> Result<Self, Loading> {
        let user_ctx = use_loader(move || async move {
            let res = Ok::<_, Error>(match get_me_handler().await {
                Ok(resp) => UserContext {
                    user: resp.user,
                    refresh_token: None,
                    membership: resp.membership,
                },
                Err(e) => UserContext::default(),
            });
            res
        });

        let user_ctx = user_ctx?();

        let ctx = Self {
            user_context: use_store(move || user_ctx),
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }
}
