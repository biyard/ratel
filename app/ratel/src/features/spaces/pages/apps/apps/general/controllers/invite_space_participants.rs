use crate::features::spaces::pages::apps::apps::general::*;
use crate::features::spaces::pages::apps::types::SpaceAppError;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;
#[cfg(feature = "server")]
use crate::features::auth::User;
#[cfg(feature = "server")]
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InviteSpaceParticipantsRequest {
    pub emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InviteSpaceParticipantsResponse {
    pub invited_emails: Vec<String>,
}

#[cfg(feature = "server")]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default, DynamoEnum,
)]
#[repr(u8)]
enum InvitationStatus {
    #[default]
    Pending = 1,
    Invited = 2,
    Accepted = 3,
    Declined = 4,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
struct SpaceInvitationMember {
    #[cfg_attr(
        feature = "server",
        dynamo(
            index = "gsi3",
            name = "find_space_invitations_by_status",
            prefix = "SIM",
            pk
        )
    )]
    pub pk: Partition,
    #[cfg_attr(feature = "server", dynamo(index = "gsi1", sk))]
    pub sk: EntityType,
    #[cfg_attr(
        feature = "server",
        dynamo(
            prefix = "SPACE_INVITATION",
            name = "find_by_user_pk",
            index = "gsi1",
            pk
        )
    )]
    #[cfg_attr(
        feature = "server",
        dynamo(
            prefix = "SPACE_INVITATION",
            name = "find_user_invitations_by_status",
            index = "gsi2",
            pk
        )
    )]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", order = 1, sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi3", sk))]
    pub status: InvitationStatus,
    #[serde(default)]
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", order = 2, sk))]
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl SpaceInvitationMember {
    fn new(
        space_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            email,
            ..
        }: User,
    ) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceInvitationMember(pk.to_string()),
            user_pk: pk,
            display_name,
            profile_url,
            username,
            email,
            status: InvitationStatus::Pending,
            created_at: crate::common::utils::time::get_now_timestamp_millis(),
        }
    }

    fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceInvitationMember(user_pk.to_string()),
        )
    }
}

#[cfg(feature = "server")]
fn normalize_email(raw: &str) -> Option<String> {
    let email = raw.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return None;
    }
    Some(email)
}

#[post(
    "/api/spaces/{space_id}/participants/invitations",
    role: SpaceUserRole
)]
pub async fn invite_space_participants(
    space_id: SpacePartition,
    req: InviteSpaceParticipantsRequest,
) -> crate::common::Result<InviteSpaceParticipantsResponse> {
    use crate::common::models::space::{SpaceCommon, SpaceParticipant};
    use crate::common::types::{InboxPayload, SpacePublishState};
    use crate::features::auth::models::user::UserQueryOption;
    use crate::features::posts::models::Post;
    use std::collections::HashSet;

    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let space = SpaceCommon::get(dynamo, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;

    let mut invite_targets = vec![];
    let mut invited_emails = vec![];
    let mut seen = HashSet::<String>::new();

    for raw_email in req.emails {
        let email = normalize_email(&raw_email)
            .ok_or_else(|| SpaceAppError::InvalidInvitationEmail)?;

        if !seen.insert(email.clone()) {
            continue;
        }

        let (users, _) =
            User::find_by_email(dynamo, &email, UserQueryOption::builder().limit(1)).await?;
        let target_user = users
            .into_iter()
            .next()
            .ok_or_else(|| Error::NotFound(format!("User not found: {}", email)))?;

        let (participant_pk, participant_sk) =
            SpaceParticipant::keys(space_pk.clone(), target_user.pk.clone());
        if SpaceParticipant::get(dynamo, &participant_pk, Some(&participant_sk))
            .await?
            .is_some()
        {
            continue;
        }

        let (invitation_pk, invitation_sk) =
            SpaceInvitationMember::keys(&space_pk, &target_user.pk);
        if let Some(existing_invitation) =
            SpaceInvitationMember::get(dynamo, &invitation_pk, Some(&invitation_sk)).await?
        {
            if matches!(
                existing_invitation.status,
                InvitationStatus::Pending | InvitationStatus::Invited
            ) {
                continue;
            }
        }

        invite_targets.push((email, target_user));
    }

    let mut invited_user_pks: Vec<Partition> = Vec::new();
    for (email, target_user) in invite_targets {
        let invitee_pk = target_user.pk.clone();
        let mut invitation = SpaceInvitationMember::new(space_pk.clone(), target_user);
        invitation.status = if space.publish_state == SpacePublishState::Published {
            InvitationStatus::Invited
        } else {
            InvitationStatus::Pending
        };

        invitation.upsert(dynamo).await?;
        invited_emails.push(email);
        invited_user_pks.push(invitee_pk);
    }

    // Fan inbox rows per invited user (idempotent per space+invitee).
    // SpaceCommon does not hold a title; the Feed post title is the canonical title.
    if !invited_user_pks.is_empty() {
        let space_id_for_inbox: SpacePartition = space.pk.clone().into();
        let space_title = match space.pk.clone().to_post_key() {
            Ok(post_pk) => Post::get(dynamo, &post_pk, Some(&EntityType::Post))
                .await
                .ok()
                .flatten()
                .map(|p| p.title)
                .unwrap_or_default(),
            Err(_) => String::new(),
        };
        let inviter_name = if !space.author_display_name.is_empty() {
            space.author_display_name.clone()
        } else {
            space.author_username.clone()
        };
        let cta_url = match &space.pk {
            Partition::Space(id) => {
                format!("https://ratel.foundation/spaces/SPACE%23{}", id)
            }
            _ => String::new(),
        };

        for invitee_pk in &invited_user_pks {
            let payload = InboxPayload::SpaceInvitation {
                space_id: space_id_for_inbox.clone(),
                space_title: space_title.clone(),
                inviter_name: inviter_name.clone(),
                cta_url: cta_url.clone(),
            };
            let dedup_source = format!("{}#{}", space.pk, invitee_pk);
            if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
                invitee_pk.clone(),
                payload,
                &dedup_source,
            )
            .await
            {
                crate::error!("space-invitation inbox row failed: {e}");
            }
        }
    }

    // TODO(main-api parity): send invitation emails for published spaces.
    // TODO(main-api parity): send invitation push notifications via FCM.

    Ok(InviteSpaceParticipantsResponse { invited_emails })
}
