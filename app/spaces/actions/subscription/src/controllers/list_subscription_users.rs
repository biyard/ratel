use crate::models::SpaceSubscriptionUser;
use crate::*;
use common::models::auth::UserFollow;
use common::models::space::SpaceCommon;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubscriptionUserItem {
    pub user_pk: UserPartition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub subscribed: bool,
}

#[get(
    "/api/spaces/{space_id}/subscriptions/users?bookmark",
    role: SpaceUserRole,
    user: ratel_auth::OptionalUser
)]
pub async fn list_subscription_users(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<SubscriptionUserItem>> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let is_first_page = bookmark.is_none();
    let mut opt = SpaceSubscriptionUser::opt()
        .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
        .limit(10);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (users, bookmark) = SpaceSubscriptionUser::query(cli, space_pk.clone(), opt).await?;
    let users: Vec<SpaceSubscriptionUser> = users
        .into_iter()
        .filter(|u| u.user_pk != Partition::None)
        .collect();

    let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;
    let creator_pk = space.user_pk.clone();

    let viewer_pk = user.0.as_ref().map(|u| u.pk.clone());
    let subscribed_targets = if let Some(viewer_pk) = viewer_pk.as_ref() {
        let keys: Vec<(Partition, EntityType)> = users
            .iter()
            .map(|target| UserFollow::follower_keys(&target.user_pk, viewer_pk))
            .collect();

        let subs: Vec<UserFollow> = if keys.is_empty() {
            vec![]
        } else {
            UserFollow::batch_get(cli, keys).await?
        };

        subs.into_iter()
            .map(|sub| sub.target_user_pk.to_string())
            .collect::<HashSet<String>>()
    } else {
        HashSet::new()
    };

    let mut items: Vec<SubscriptionUserItem> = users
        .into_iter()
        .map(|u| SubscriptionUserItem {
            user_pk: u.user_pk.clone().into(),
            display_name: u.display_name,
            profile_url: u.profile_url,
            username: u.username,
            user_type: u.user_type,
            subscribed: subscribed_targets.contains(&u.user_pk.to_string()),
        })
        .collect();

    let creator_item = if let Some(idx) = items.iter().position(|u| {
        let pk: Partition = u.user_pk.clone().into();
        pk == creator_pk
    }) {
        let item = items.remove(idx);
        if is_first_page { Some(item) } else { None }
    } else {
        if is_first_page {
            let creator_sk = EntityType::SpaceSubscriptionUser(creator_pk.to_string());
            let creator = SpaceSubscriptionUser::get(cli, &space_pk, Some(creator_sk))
                .await?
                .map(|u| SubscriptionUserItem {
                    user_pk: u.user_pk.clone().into(),
                    display_name: u.display_name,
                    profile_url: u.profile_url,
                    username: u.username,
                    user_type: u.user_type,
                    subscribed: subscribed_targets.contains(&u.user_pk.to_string()),
                });
            if creator.is_some() {
                creator
            } else if let Some(user) =
                ratel_auth::User::get(cli, creator_pk.clone(), Some(EntityType::User)).await?
            {
                Some(SubscriptionUserItem {
                    user_pk: creator_pk.clone().into(),
                    display_name: user.display_name,
                    profile_url: user.profile_url,
                    username: user.username,
                    user_type: user.user_type,
                    subscribed: subscribed_targets.contains(&creator_pk.to_string()),
                })
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(creator_item) = creator_item {
        items.insert(0, creator_item);
    }

    Ok(ListResponse { items, bookmark })
}
