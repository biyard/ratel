use dioxus::fullstack::{Loader, Loading};

use crate::{controllers::get_user_role, *};

pub fn use_user_role() -> SpaceUserRole {
    let role = use_context::<Loader<SpaceUserRole>>();
    role.read().clone()
}

pub fn user_role_provider(space_id: SpacePartition) -> std::result::Result<(), Loading> {
    let role = use_loader(move || get_user_role(space_id.clone()))?;
    debug!("role: {:#?}", role);
    use_context_provider(|| role.clone());
    Ok(())
}

pub fn reload_user_role() {
    let mut ctx = use_context::<Loader<SpaceUserRole>>();
    ctx.restart();
}
