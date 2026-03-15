use crate::common::models::auth::UserFollow;
use crate::features::my_follower::*;

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

#[get("/api/my-follower/status?target_username", user: crate::features::auth::User)]
pub async fn check_follow_status_handler(
    target_username: String,
) -> Result<FollowStatusResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let (users, _) = crate::features::auth::User::find_by_username(
        cli,
        &target_username,
        Default::default(),
    )
    .await?;

    let target = users
        .into_iter()
        .find(|u| u.username == target_username)
        .ok_or(Error::NotFound(format!(
            "User not found: {}",
            target_username
        )))?;

    let (follower_pk, follower_sk) = UserFollow::follower_keys(&target.pk, &user.pk);
    let is_following = UserFollow::get(cli, follower_pk, Some(follower_sk))
        .await?
        .is_some();

    Ok(FollowStatusResponse {
        target_pk: target.pk,
        target_display_name: target.display_name,
        target_profile_url: target.profile_url,
        target_description: target.description,
        target_followers_count: target.followers_count,
        target_followings_count: target.followings_count,
        is_following,
    })
}
