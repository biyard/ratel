use names::{Generator, Name};

use crate::Permissions;
use crate::features::spaces::members::{InvitationStatus, SpaceInvitationMember};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ParticipateSpaceRequest {
    #[schemars(description = "Proof if the user has rights to participate in the space")]
    pub verifiable_presentation: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
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
    let now = time::get_now_timestamp_millis();

    tracing::debug!("Handling request: {:?}", req);
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    if space.block_participate {
        return Err(Error::ParticipationBlocked);
    }

    let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
    space
        .check_if_satisfying_panel_attribute(&dynamo.client, &user)
        .await?;

    if space.visibility != SpaceVisibility::Public {
        let member =
            SpaceInvitationMember::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

        tracing::debug!("display_name generated: {:?}", member);
        let Some(member) = member else {
            return Err(Error::NoPermission);
        };
        match member.status {
            InvitationStatus::Invited => {}
            InvitationStatus::Pending => return Err(Error::NoPermission),
            InvitationStatus::Accepted | InvitationStatus::Declined => {
                let (pk, sk) = SpaceParticipant::keys(space.pk.clone(), user.pk.clone());

                let participant = SpaceParticipant::get(&dynamo.client, pk, Some(sk)).await?;
                if participant.is_none() {
                    let sp = if space.anonymous_participation {
                        let display_name = Generator::with_naming(Name::Numbered)
                            .next()
                            .unwrap()
                            .replace('-', " ");
                        SpaceParticipant::new(space.pk.clone(), user.pk.clone(), display_name)
                    } else {
                        SpaceParticipant::new_non_anonymous(space.pk.clone(), user.clone())
                    };
                    let new_space = SpaceCommon::updater(&space.pk, &space.sk)
                        .increase_participants(1)
                        .with_updated_at(now);

                    transact_write!(
                        &dynamo.client,
                        sp.create_transact_write_item(),
                        new_space.transact_write_item(),
                    )?;

                    return Ok(Json(ParticipateSpaceResponse {
                        username: sp.username,
                        display_name: sp.display_name,
                        profile_url: sp.profile_url,
                    }));
                } else {
                    return Err(Error::AlreadyParticipating);
                }
            }
        }
    }

    let sp = if space.anonymous_participation {
        let display_name = Generator::with_naming(Name::Numbered)
            .next()
            .unwrap()
            .replace('-', " ");
        SpaceParticipant::new(space.pk.clone(), user.pk.clone(), display_name)
    } else {
        SpaceParticipant::new_non_anonymous(space.pk.clone(), user.clone())
    };

    let new_space = SpaceCommon::updater(&space.pk, &space.sk)
        .increase_participants(1)
        .with_updated_at(now);

    let invitation = SpaceInvitationMember::new(space.pk.clone(), user.clone())
        .with_status(InvitationStatus::Accepted);

    transact_write!(
        &dynamo.client,
        sp.create_transact_write_item(),
        new_space.transact_write_item(),
        invitation.upsert_transact_write_item(),
    )?;

    tracing::debug!("space participants: {:?}", sp);

    Ok(Json(ParticipateSpaceResponse {
        username: sp.username,
        display_name: sp.display_name,
        profile_url: sp.profile_url,
    }))
}
