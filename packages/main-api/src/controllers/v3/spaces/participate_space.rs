use names::{Generator, Name};

use crate::Permissions;
use crate::features::spaces::members::{InvitationStatus, SpaceInvitationMember};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceRequest {
    #[schemars(description = "Proof if the user has rights to participate in the space")]
    pub verifiable_presentation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

pub async fn participate_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<ParticipateSpaceRequest>,
) -> Result<Json<ParticipateSpaceResponse>> {
    tracing::debug!("Handling request: {:?}", req);
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    // TODO: Check verifiable_presentation and add user as SpaceParticipant

    // let is_verified = SpaceParticipant::verify_credential(&dynamo, &space_pk, user.clone()).await;

    // if !is_verified {
    //     return Err(Error::InvalidPanel);
    // }

    let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);

    if space.visibility != SpaceVisibility::Public {
        let member =
            SpaceInvitationMember::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

        tracing::debug!("display_name generated: {:?}", member);

        if member.is_none() {
            return Err(Error::NoPermission);
        } else if let Some(member) = member {
            match member.status {
                InvitationStatus::Invited => {}
                InvitationStatus::Pending => return Err(Error::NoPermission),
                InvitationStatus::Accepted | InvitationStatus::Declined => {
                    return Err(Error::AlreadyParticipating);
                }
            }
        }
    }

    let now = time::get_now_timestamp_millis();

    let display_name = Generator::with_naming(Name::Numbered)
        .next()
        .unwrap()
        .replace('-', " ");

    let sp = SpaceParticipant::new(space.pk.clone(), user.pk.clone(), display_name);
    let new_space = SpaceCommon::updater(&space.pk, &space.sk)
        .increase_participants(1)
        .with_updated_at(now);

    let invitation =
        SpaceInvitationMember::updater(&pk, &sk).with_status(InvitationStatus::Accepted);

    transact_write!(
        &dynamo.client,
        sp.create_transact_write_item(),
        new_space.transact_write_item(),
        invitation.transact_write_item(),
    )?;

    Ok(Json(ParticipateSpaceResponse {
        username: sp.username,
        display_name: sp.display_name,
        profile_url: sp.profile_url,
    }))
}
