use crate::auth::UserTeam;
use crate::features::social::*;
use crate::social::pages::member::dto::TeamRole;
use crate::{auth::User, posts::models::Team, *};

#[get("/api/social/:username", user: OptionalUser)]
pub async fn get_wall_by_username(username: String) -> Result<Wall> {
    use crate::auth::UserFollow;

    let cli = config::get().dynamodb();
    let users = WallUser::find_by_username(cli, username).await?;

    if users.is_empty() {
        Err(SocialError::InvalidUserName)?;
    }

    let user = user.0;

    let is_following = if let Some(u) = &user {
        let target_pk = match &users[0] {
            WallUser::User(u) => &u.pk,
            WallUser::Team(t) => &t.pk,
        };
        let (follower_pk, follower_sk) = UserFollow::follower_keys(target_pk, &u.pk);
        UserFollow::get(cli, follower_pk, Some(follower_sk))
            .await?
            .is_some()
    } else {
        false
    };

    let mut wall = users[0].clone().into();

    match &mut wall {
        Wall::User {
            id,
            can_edit,
            following,
            ..
        } => {
            *following = is_following;
            if let Some(u) = user {
                let pk_id: UserPartition = u.pk.into();
                if &pk_id == id {
                    *can_edit = true;
                }
            }
        }
        Wall::Team {
            id,
            role,
            following,
            ..
        } => {
            *following = is_following;
            if let Some(u) = user {
                // Check if the user is a member of the team.

                let team_pk: Partition = id.clone().into();

                let member =
                    UserTeam::get(cli, &u.pk, Some(EntityType::UserTeam(team_pk.to_string())))
                        .await?;
                if let Some(m) = member {
                    *role = Some(m.role);
                }
            }
        }
    }

    Ok(wall)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "user_type")]
pub enum Wall {
    User {
        id: UserPartition,
        display_name: String,
        username: String,
        profile_url: String,
        description: String,
        can_edit: bool,
        following: bool,
    },
    Team {
        id: TeamPartition,
        display_name: String,
        username: String,
        profile_url: String,
        description: String,
        role: Option<TeamRole>,
        following: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
#[serde(untagged)]
#[dynamo(pk_prefix = "USERNAME", index = "gsi2", name = "find_by_username")]
pub enum WallUser {
    User(User),
    Team(Team),
}

impl Into<Wall> for WallUser {
    fn into(self) -> Wall {
        match self {
            WallUser::User(u) => Wall::User {
                id: u.pk.into(),
                display_name: u.display_name,
                username: u.username,
                profile_url: u.profile_url,
                description: u.description,
                can_edit: false,
                following: false,
            },
            WallUser::Team(t) => Wall::Team {
                id: t.pk.into(),
                display_name: t.display_name,
                username: t.username,
                profile_url: t.profile_url,
                description: t.description,
                role: None,
                following: false,
            },
        }
    }
}
