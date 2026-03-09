use super::*;
use crate::features::membership::*;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::{Membership, MembershipTier, UserMembership, UserMembershipResponse};
use crate::features::auth::User;

#[get("/v3/me/memberships", user: User)]
pub async fn get_membership_handler() -> Result<UserMembershipResponse> {
    let result = async {
        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();

        let user_membership =
            UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership)).await?;

        let (user_membership, _membership) = match user_membership {
            Some(user_membership) => {
                let membership_pk: Partition = user_membership.membership_pk.clone().into();
                let membership = Membership::get(cli, membership_pk, Some(EntityType::Membership))
                    .await?
                    .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;
                (user_membership, membership)
            }
            None => {
                let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
                let free_membership = Membership::get(
                    cli,
                    free_membership_pk.clone(),
                    Some(EntityType::Membership),
                )
                .await?
                .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;

                let user_membership = UserMembership::new(
                    user.pk.clone().into(),
                    free_membership.pk.clone().into(),
                    free_membership.duration_days,
                    free_membership.credits,
                )?;
                user_membership.create(cli).await?;

                (user_membership, free_membership)
            }
        };

        Ok(user_membership.into())
    }
    .await;

    result.map_err(normalize_error)
}
