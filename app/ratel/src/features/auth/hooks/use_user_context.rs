use crate::features::auth::{context::*, *};

pub fn use_user_context() -> Store<UserContext> {
    use_context::<AuthContext>().user_context
}
