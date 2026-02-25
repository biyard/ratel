use dioxus::fullstack::{Loader, Loading};

use crate::{
    controllers::{SpaceResponse, get_space},
    *,
};

pub fn use_space() -> SpaceResponse {
    let space = use_context::<Loader<SpaceResponse>>();
    space.read().clone()
}

pub fn space_provider(space_id: SpacePartition) -> std::result::Result<(), Loading> {
    let space = use_loader(move || get_space(space_id.clone()))?;
    use_context_provider(|| space.clone());
    Ok(())
}

pub fn reload_space() {
    let mut ctx = use_context::<Loader<SpaceResponse>>();
    ctx.restart();
}
