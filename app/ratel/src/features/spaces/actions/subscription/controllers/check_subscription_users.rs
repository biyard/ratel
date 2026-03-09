use crate::features::spaces::actions::subscription::*;
use crate::features::posts::models::Team;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CheckSubscriptionUsersRequest {
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CheckSubscriptionUsersResponse {
    pub existing_identifiers: Vec<String>,
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

#[post("/api/spaces/{space_id}/subscriptions/users/check", role: SpaceUserRole)]
pub async fn check_subscription_users(
    space_id: SpacePartition,
    req: CheckSubscriptionUsersRequest,
) -> Result<CheckSubscriptionUsersResponse> {
    SpaceSubscription::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    use crate::features::auth::models::user::UserQueryOption;

    let mut existing_identifiers = vec![];
    let mut seen = std::collections::HashSet::<String>::new();

    for raw_identifier in req.identifiers {
        let Some((identifier, is_email)) = normalize_identifier(&raw_identifier) else {
            continue;
        };
        let de_key = if is_email {
            format!("email:{}", identifier)
        } else {
            format!("username:{}", identifier)
        };
        if !seen.insert(de_key) {
            continue;
        }

        if is_email {
            let (users, _) = crate::features::auth::User::find_by_email(
                cli,
                &identifier,
                UserQueryOption::builder().limit(1),
            )
            .await?;
            if users.first().is_some() {
                existing_identifiers.push(identifier);
            }
            continue;
        }

        let users = match crate::features::auth::User::find_by_username(
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
        };

        if users.first().is_some() {
            existing_identifiers.push(identifier);
            continue;
        }

        let (teams, _) =
            Team::find_by_username_prefix(cli, &identifier, Team::opt().limit(5)).await?;
        if teams.into_iter().any(|team| team.username == identifier) {
            existing_identifiers.push(identifier);
        }
    }

    Ok(CheckSubscriptionUsersResponse {
        existing_identifiers,
    })
}
