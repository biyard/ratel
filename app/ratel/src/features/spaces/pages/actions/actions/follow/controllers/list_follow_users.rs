use crate::common::models::auth::UserFollow;
use crate::common::models::space::SpaceCommon;
use crate::features::posts::models::Team;
use crate::features::spaces::pages::actions::actions::follow::models::SpaceFollowUser;
use crate::features::spaces::pages::actions::actions::follow::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct FollowUserItem {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub description: String,
    pub user_type: UserType,
    pub subscribed: bool,
}

#[get(
    "/api/spaces/{space_id}/follows/users?bookmark",
    role: SpaceUserRole,
    user: crate::features::auth::OptionalUser
)]
pub async fn list_follow_users(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<FollowUserItem>> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let is_first_page = bookmark.is_none();
    let mut opt = SpaceFollowUser::opt()
        .sk(EntityType::SpaceSubscriptionUser(String::default()).to_string())
        .limit(10);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (users, bookmark) = SpaceFollowUser::query(cli, space_pk.clone(), opt).await?;
    let users: Vec<SpaceFollowUser> = users
        .into_iter()
        .filter(|u| u.user_pk != Partition::None)
        .collect();

    let space = SpaceCommon::get(cli, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;
    let (creator_pk, creator_profile) = {
        let mut creator_profile = (
            space.author_display_name.clone(),
            space.author_profile_url.clone(),
            space.author_username.clone(),
            String::new(),
        );
        let mut creator_pk = space.user_pk.clone();

        let mut resolved = false;
        if let Ok((teams, _)) =
            Team::find_by_username_prefix(cli, &space.author_username, Team::opt().limit(5)).await
        {
            if let Some(team) = teams
                .into_iter()
                .find(|team| team.username == space.author_username)
            {
                creator_pk = team.pk.clone();
                creator_profile = (
                    team.display_name,
                    team.profile_url,
                    team.username,
                    team.description,
                );
                resolved = true;
            }
        }

        if !resolved {
            let users = match crate::features::auth::User::find_by_username(
                cli,
                &space.author_username,
                crate::features::auth::UserQueryOption::builder().limit(1),
            )
            .await
            {
                Ok((users, _)) => users,
                Err(err) => {
                    error!("Failed to query user by username: {:?}", err);
                    vec![]
                }
            };
            if let Some(user) = users.into_iter().next() {
                creator_pk = user.pk.clone();
                creator_profile = (
                    user.display_name,
                    user.profile_url,
                    user.username,
                    user.description,
                );
            }
        }

        (creator_pk, creator_profile)
    };

    let viewer_pk = user.0.as_ref().map(|u| u.pk.clone());
    let subscribed_targets = if let Some(viewer_pk) = viewer_pk.as_ref() {
        let mut keys: Vec<(Partition, EntityType)> = users
            .iter()
            .map(|target| UserFollow::follower_keys(&target.user_pk, viewer_pk))
            .collect();
        if !users.iter().any(|u| u.user_pk == creator_pk) {
            keys.push(UserFollow::follower_keys(&creator_pk, viewer_pk));
        }

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

    let mut items: Vec<FollowUserItem> = users
        .into_iter()
        .map(|u| FollowUserItem {
            user_pk: u.user_pk.clone(),
            display_name: u.display_name,
            profile_url: u.profile_url,
            username: u.username,
            description: u.description,
            user_type: u.user_type,
            subscribed: subscribed_targets.contains(&u.user_pk.to_string()),
        })
        .collect();

    if let Some(creator) = items.iter_mut().find(|u| u.user_pk == creator_pk) {
        if creator.description.is_empty() {
            creator.description = creator_profile.3.clone();
        }
    }

    let creator_item = if let Some(idx) = items.iter().position(|u| u.user_pk == creator_pk) {
        let item = items.remove(idx);
        if is_first_page {
            Some(item)
        } else {
            None
        }
    } else {
        if is_first_page {
            let creator_sk = EntityType::SpaceSubscriptionUser(creator_pk.to_string());
            let creator = SpaceFollowUser::get(cli, &space_pk, Some(creator_sk))
                .await?
                .map(|u| FollowUserItem {
                    user_pk: u.user_pk.clone(),
                    display_name: u.display_name,
                    profile_url: u.profile_url,
                    username: u.username,
                    description: u.description,
                    user_type: u.user_type,
                    subscribed: subscribed_targets.contains(&u.user_pk.to_string()),
                });
            // Only hit the User table when creator_pk is actually a User
            // partition. If it's a Team (team-owned space), calling
            // `User::get(TEAM#..., "USER")` triggers the DynamoEntity macro's
            // `begins_with` fallback, which then matches *any* sk starting
            // with "USER" under that team's pk (USER_TEAM, USER_ESSENCE_STATS,
            // USER_EVM_ADDRESS, etc.) and tries to deserialize that unrelated
            // row as a User — which blows up with missing required fields
            // (e.g. "missing field `created_at`"). For Teams we always fall
            // through to the creator_profile fallback below.
            let user_lookup = if matches!(&creator_pk, Partition::User(_)) {
                crate::features::auth::User::get(cli, creator_pk.clone(), Some(EntityType::User))
                    .await?
            } else {
                None
            };
            if creator.is_some() {
                creator
            } else if let Some(user) = user_lookup
            {
                Some(FollowUserItem {
                    user_pk: creator_pk.clone(),
                    display_name: user.display_name,
                    profile_url: user.profile_url,
                    username: user.username,
                    description: user.description,
                    user_type: user.user_type,
                    subscribed: subscribed_targets.contains(&creator_pk.to_string()),
                })
            } else {
                Some(FollowUserItem {
                    user_pk: creator_pk.clone(),
                    display_name: creator_profile.0,
                    profile_url: creator_profile.1,
                    username: creator_profile.2,
                    description: creator_profile.3,
                    user_type: match creator_pk {
                        Partition::Team(_) => UserType::Team,
                        _ => UserType::Individual,
                    },
                    subscribed: subscribed_targets.contains(&creator_pk.to_string()),
                })
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
