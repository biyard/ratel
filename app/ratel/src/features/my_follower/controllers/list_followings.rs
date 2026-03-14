use crate::features::my_follower::controllers::dto::FollowUserItem;
use crate::features::my_follower::controllers::profile::build_profiles;
use crate::features::my_follower::*;
use crate::common::models::auth::UserFollow;

#[get("/api/my-follower/followings?bookmark", user: crate::features::auth::User)]
pub async fn list_followings(bookmark: Option<String>) -> Result<ListResponse<FollowUserItem>> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let mut opt = UserFollow::opt()
        .sk(EntityType::Following(String::default()).to_string())
        .limit(10);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (follows, bookmark) = UserFollow::query(cli, user.pk.clone(), opt).await?;
    let target_pks: Vec<Partition> = follows
        .into_iter()
        .map(|follow| follow.target_user_pk)
        .collect();

    let profiles = build_profiles(cli, &target_pks).await?;

    let items = target_pks
        .into_iter()
        .filter_map(|pk| {
            let key = pk.to_string();
            profiles.get(&key).map(|profile| FollowUserItem {
                user_pk: pk,
                display_name: profile.display_name.clone(),
                profile_url: profile.profile_url.clone(),
                username: profile.username.clone(),
                user_type: profile.user_type,
                description: profile.description.clone(),
                is_following: true,
            })
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
