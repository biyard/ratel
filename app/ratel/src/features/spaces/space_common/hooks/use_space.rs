use crate::features::spaces::space_common::{controllers::SpaceResponse, providers::use_space_context};
use dioxus::fullstack::Loader;

pub fn use_space() -> Loader<SpaceResponse> {
    let ctx = use_space_context();
    ctx.space
}
