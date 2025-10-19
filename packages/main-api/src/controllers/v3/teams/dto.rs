use bdk::prelude::*;

use crate::types::*;
use crate::{models::team::*, types::UserType};
use axum::extract::Path;

pub type TeamPath = Path<TeamPathParam>;

#[derive(Debug, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct TeamPathParam {
    pub team_pk: Partition,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TeamResponse {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,

    pub nickname: String,
    pub username: String,
    pub profile_url: Option<String>,
    pub user_type: u8,

    pub html_contents: String,
}

impl From<Team> for TeamResponse {
    fn from(team: Team) -> Self {
        Self {
            id: team.pk.to_string(),
            created_at: team.created_at,
            updated_at: team.updated_at,
            nickname: team.display_name,
            username: team.username,
            profile_url: Some(team.profile_url),
            user_type: UserType::Team as u8,
            html_contents: team.description,
        }
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TeamGroupResponse {
    pub id: String, // Just the UUID part, not the full EntityType
    pub name: String,
    pub description: String,
    pub members: i64,
    pub permissions: i64,
}

impl From<TeamGroup> for TeamGroupResponse {
    fn from(group: TeamGroup) -> Self {
        // Extract UUID from EntityType::TeamGroup(uuid)
        let group_id = match group.sk {
            EntityType::TeamGroup(uuid) => uuid,
            _ => group.sk.to_string(), // Fallback
        };

        Self {
            id: group_id,
            name: group.name,
            description: group.description,
            members: group.members,
            permissions: group.permissions,
        }
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TeamOwnerResponse {
    pub id: String, // Just the UUID part, not the full Partition
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<TeamOwner> for TeamOwnerResponse {
    fn from(owner: TeamOwner) -> Self {
        // Extract UUID from Partition::User(uuid)
        let user_id = match owner.user_pk {
            Partition::User(uuid) => uuid,
            _ => owner.user_pk.to_string(), // Fallback
        };

        Self {
            id: user_id,
            display_name: owner.display_name,
            profile_url: owner.profile_url,
            username: owner.username,
        }
    }
}
#[derive(Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TeamDetailResponse {
    #[serde(flatten)]
    pub team: TeamResponse,
    pub groups: Option<Vec<TeamGroupResponse>>,
    pub owner: Option<TeamOwnerResponse>,
    /// User's permissions bitmask for this team (i64)
    pub permissions: Option<i64>,
}

impl From<Vec<TeamMetadata>> for TeamDetailResponse {
    fn from(items: Vec<TeamMetadata>) -> Self {
        let mut res = Self::default();
        for item in items {
            match item {
                TeamMetadata::Team(user) => {
                    res.team = user.into();
                }
                TeamMetadata::TeamGroup(group) => {
                    if let Some(groups) = &mut res.groups {
                        groups.push(group.into());
                    } else {
                        res.groups = Some(vec![group.into()]);
                    }
                }
                TeamMetadata::TeamOwner(owner) => {
                    res.owner = Some(owner.into());
                }
            }
        }
        res
    }
}
