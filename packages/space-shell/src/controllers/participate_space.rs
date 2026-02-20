use crate::models::{InvitationStatus, SpaceCommon, SpaceInvitationMember, SpaceParticipant};
use crate::*;
use common::utils::time::get_now_timestamp_millis;
use ratel_auth::models::user::OptionalUser;
use ratel_post::models::Post;
use ratel_post::types::{SpaceVisibility, TeamGroupPermission};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParticipateSpaceResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

#[post("/api/spaces/{space_pk}/participate", user: OptionalUser)]
pub async fn participate_space(space_pk: SpacePartition) -> Result<ParticipateSpaceResponse> {
    let config = crate::config::get();
    let dynamo = config.common.dynamodb();
    let now = get_now_timestamp_millis();

    let space_pk_partition: Partition = space_pk.clone().into();
    let space =
        SpaceCommon::get(dynamo, &space_pk_partition, Some(&EntityType::SpaceCommon)).await?;
    let space = space.ok_or_else(|| Error::NotFound("Space Not Found".to_string()))?;

    if space.block_participate {
        return Err(Error::ParticipationBlocked);
    }

    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(dynamo, &post_pk, Some(EntityType::Post)).await?;
    let post = post.ok_or_else(|| Error::NotFound("Post Not Found".to_string()))?;

    let user: Option<ratel_auth::User> = user.into();
    let user = user.ok_or(Error::NoSessionFound)?;

    let permissions = post.get_permissions(dynamo, Some(user.clone())).await?;
    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
    space
        .check_if_satisfying_panel_attribute(dynamo, &user)
        .await?;

    let (participant_pk, participant_sk) =
        SpaceParticipant::keys(space.pk.clone(), user.pk.clone());
    let participant = SpaceParticipant::get(dynamo, &participant_pk, Some(&participant_sk)).await?;
    if participant.is_some() {
        return Err(Error::AlreadyParticipating);
    }

    if space.visibility != SpaceVisibility::Public {
        let member = SpaceInvitationMember::get(dynamo, &pk, Some(&sk)).await?;
        let Some(member) = member else {
            return Err(Error::NoPermission);
        };
        match member.status {
            InvitationStatus::Invited => {}
            InvitationStatus::Pending => return Err(Error::NoPermission),
            InvitationStatus::Accepted | InvitationStatus::Declined => {
                let (pk, sk) = SpaceParticipant::keys(space.pk.clone(), user.pk.clone());

                let participant = SpaceParticipant::get(dynamo, &pk, Some(&sk)).await?;
                if participant.is_none() {
                    let sp = if space.anonymous_participation {
                        SpaceParticipant::new(space.pk.clone(), user.pk.clone())
                    } else {
                        SpaceParticipant::new_non_anonymous(space.pk.clone(), user.clone())
                    };
                    let new_space = SpaceCommon::updater(&space.pk, &space.sk)
                        .increase_participants(1)
                        .with_updated_at(now);

                    transact_write!(
                        dynamo,
                        sp.create_transact_write_item(),
                        new_space.transact_write_item(),
                    )?;

                    return Ok(ParticipateSpaceResponse {
                        username: sp.username,
                        display_name: sp.display_name,
                        profile_url: sp.profile_url,
                    });
                } else {
                    return Err(Error::AlreadyParticipating);
                }
            }
        }
    }

    let sp = if space.anonymous_participation {
        SpaceParticipant::new(space.pk.clone(), user.pk.clone())
    } else {
        SpaceParticipant::new_non_anonymous(space.pk.clone(), user.clone())
    };

    let space_update = SpaceCommon::updater(&space.pk, &space.sk)
        .increase_participants(1)
        .with_updated_at(now);
    let invitation = SpaceInvitationMember::new(space.pk.clone(), user.clone())
        .with_status(InvitationStatus::Accepted);

    transact_write!(
        dynamo,
        sp.create_transact_write_item(),
        space_update.transact_write_item(),
        invitation.upsert_transact_write_item(),
    )?;

    Ok(ParticipateSpaceResponse {
        username: sp.username,
        display_name: sp.display_name,
        profile_url: sp.profile_url,
    })
}
