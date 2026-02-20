mod user_context;

pub use user_context::*;

use crate::*;

#[derive(Clone, Copy)]
pub struct Context {
    pub user_context: Store<UserContext>,
}

impl Context {
    pub fn init() {
        let ctx = Self {
            user_context: use_store(|| UserContext::default()),
        };
        use_context_provider(move || ctx);
    }
}
