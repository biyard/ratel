use crate::common::models::auth::{OptionalUser, User};
use crate::common::models::space::SpaceCommon;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::SpaceVisibility;
use crate::features::posts::models::Post;
use crate::features::posts::types::TeamGroupPermission;
use crate::features::spaces::models::{
    InvitationStatus, SpaceInvitationMember, SpacePanelParticipant, SpacePanelQuota,
    SpaceParticipant,
};
use crate::features::spaces::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParticipateSpaceResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

#[post("/api/spaces/{space_id}/participate", user: OptionalUser)]
pub async fn participate_space(space_id: SpacePartition) -> Result<ParticipateSpaceResponse> {
    let config = crate::features::spaces::config::get();
    let dynamo = config.common.dynamodb();
    let now = get_now_timestamp_millis();

    let space_pk_partition: Partition = space_id.into();
    let space =
        SpaceCommon::get(dynamo, &space_pk_partition, Some(&EntityType::SpaceCommon)).await?;
    let space = space.ok_or_else(|| Error::NotFound("Space Not Found".to_string()))?;

    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(dynamo, &post_pk, Some(EntityType::Post)).await?;
    let post = post.ok_or_else(|| Error::NotFound("Post Not Found".to_string()))?;

    let user: Option<User> = user.into();
    let user = user.ok_or(Error::NoSessionFound)?;

    let permissions = post.get_permissions(dynamo, Some(user.clone())).await?;
    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
    #[cfg(feature = "server")]
    check_if_satisfying_panel_attribute(&space, dynamo, &user).await?;

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

                    crate::transact_write!(
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

    crate::transact_write!(
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

#[cfg(feature = "server")]
async fn check_if_satisfying_panel_attribute(
    space: &SpaceCommon,
    cli: &aws_sdk_dynamodb::Client,
    user: &User,
) -> Result<()> {
    use crate::features::spaces::models::UserAttributesExt;

    let panel_quota =
        crate::features::spaces::controllers::panel_requirements::list_panel_quotas(cli, &space.pk)
            .await?;

    if panel_quota.is_empty() {
        return Ok(());
    }

    let user_attributes = user.get_attributes(cli).await?;
    let age: Option<u8> = user_attributes.age().and_then(|v| u8::try_from(v).ok());
    let gender = user_attributes.gender;
    let has_university = user_attributes
        .university
        .as_ref()
        .map(|value| !value.is_empty())
        .unwrap_or(false);

    if space.remains <= 0 {
        return Err(Error::FullQuota);
    }

    for q in panel_quota {
        if q.remains <= 0 {
            continue;
        }

        if crate::features::spaces::controllers::panel_requirements::panel_matches_user(
            age,
            gender,
            has_university,
            &q,
        ) {
            let pk = q.pk;
            let sk = q.sk;

            let (panel_pk, panel_sk) =
                SpacePanelParticipant::keys(&space.pk.clone(), &user.pk.clone());

            let participant =
                SpacePanelParticipant::get(cli, panel_pk, Some(panel_sk.clone())).await?;

            if participant.is_none() {
                let participants = SpacePanelParticipant::new(space.pk.clone(), user.clone());

                let space_updater = SpaceCommon::updater(space.pk.clone(), EntityType::SpaceCommon)
                    .decrease_remains(1);

                let quota_updater =
                    SpacePanelQuota::updater(pk.clone(), sk.clone()).decrease_remains(1);

                crate::transact_write!(
                    cli,
                    participants.create_transact_write_item(),
                    space_updater.transact_write_item(),
                    quota_updater.transact_write_item(),
                )?;
            }

            return Ok(());
        }
    }

    Err(Error::LackOfVerifiedAttributes)
}
