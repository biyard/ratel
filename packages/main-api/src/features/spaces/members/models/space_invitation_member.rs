use futures::future::try_join_all;

use super::*;
use crate::features::spaces::members::{
    InvitationStatus, SpaceEmailVerification, SpaceInvitationMemberResponse,
};
use crate::models::SpaceCommon;
use crate::types::*;
use crate::utils::aws::{DynamoClient, SesClient};
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
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
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi3", sk)]
    pub status: InvitationStatus,
}

impl SpaceInvitationMember {
    pub fn new(
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
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceInvitationMember(user_pk.to_string()),
        )
    }

    pub async fn send_email(
        dynamo: &DynamoClient,
        ses: &SesClient,
        space: &SpaceCommon,
        title: String,
    ) -> Result<()> {
        let (responses, _) = SpaceInvitationMember::find_space_invitations_by_status(
            &dynamo.client,
            space.pk.clone(),
            SpaceInvitationMember::opt_all().sk(InvitationStatus::Pending.to_string()),
        )
        .await?;

        let updates = responses.iter().map(|member| {
            let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &member.user_pk);
            SpaceInvitationMember::updater(pk, sk)
                .with_status(InvitationStatus::Invited)
                .execute(&dynamo.client)
        });

        let futs = responses.clone().into_iter().map(|member| {
            SpaceEmailVerification::send_email(
                &dynamo,
                &ses,
                member.email,
                space.clone(),
                title.clone(),
            )
        });

        try_join_all(updates).await?;
        try_join_all(futs).await?;

        Ok(())
    }

    pub async fn list_invitation_members(
        dynamo: &DynamoClient,
        space_pk: &Partition,
    ) -> Result<Vec<SpaceInvitationMemberResponse>> {
        let mut members: Vec<SpaceInvitationMemberResponse> = vec![];
        let mut bookmark = None::<String>;

        loop {
            tracing::debug!("1111");
            let (responses, new_bookmark) = SpaceInvitationMember::query(
                &dynamo.client,
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
            tracing::debug!("11112");

            for response in responses {
                tracing::debug!("111123");
                let mut member: SpaceInvitationMemberResponse = response.into();
                tracing::debug!("1111234");

                let verification = SpaceEmailVerification::get(
                    &dynamo.client,
                    &space_pk,
                    Some(EntityType::SpaceEmailVerification(member.email.clone())),
                )
                .await?;
                tracing::debug!("11112345");

                if verification.is_some() && verification.unwrap_or_default().authorized {
                    member.authorized = true;
                } else {
                    member.authorized = false;
                }
                tracing::debug!("111123456");

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
