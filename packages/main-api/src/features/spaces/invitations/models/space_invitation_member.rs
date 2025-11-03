use crate::features::spaces::invitations::{SpaceEmailVerification, SpaceInvitationMemberResponse};
use crate::types::*;
use crate::utils::aws::DynamoClient;
use crate::{Error, User};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceInvitationMember {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
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
        }
    }

    pub async fn list_invitation_members(
        dynamo: &DynamoClient,
        space_pk: &Partition,
    ) -> Result<Vec<SpaceInvitationMemberResponse>, Error> {
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
