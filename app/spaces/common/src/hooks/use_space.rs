use dioxus::fullstack::{Loader, Loading};

use crate::{
    controllers::{SpaceResponse, get_space},
    types::space_key,
    *,
};

pub fn use_space_query(
    space_id: &SpacePartition,
) -> dioxus::prelude::Result<Loader<SpaceResponse>, Loading> {
    let key = space_key(space_id);
    use_query(&key, {
        let space_id = space_id.clone();
        move || get_space(space_id.clone())
    })
}
