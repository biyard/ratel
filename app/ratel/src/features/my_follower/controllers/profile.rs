use crate::features::my_follower::controllers::dto::FollowUserProfile;
use crate::features::my_follower::*;
use crate::features::posts::models::Team;
use std::collections::HashMap;

#[cfg(feature = "server")]
pub async fn build_profiles(
    cli: &aws_sdk_dynamodb::Client,
    target_pks: &[Partition],
) -> Result<HashMap<String, FollowUserProfile>> {
    let mut user_keys = Vec::new();
    let mut team_keys = Vec::new();

    for pk in target_pks {
        match pk {
            Partition::Team(_) => team_keys.push((pk.clone(), EntityType::Team)),
            _ => user_keys.push((pk.clone(), EntityType::User)),
        }
    }

    let users: Vec<crate::features::auth::User> = if user_keys.is_empty() {
        vec![]
    } else {
        crate::features::auth::User::batch_get(cli, user_keys).await?
    };
    let teams: Vec<Team> = if team_keys.is_empty() {
        vec![]
    } else {
        Team::batch_get(cli, team_keys).await?
    };

    let mut profiles = HashMap::new();
    for user in users {
        profiles.insert(
            user.pk.to_string(),
            FollowUserProfile {
                display_name: user.display_name,
                profile_url: user.profile_url,
                username: user.username,
                user_type: user.user_type,
                description: user.description,
            },
        );
    }
    for team in teams {
        profiles.insert(
            team.pk.to_string(),
            FollowUserProfile {
                display_name: team.display_name,
                profile_url: team.profile_url,
                username: team.username,
                user_type: UserType::Team,
                description: team.description,
            },
        );
    }

    Ok(profiles)
}

#[cfg(not(feature = "server"))]
pub async fn build_profiles(
    _cli: &(),
    _target_pks: &[Partition],
) -> Result<HashMap<String, FollowUserProfile>> {
    Ok(HashMap::new())
}
