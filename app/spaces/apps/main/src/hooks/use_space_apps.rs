use dioxus::fullstack::{Loader, Loading};

use crate::*;

pub fn use_space_apps(
    space_id: &SpacePartition,
) -> dioxus::prelude::Result<Loader<Vec<SpaceApp>>, Loading> {
    use_loader({
        let space_id = space_id.clone();
        move || get_space_apps(space_id.clone())
    })
}
