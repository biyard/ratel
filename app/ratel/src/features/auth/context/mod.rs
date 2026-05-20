mod user_context;
use crate::*;

pub use user_context::*;

use crate::features::auth::{controllers::get_me_handler, *};

#[derive(Clone, Copy, DioxusController)]
pub struct AuthContext {
    pub user_context: Store<UserContext>,
    pub logged_in: Memo<bool>,
}

impl AuthContext {
    pub fn init() -> Result<Self, Loading> {
        let user_ctx = use_loader(move || async move {
            let res = Ok::<_, Error>(match get_me_handler().await {
                Ok(resp) => UserContext {
                    user: resp.user,
                    refresh_token: None,
                    membership: resp.membership,
                },
                Err(_) => UserContext::default(),
            });
            res
        });

        let user_ctx = user_ctx?;
        let user_context = use_store(move || user_ctx());
        let logged_in = use_memo(move || {
            let ctx = user_context();
            debug!("AuthContext initialized with user: {:?}", ctx.user);
            ctx.user.is_some()
        });

        let ctx = Self {
            user_context,
            logged_in,
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }
}
