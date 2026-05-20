use crate::features::auth::{context::*, *};

pub fn use_auth_context() -> AuthContext {
    use_context::<AuthContext>()
}

pub fn consume_auth_context() -> AuthContext {
    consume_context::<AuthContext>()
}
