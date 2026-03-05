use crate::controllers::dto::FollowUserItem;
use crate::controllers::profile::build_profiles;
use crate::*;
use common::models::auth::UserFollow;
use std::collections::HashSet;

#[get("/api/my-follower/followers?bookmark", user: ratel_auth::User)]
pub async fn list_followers(bookmark: Option<String>) -> Result<ListResponse<FollowUserItem>> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let mut opt = UserFollow::opt()
        .sk(EntityType::Follower(String::default()).to_string())
        .limit(10);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (follows, bookmark) = UserFollow::query(cli, user.pk.clone(), opt).await?;
    let follower_pks: Vec<Partition> = follows.into_iter().map(|follow| follow.user_pk).collect();

    let profiles = build_profiles(cli, &follower_pks).await?;

    let following_keys: Vec<(Partition, EntityType)> = follower_pks
        .iter()
        .map(|pk| UserFollow::following_keys(&user.pk, pk))
        .collect();
    let following_records: Vec<UserFollow> = if following_keys.is_empty() {
        vec![]
    } else {
        UserFollow::batch_get(cli, following_keys).await?
    };
    let following_targets: HashSet<String> = following_records
        .into_iter()
        .map(|record| record.target_user_pk.to_string())
        .collect();

    let items = follower_pks
        .into_iter()
        .filter_map(|pk| {
            let key = pk.to_string();
            profiles.get(&key).map(|profile| FollowUserItem {
                user_pk: pk.clone(),
                display_name: profile.display_name.clone(),
                profile_url: profile.profile_url.clone(),
                username: profile.username.clone(),
                user_type: profile.user_type,
                description: profile.description.clone(),
                is_following: following_targets.contains(&pk.to_string()),
            })
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
