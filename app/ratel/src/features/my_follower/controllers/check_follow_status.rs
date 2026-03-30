use crate::common::models::auth::UserFollow;
use crate::features::auth::OptionalUser;
use crate::features::my_follower::*;
#[cfg(feature = "server")]
use crate::features::posts::models::Team;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct FollowStatusResponse {
    pub target_pk: Partition,
    pub target_display_name: String,
    pub target_profile_url: String,
    pub target_description: String,
    pub target_followers_count: i64,
    pub target_followings_count: i64,
    pub is_following: bool,
}

#[get("/api/my-follower/status?target_username", user: OptionalUser)]
pub async fn check_follow_status_handler(
    target_username: String,
) -> Result<FollowStatusResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let user: Option<crate::features::auth::User> = user.into();

    // Try user lookup first; treat any lookup failure as "no user found"
    // (Error::Aws is not Serialize and would cause an SSR panic if propagated)
    let user_lookup = match crate::features::auth::User::find_by_username(
        cli,
        &target_username,
        Default::default(),
    )
    .await
    {
        Ok((users, _)) => users.into_iter().find(|u| u.username == target_username),
        Err(_) => None,
    };

    let (target_pk, display_name, profile_url, description, followers_count, followings_count) =
        if let Some(u) = user_lookup {
            (u.pk, u.display_name, u.profile_url, u.description, u.followers_count, u.followings_count)
        } else {
            // Fallback to team lookup
            let gsi2_sk_prefix = Team::compose_gsi2_sk(String::default());
            let team_query_option = Team::opt().sk(gsi2_sk_prefix);
            let (teams, _) = Team::find_by_username_prefix(cli, target_username.clone(), team_query_option).await?;
            let team = teams
                .into_iter()
                .find(|t| t.username == target_username)
                .ok_or(Error::NotFound(format!("Not found: {}", target_username)))?;
            (team.pk, team.display_name, team.profile_url, team.description, team.followers, 0i64)
        };

    let is_following = if let Some(ref user) = user {
        let (follower_pk, follower_sk) = UserFollow::follower_keys(&target_pk, &user.pk);
        UserFollow::get(cli, follower_pk, Some(follower_sk))
            .await?
            .is_some()
    } else {
        false
    };

    Ok(FollowStatusResponse {
        target_pk,
        target_display_name: display_name,
        target_profile_url: profile_url,
        target_description: description,
        target_followers_count: followers_count,
        target_followings_count: followings_count,
        is_following,
    })
}
