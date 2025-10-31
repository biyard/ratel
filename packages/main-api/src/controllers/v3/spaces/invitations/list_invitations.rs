use crate::controllers::v3::spaces::SpacePath;
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::invitations::SpaceInvitationMember;
use crate::features::spaces::invitations::SpaceInvitationMemberQueryOption;
use crate::features::spaces::invitations::SpaceInvitationMemberResponse;
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

    let mut members: Vec<SpaceInvitationMemberResponse> = vec![];
    let mut bookmark = None::<String>;

    loop {
        let (responses, new_bookmark) = SpaceInvitationMember::query(
            &dynamo.client,
            space_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceInvitationMemberQueryOption::builder()
                    .sk("SPACE_INVITATION_MEMBER#".into())
                    .bookmark(b.clone())
            } else {
                SpaceInvitationMemberQueryOption::builder().sk("SPACE_INVITATION_MEMBER#".into())
            },
        )
        .await?;

        for response in responses {
            members.push(response.into());
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(Json(members))
}
