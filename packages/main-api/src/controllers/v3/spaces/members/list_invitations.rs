use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::invitations::SpaceEmailVerification;
use crate::features::spaces::invitations::SpaceInvitationMember;
use crate::features::spaces::invitations::SpaceInvitationMemberQueryOption;
use crate::features::spaces::invitations::SpaceInvitationMemberResponse;
use crate::types::EntityType;
use crate::types::Partition;
use crate::{AppState, Error, models::user::User};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

pub async fn list_invitations_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<Vec<SpaceInvitationMemberResponse>>, Error> {
    if user.is_none() {
        return Ok(Json(vec![]));
    }

    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let members: Vec<SpaceInvitationMemberResponse> =
        SpaceInvitationMember::list_invitation_members(&dynamo, &space_pk).await?;

    Ok(Json(members))
}
