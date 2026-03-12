use crate::features::auth::*;

#[cfg(feature = "membership")]
use crate::features::membership::models::{
    Membership, MembershipTier, UserMembership, UserMembershipResponse,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct GetMeResponse {
    pub user: Option<User>,
    #[cfg(feature = "membership")]
    pub membership: Option<UserMembershipResponse>,
}

#[get("/api/auth/me", user: OptionalUser)]
pub async fn get_me_handler() -> Result<GetMeResponse> {
    let user: Option<User> = user.into();

    #[cfg(feature = "membership")]
    let membership = {
        match &user {
            Some(u) => {
                let conf = crate::config::get();
                let cli = conf.dynamodb();
                match UserMembership::get(cli, u.pk.clone(), Some(EntityType::UserMembership))
                    .await
                {
                    Ok(Some(um)) => {
                        let mut resp: UserMembershipResponse = um.clone().into();
                        let membership_pk: Partition = um.membership_pk.clone().into();
                        if let Ok(Some(m)) = Membership::get(
                            cli,
                            membership_pk,
                            Some(EntityType::Membership),
                        )
                        .await
                        {
                            resp.max_credits_per_space = m.max_credits_per_space;
                        }
                        Some(resp)
                    }
                    Ok(None) => {
                        // Create a default Free membership if none exists
                        let free_tier = MembershipTier::Free;
                        let free_pk = Partition::Membership(free_tier.to_string());
                        match Membership::get(cli, free_pk, Some(EntityType::Membership)).await {
                            Ok(Some(free_membership)) => {
                                let um = UserMembership::new(
                                    u.pk.clone().into(),
                                    free_membership.pk.clone().into(),
                                    free_membership.duration_days,
                                    free_membership.credits,
                                )
                                .ok();
                                if let Some(ref um) = um {
                                    let _ = um.create(cli).await;
                                }
                                um.map(|m| {
                                    let mut resp: UserMembershipResponse = m.into();
                                    resp.max_credits_per_space =
                                        free_membership.max_credits_per_space;
                                    resp
                                })
                            }
                            _ => None,
                        }
                    }
                    Err(_) => None,
                }
            }
            None => None,
        }
    };

    Ok(GetMeResponse {
        user,
        #[cfg(feature = "membership")]
        membership,
    })
}
