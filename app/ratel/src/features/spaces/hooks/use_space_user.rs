use crate::spaces::space_common::{use_space, SpaceResponse};
use crate::*;
use crate::{auth::use_user_context, features::spaces::controllers::user::get_user};

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
    let user_inner = user();
    let user_pk = user_inner.user_pk().ok_or(Error::UnauthorizedAccess)?;

    if participated {
        return Ok(SpaceUser {
            pk: user_pk,
            username: participant_username.unwrap_or_default(),
            display_name: participant_display_name.unwrap_or_default(),
            profile_url: participant_profile_url.unwrap_or_default(),
        });
    }

    let user_obj = user_inner.user.as_ref().ok_or(Error::UnauthorizedAccess)?;
    Ok(SpaceUser {
        pk: user_pk,
        username: user_obj.username.clone(),
        display_name: user_obj.display_name.clone(),
        profile_url: user_obj.profile_url.clone(),
    })
}
