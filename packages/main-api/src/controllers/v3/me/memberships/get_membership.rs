use crate::features::membership::*;
use super::*;

pub type GetMembershipResponse = UserMembershipResponse;

pub async fn get_membership_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
) -> Result<Json<GetMembershipResponse>> {
    let cli = &dynamo.client;

    // Try to get existing membership, or create a Free one if it doesn't exist
    let (user_membership, _) = match user.get_membership(cli).await {
        Ok(result) => result,
        Err(Error::NoUserMembershipFound) | Err(Error::NoMembershipFound) => {
            // User doesn't have a membership yet, create a Free membership
            let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
            let free_membership = Membership::get(
                cli,
                free_membership_pk.clone(),
                Some(EntityType::Membership),
            )
            .await?
            .ok_or_else(|| Error::NoMembershipFound)?;

            let user_membership = UserMembership::new(
                user.pk.clone().into(),
                free_membership.pk.clone().into(),
                free_membership.duration_days,
                free_membership.credits,
            )?;
            user_membership.create(cli).await?;

            (user_membership, free_membership)
        }
        Err(e) => return Err(e),
    };

    Ok(Json(user_membership.into()))
}
