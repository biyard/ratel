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
            let mut member: SpaceInvitationMemberResponse = response.into();

            let verification = SpaceEmailVerification::get(
                &dynamo.client,
                &space_pk,
                Some(EntityType::SpaceEmailVerification(member.email.clone())),
            )
            .await?;

            if verification.is_some() && verification.unwrap_or_default().authorized {
                member.authorized = true;
            } else {
                member.authorized = false;
            }

            members.push(member);
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(Json(members))
}
