use crate::*;
#[cfg(feature = "server")]
use ratel_auth::User;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Default)]
#[cfg_attr(feature = "server", derive(DynamoEnum))]
#[cfg(feature = "server")]
#[repr(u8)]
enum InvitationStatus {
    #[default]
    Pending = 1,
    Invited = 2,
    Accepted = 3,
    Declined = 4,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
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
            created_at: common::utils::time::get_now_timestamp_millis(),
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
    user: User
)]
pub async fn invite_space_participants(
    space_id: SpacePartition,
    req: InviteSpaceParticipantsRequest,
) -> common::Result<InviteSpaceParticipantsResponse> {
    use common::models::space::SpaceParticipant;
    use common::types::SpacePublishState;
    use ratel_auth::models::user::UserQueryOption;
    use std::collections::HashSet;

    let dynamo = crate::config::get().common.dynamodb();
    let space = super::get_space_and_ensure_admin(&space_id, &user).await?;
    let space_pk: Partition = space_id.into();

    let mut invite_targets = vec![];
    let mut invited_emails = vec![];
    let mut seen = HashSet::<String>::new();

    for raw_email in req.emails {
        let email = normalize_email(&raw_email)
            .ok_or_else(|| Error::BadRequest(format!("Invalid email: {}", raw_email)))?;

        if !seen.insert(email.clone()) {
            return Err(Error::BadRequest(format!("Duplicate email: {}", email)));
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
            return Err(Error::BadRequest(format!("Already participant: {}", email)));
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
                return Err(Error::BadRequest(format!("Already invited: {}", email)));
            }
        }

        invite_targets.push((email, target_user));
    }

    for (email, target_user) in invite_targets {
        let mut invitation = SpaceInvitationMember::new(space_pk.clone(), target_user);
        invitation.status = if space.publish_state == SpacePublishState::Published {
            InvitationStatus::Invited
        } else {
            InvitationStatus::Pending
        };

        invitation.upsert(dynamo).await?;
        invited_emails.push(email);
    }

    // TODO(main-api parity): send invitation emails for published spaces.
    // TODO(main-api parity): send invitation push notifications via FCM.

    Ok(InviteSpaceParticipantsResponse { invited_emails })
}
