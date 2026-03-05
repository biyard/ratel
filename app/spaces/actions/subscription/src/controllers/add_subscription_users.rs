use crate::models::SpaceSubscriptionUser;
use crate::*;
use ratel_post::models::Team;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddSubscriptionUsersRequest {
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddSubscriptionUsersResponse {
    pub added_identifiers: Vec<String>,
}

#[cfg(feature = "server")]
fn normalize_identifier(raw: &str) -> Option<(String, bool)> {
    let value = raw.trim().trim_start_matches('@').to_ascii_lowercase();
    if value.is_empty() {
        return None;
    }
    let is_email = value.contains('@');
    Some((value, is_email))
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

    let mut added_identifiers = vec![];
    let mut seen = std::collections::HashSet::<String>::new();

    for raw_identifier in req.identifiers {
        let (identifier, is_email) = normalize_identifier(&raw_identifier)
            .ok_or_else(|| Error::BadRequest(format!("Invalid identifier: {}", raw_identifier)))?;
        let de_key = if is_email {
            format!("email:{}", identifier)
        } else {
            format!("username:{}", identifier)
        };
        if !seen.insert(de_key) {
            continue;
        }

        let users = if is_email {
            ratel_auth::User::find_by_email(cli, &identifier, UserQueryOption::builder().limit(1))
                .await?
                .0
        } else {
            match ratel_auth::User::find_by_username(
                cli,
                &identifier,
                UserQueryOption::builder().limit(1),
            )
            .await
            {
                Ok((users, _)) => users,
                Err(err) => {
                    error!("Failed to query user by username: {:?}", err);
                    vec![]
                }
            }
        };
        let target_user = users.into_iter().next();
        let subscription_user = if let Some(target_user) = target_user {
            SpaceSubscriptionUser::new(
                space_id.clone(),
                target_user.pk,
                target_user.display_name,
                target_user.profile_url,
                target_user.username,
                target_user.user_type,
            )
        } else if !is_email {
            let (teams, _) =
                Team::find_by_username_prefix(cli, &identifier, Team::opt().limit(5)).await?;
            let target_team = teams.into_iter().find(|team| team.username == identifier);
            if let Some(team) = target_team {
                SpaceSubscriptionUser::new(
                    space_id.clone(),
                    team.pk,
                    team.display_name,
                    team.profile_url,
                    team.username,
                    UserType::Team,
                )
            } else {
                return Err(Error::NotFound(format!("User not found: {}", identifier)));
            }
        } else {
            return Err(Error::NotFound(format!("User not found: {}", identifier)));
        };

        let (pk, sk) = SpaceSubscriptionUser::keys(&space_id, &subscription_user.user_pk);
        if SpaceSubscriptionUser::get(cli, &pk, Some(sk))
            .await?
            .is_some()
        {
            continue;
        }

        subscription_user.upsert(cli).await?;
        added_identifiers.push(identifier);
    }

    Ok(AddSubscriptionUsersResponse { added_identifiers })
}
