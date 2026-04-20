use crate::spaces::space_common::{use_space, SpaceResponse};
use crate::*;
use crate::{auth::use_user_context, features::spaces::controllers::user::get_user};
use dioxus::fullstack::{Loader, Loading};

pub struct SpaceUser {
    pub pk: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

#[track_caller]
pub fn use_space_user() -> Result<SpaceUser> {
    let SpaceResponse {
        participated,
        participant_display_name,
        participant_profile_url,
        participant_username,
        ..
    } = use_space()();
    let user = use_user_context();
    let user_pk = user().user_pk().ok_or(Error::UnauthorizedAccess)?;

    if !participated && participant_display_name.is_none()
        || participant_username.is_none()
        || participant_profile_url.is_none()
    {
        return Err(Error::UnauthorizedAccess);
    }

    Ok(SpaceUser {
        pk: user_pk,
        username: participant_username.unwrap(),
        display_name: participant_display_name.unwrap(),
        profile_url: participant_profile_url.unwrap(),
    })
}
