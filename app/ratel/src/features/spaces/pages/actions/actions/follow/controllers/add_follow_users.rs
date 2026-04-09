use crate::features::posts::models::Team;
use crate::features::spaces::pages::actions::actions::follow::models::SpaceFollowUser;
use crate::features::spaces::pages::actions::actions::follow::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddFollowUsersRequest {
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddFollowUsersResponse {
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

#[post("/api/spaces/{space_id}/follows/users", role: SpaceUserRole)]
pub async fn add_follow_users(
    space_id: SpacePartition,
    req: AddFollowUsersRequest,
) -> Result<AddFollowUsersResponse> {
    SpaceFollowAction::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    use crate::features::auth::models::user::UserQueryOption;

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
            crate::features::auth::User::find_by_email(
                cli,
                &identifier,
                UserQueryOption::builder().limit(1),
            )
            .await?
            .0
        } else {
            match crate::features::auth::User::find_by_username(
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
        let follow_user = if let Some(target_user) = target_user {
            SpaceFollowUser::new(
                space_id.clone(),
                target_user.pk,
                target_user.display_name,
                target_user.profile_url,
                target_user.username,
                target_user.user_type,
                target_user.description,
            )
        } else if !is_email {
            let (teams, _) =
                Team::find_by_username_prefix(cli, &identifier, Team::opt().limit(5)).await?;
            let target_team = teams.into_iter().find(|team| team.username == identifier);
            if let Some(team) = target_team {
                SpaceFollowUser::new(
                    space_id.clone(),
                    team.pk,
                    team.display_name,
                    team.profile_url,
                    team.username,
                    UserType::Team,
                    team.description,
                )
            } else {
                return Err(Error::NotFound(format!("User not found: {}", identifier)));
            }
        } else {
            return Err(Error::NotFound(format!("User not found: {}", identifier)));
        };

        let (pk, sk) = SpaceFollowUser::keys(&space_id, &follow_user.user_pk);
        if SpaceFollowUser::get(cli, &pk, Some(sk)).await?.is_some() {
            continue;
        }

        follow_user.upsert(cli).await?;
        added_identifiers.push(identifier);
    }

    Ok(AddFollowUsersResponse { added_identifiers })
}
