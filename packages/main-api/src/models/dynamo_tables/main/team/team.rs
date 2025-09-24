use crate::{
    models::team::{TeamGroup, TeamMetadata, TeamOwner},
    types::*,
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct Team {
    pub pk: Partition,
    #[dynamo(
        prefix = "TEAM_NAME_IDX",
        name = "find_by_name_prefix",
        index = "gsi1",
        pk
    )]
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    // TODO: When chaning display_name, profile_url and username, UserTeam items should be updated as well.
    #[dynamo(index = "gsi1", sk)]
    pub display_name: String,
    pub profile_url: String,
    // NOTE: username is linked with gsi2-index of user model.
    #[dynamo(
        prefix = "USERNAME",
        name = "find_by_username_prefix",
        index = "gsi2",
        pk
    )]
    pub username: String, // Team Name

    pub description: String,
}

impl Team {
    pub fn new(
        display_name: String,
        profile_url: String,
        username: String,
        description: String,
    ) -> Self {
        let team_id = uuid::Uuid::new_v4().to_string();
        let pk = Partition::Team(team_id);
        let sk = EntityType::Team;

        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            display_name,
            profile_url,
            username,
            description,
        }
    }
}

#[derive(Debug, Default, serde::Serialize, schemars::JsonSchema)]
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
#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct TeamGroupResponse {
    pub sk: String,
    pub name: String,
    pub description: String,
    pub members: i64,
    pub permissions: i64,
}

impl From<TeamGroup> for TeamGroupResponse {
    fn from(group: TeamGroup) -> Self {
        Self {
            sk: group.sk.to_string(),
            name: group.name,
            description: group.description,
            members: group.members,
            permissions: group.permissions,
        }
    }
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct TeamOwnerResponse {
    pub user_pk: String,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<TeamOwner> for TeamOwnerResponse {
    fn from(owner: TeamOwner) -> Self {
        Self {
            user_pk: owner.user_pk.to_string(),
            display_name: owner.display_name,
            profile_url: owner.profile_url,
            username: owner.username,
        }
    }
}
#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct TeamDetailResponse {
    #[serde(flatten)]
    pub team: TeamResponse,
    pub groups: Option<Vec<TeamGroupResponse>>,
    // pub owner: TeamOwner,
    pub owner: Option<TeamOwnerResponse>,
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
