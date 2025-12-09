use futures::future::try_join_all;
use serde_json::json;

use super::*;
use crate::features::spaces::members::{
    InvitationStatus, SpaceEmailVerification, SpaceInvitationMemberResponse,
};
use crate::models::{SpaceCommon, UserNotification};
use crate::services::fcm_notification::FCMService;
use crate::types::*;
use crate::utils::aws::{DynamoClient, SesClient};
use crate::*;
use aws_sdk_dynamodb::types::AttributeValue;

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
    #[dynamo(index = "gsi2", order = 1, sk)]
    #[dynamo(index = "gsi3", sk)]
    pub status: InvitationStatus,

    #[serde(default)]
    #[dynamo(index = "gsi2", order = 2, sk)]
    pub created_at: i64,
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
            created_at: now(),
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceInvitationMember(user_pk.to_string()),
        )
    }

    pub async fn find_user_invitations_by_status_latest(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        mut opt: SpaceInvitationMemberQueryOption,
    ) -> Result<(Vec<Self>, Option<String>)> {
        opt.scan_index_forward = false;
        SpaceInvitationMember::find_user_invitations_by_status(cli, user_pk, opt).await
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

        let emails: Vec<String> = responses
            .iter()
            .map(|member| member.email.clone())
            .collect();

        try_join_all(updates).await?;

        if !emails.is_empty() {
            let _ = SpaceEmailVerification::send_email(
                dynamo,
                ses,
                emails,
                space.clone(),
                title.clone(),
            )
            .await?;
        }

        Ok(())
    }

    pub async fn send_notification(
        dynamo: &DynamoClient,
        fcm: &mut FCMService,
        space: &SpaceCommon,
        space_title: String,
    ) -> Result<()> {
        tracing::info!(
            "SpaceInvitationMember::send_notification: start for space_pk={}",
            space.pk
        );

        let (invites, _) = SpaceInvitationMember::find_space_invitations_by_status(
            &dynamo.client,
            space.pk.clone(),
            SpaceInvitationMember::opt_all().sk(InvitationStatus::Invited.to_string()),
        )
        .await?;

        if invites.is_empty() {
            tracing::info!(
                "SpaceInvitationMember::send_notification: no invited members for space_pk={}",
                space.pk
            );
            return Ok(());
        }

        let title = "You are invited in space.".to_string();
        let body = format!("Participate new space: {space_title}");

        let user_pks: Vec<Partition> = invites.into_iter().map(|m| m.user_pk).collect();

        UserNotification::send_to_users(dynamo, fcm, &user_pks, title, body).await?;

        tracing::info!(
            "SpaceInvitationMember::send_notification: done for space_pk={}",
            space.pk
        );
        Ok(())
    }

    pub async fn list_invitation_members(
        dynamo: &DynamoClient,
        space_pk: &Partition,
    ) -> Result<Vec<SpaceInvitationMemberResponse>> {
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
                    SpaceInvitationMemberQueryOption::builder()
                        .sk("SPACE_INVITATION_MEMBER#".into())
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

        Ok(members)
    }
}
