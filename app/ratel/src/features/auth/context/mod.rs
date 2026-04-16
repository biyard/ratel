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
            // Mobile clients lose their reqwest cookie jar on every app
            // restart. If we have a cached refresh token, exchange it for
            // a fresh server session before the get_me call below — that's
            // the only way the user actually stays signed in across
            // restarts. Web targets fall through fast (no cached token).
            crate::features::auth::services::try_restore_session().await;

            Ok::<_, Error>(match get_me_handler().await {
                Ok(resp) => UserContext {
                    user: resp.user,
                    refresh_token: None,
                    membership: resp.membership,
                },
                Err(e) => {
                    // Server says no session — wipe any cached user blob
                    // and refresh token so the next restart doesn't try
                    // to restore a dead session in a loop.
                    crate::common::services::persistent_state::clear_cached_session();
                    crate::features::auth::services::restore_session::clear_refresh_token();
                    crate::error!("get_me failed during Context::init: {e}");
                    UserContext::default()
                }
            })
        })?();

        let ctx = Self {
            user_context: use_store(move || user_ctx),
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }
}
