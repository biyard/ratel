use crate::{context::*, *};

pub fn use_user_context() -> Store<UserContext> {
    use_context::<Context>().user_context
}
