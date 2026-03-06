use crate::{providers::use_space_context, *};

pub fn use_space_role() -> ReadSignal<SpaceUserRole> {
    let ctx = use_space_context();

    ctx.current_role.into()
}
