use crate::*;
#[cfg(feature = "server")]
use common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use common::models::User;
#[cfg(feature = "server")]
use common::utils::aws::SesClient;
#[cfg(feature = "server")]
use common::utils::time::get_now_timestamp_millis;

use super::SpaceEmailVerification;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    DynamoEnum,
)]
#[repr(u8)]
pub enum InvitationStatus {
    #[default]
    Pending = 1,
    Invited = 2,
    Accepted = 3,
    Declined = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
pub struct SpaceInvitationMember {
    #[dynamo(
        index = "gsi3",
        name = "find_space_invitations_by_status",
        prefix = "SIM",
        pk
    )]
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(
        prefix = "SPACE_INVITATION",
        name = "find_by_user_pk",
        index = "gsi1",
        pk
    )]
    #[dynamo(
        prefix = "SPACE_INVITATION",
        name = "find_user_invitations_by_status",
        index = "gsi2",
        pk
    )]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
    #[dynamo(index = "gsi2", order = 1, sk)]
    #[dynamo(index = "gsi3", sk)]
    pub status: InvitationStatus,

    #[serde(default)]
    #[dynamo(index = "gsi2", order = 2, sk)]
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpaceInvitationMemberResponse {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
    pub authorized: bool,
}

impl From<SpaceInvitationMember> for SpaceInvitationMemberResponse {
    fn from(member: SpaceInvitationMember) -> Self {
        Self {
            user_pk: member.user_pk,
            display_name: member.display_name,
            profile_url: member.profile_url,
            username: member.username,
            email: member.email,
            authorized: false,
        }
    }
}

impl SpaceInvitationMember {
    #[cfg(feature = "server")]
    pub fn new(space_pk: Partition, user: User) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceInvitationMember(user.pk.to_string()),
            user_pk: user.pk,
            display_name: user.display_name,
            profile_url: user.profile_url,
            username: user.username,
            email: user.email,
            status: InvitationStatus::Pending,
            created_at: get_now_timestamp_millis(),
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceInvitationMember(user_pk.to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl SpaceInvitationMember {
    pub async fn find_user_invitations_by_status_latest(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        mut opt: SpaceInvitationMemberQueryOption,
    ) -> Result<(Vec<Self>, Option<String>)> {
        opt.scan_index_forward = false;
        SpaceInvitationMember::find_user_invitations_by_status(cli, user_pk, opt).await
    }

    pub async fn send_email(
        ddb: &aws_sdk_dynamodb::Client,
        ses: &SesClient,
        space: &SpaceCommon,
        title: String,
    ) -> Result<()> {
        let (responses, _) = SpaceInvitationMember::find_space_invitations_by_status(
            ddb,
            space.pk.clone(),
            SpaceInvitationMember::opt_all().sk(InvitationStatus::Pending.to_string()),
        )
        .await?;

        let updates = responses.iter().map(|member| {
            let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &member.user_pk);
            SpaceInvitationMember::updater(pk, sk)
                .with_status(InvitationStatus::Invited)
                .execute(ddb)
        });

        let emails: Vec<String> = responses
            .iter()
            .map(|member| member.email.clone())
            .collect();

        futures::future::try_join_all(updates).await?;

        if !emails.is_empty() {
            SpaceEmailVerification::send_invitation_emails(ddb, ses, emails, space, title).await?;
        }

        Ok(())
    }

    pub async fn list_invitation_members(
        ddb: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> Result<Vec<SpaceInvitationMemberResponse>> {
        let mut members: Vec<SpaceInvitationMemberResponse> = vec![];
        let mut bookmark = None::<String>;

        loop {
            let (responses, new_bookmark) = SpaceInvitationMember::query(
                ddb,
                space_pk.clone(),
                if let Some(b) = &bookmark {
                    SpaceInvitationMemberQueryOption::builder()
                        .sk("SPACE_INVITATION_MEMBER#".into())
                        .bookmark(b.clone())
                } else {
                    SpaceInvitationMemberQueryOption::builder()
                        .sk("SPACE_INVITATION_MEMBER#".into())
                },
            )
            .await?;

            for response in responses {
                let mut member: SpaceInvitationMemberResponse = response.into();

                let verification = SpaceEmailVerification::get(
                    ddb,
                    space_pk,
                    Some(EntityType::SpaceEmailVerification(member.email.clone())),
                )
                .await?;

                member.authorized = verification
                    .map(|v| v.authorized)
                    .unwrap_or(false);

                members.push(member);
            }

            match new_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        Ok(members)
    }
}
