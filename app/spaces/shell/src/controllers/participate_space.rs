use crate::models::{
    InvitationStatus, SpaceInvitationMember, SpacePanelParticipant, SpacePanelQuota,
    SpaceParticipant,
};
use crate::*;
use common::models::auth::{OptionalUser, User};
use common::models::space::SpaceCommon;
use common::utils::time::get_now_timestamp_millis;
use common::SpaceVisibility;
use space_common::ratel_post::models::Post;
use space_common::ratel_post::types::TeamGroupPermission;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParticipateSpaceResponse {
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
}

#[post("/api/spaces/{space_id}/participate", user: OptionalUser)]
pub async fn participate_space(space_id: SpacePartition) -> Result<ParticipateSpaceResponse> {
    let config = crate::config::get();
    let dynamo = config.common.dynamodb();
    let now = get_now_timestamp_millis();

    let space_pk_partition: Partition = space_id.clone().into();
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

#[cfg(feature = "server")]
async fn check_if_satisfying_panel_attribute(
    space: &SpaceCommon,
    cli: &aws_sdk_dynamodb::Client,
    user: &User,
) -> Result<()> {
    use crate::models::UserAttributesExt;

    let panel_quota = SpacePanelQuota::query(
        cli,
        CompositePartition(space.pk.clone(), Partition::PanelAttribute),
        SpacePanelQuota::opt_all().sk("SPACE_PANEL_ATTRIBUTE#".to_string()),
    )
    .await
    .unwrap_or_default()
    .0;

    if panel_quota.is_empty() {
        return Ok(());
    }

    let user_attributes = user.get_attributes(cli).await?;
    let age: Option<u8> = user_attributes.age().and_then(|v| u8::try_from(v).ok());
    let gender = user_attributes.gender;

    if space.remains <= 0 {
        return Err(Error::FullQuota);
    }

    for q in panel_quota {
        if q.remains <= 0 {
            continue;
        }

        if let EntityType::SpacePanelAttribute(label, _) = &q.sk {
            if label.eq_ignore_ascii_case("university") {
                continue;
            }
        }

        if match_by_sk(age, gender, &q.sk) {
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

                transact_write!(
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

// #[cfg(not(feature = "server"))]
// async fn check_if_satisfying_panel_attribute(
//     _space: &SpaceCommon,
//     _cli: &(),
//     _user: &ratel_auth::User,
// ) -> Result<()> {
//     Ok(())
// }

fn match_by_sk(age: Option<u8>, gender: Option<crate::models::Gender>, sk: &EntityType) -> bool {
    if age.is_none() && gender.is_none() {
        return false;
    }

    let (label_raw, value_raw) = match sk {
        EntityType::SpacePanelAttribute(label, value) => (label.as_str(), value.as_str()),
        _ => return false,
    };

    let label = label_raw.to_ascii_lowercase();
    let value = value_raw.to_ascii_lowercase();

    match label.as_str() {
        "verifiable_attribute" => match value.as_str() {
            v if v.starts_with("age") => match_age_rule(age, v),
            v if v.starts_with("gender") => match_gender_rule(gender, v),
            _ => false,
        },
        "collective_attribute" => true,
        "gender" => {
            let encoded = format!("gender:{value}");
            match_gender_rule(gender, &encoded)
        }
        "university" => true,
        _ => false,
    }
}

fn match_age_rule(age: Option<u8>, v: &str) -> bool {
    if v == "age" {
        return age.is_some();
    }

    if let Some(rest) = v.strip_prefix("age:") {
        if let Some((min_s, max_s)) = rest.split_once('-') {
            if let (Ok(min), Ok(max)) = (min_s.trim().parse::<u8>(), max_s.trim().parse::<u8>()) {
                return age.map(|a| a >= min && a <= max).unwrap_or(false);
            }
        } else if let Ok(specific) = rest.trim().parse::<u8>() {
            return age.map(|a| a == specific).unwrap_or(false);
        }
    }

    true
}

fn match_gender_rule(gender: Option<crate::models::Gender>, v: &str) -> bool {
    if v == "gender" {
        return gender.is_some();
    }

    if let Some(rest) = v.strip_prefix("gender:") {
        let want = rest.trim().to_ascii_lowercase();
        return match (want.as_str(), gender) {
            ("male", Some(crate::models::Gender::Male)) => true,
            ("female", Some(crate::models::Gender::Female)) => true,
            _ => false,
        };
    }

    true
}
