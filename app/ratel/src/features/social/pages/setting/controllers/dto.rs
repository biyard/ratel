use super::super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamResponse {
    pub id: TeamPartition,
    pub created_at: i64,
    pub updated_at: i64,
    pub nickname: String,
    pub username: String,
    pub profile_url: Option<String>,
    pub dao_address: Option<String>,
    pub html_contents: String,
    pub permissions: Option<i64>,
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub allow_invite: bool,
    #[serde(default)]
    pub allow_create_space: bool,
}

#[cfg(feature = "server")]
impl From<(crate::features::posts::models::Team, i64)> for TeamResponse {
    fn from((team, permissions): (crate::features::posts::models::Team, i64)) -> Self {
        Self {
            id: team.pk.into(),
            created_at: team.created_at,
            updated_at: team.updated_at,
            nickname: team.display_name,
            username: team.username,
            profile_url: Some(team.profile_url),
            dao_address: team.dao_address,
            html_contents: team.description,
            permissions: Some(permissions),
            thumbnail_url: team.thumbnail_url,
            allow_invite: team.allow_invite,
            allow_create_space: team.allow_create_space,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateTeamRequest {
    pub nickname: Option<String>,
    pub description: Option<String>,
    pub profile_url: Option<String>,
    pub dao_address: Option<String>,
    pub thumbnail_url: Option<String>,
    pub allow_invite: Option<bool>,
    pub allow_create_space: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DeleteTeamResponse {
    pub message: String,
    pub deleted_count: usize,
}
