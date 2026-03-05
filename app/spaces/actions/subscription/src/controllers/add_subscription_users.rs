use crate::models::SpaceSubscriptionUser;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddSubscriptionUsersRequest {
    pub emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddSubscriptionUsersResponse {
    pub added_emails: Vec<String>,
}

#[cfg(feature = "server")]
fn normalize_email(raw: &str) -> Option<String> {
    let email = raw.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return None;
    }
    Some(email)
}

#[post("/api/spaces/{space_id}/subscriptions/users", role: SpaceUserRole)]
pub async fn add_subscription_users(
    space_id: SpacePartition,
    req: AddSubscriptionUsersRequest,
) -> Result<AddSubscriptionUsersResponse> {
    SpaceSubscription::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    use ratel_auth::models::user::UserQueryOption;

    let mut added_emails = vec![];
    let mut seen = std::collections::HashSet::<String>::new();

    for raw_email in req.emails {
        let email = normalize_email(&raw_email)
            .ok_or_else(|| Error::BadRequest(format!("Invalid email: {}", raw_email)))?;
        if !seen.insert(email.clone()) {
            continue;
        }

        let (users, _) =
            ratel_auth::User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
                .await?;
        let target_user = users
            .into_iter()
            .next()
            .ok_or_else(|| Error::NotFound(format!("User not found: {}", email)))?;

        let (pk, sk) = SpaceSubscriptionUser::keys(&space_id, &target_user.pk);
        if SpaceSubscriptionUser::get(cli, &pk, Some(sk))
            .await?
            .is_some()
        {
            continue;
        }

        let subscription_user = SpaceSubscriptionUser::new(space_id.clone(), &target_user);
        subscription_user.upsert(cli).await?;
        added_emails.push(email);
    }

    Ok(AddSubscriptionUsersResponse { added_emails })
}
