use crate::features::social::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserResponse {
    pub pk: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub description: String,
}

#[cfg(feature = "server")]
impl From<crate::features::auth::User> for UserResponse {
    fn from(user: crate::features::auth::User) -> Self {
        Self {
            pk: user.pk.to_string(),
            username: user.username,
            display_name: user.display_name,
            profile_url: user.profile_url,
            description: user.description,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamResponse {
    pub pk: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub nickname: String,
    pub username: String,
    pub profile_url: Option<String>,
    pub dao_address: Option<String>,
    pub html_contents: String,
    pub thumbnail_url: Option<String>,
    pub permissions: Option<Vec<crate::features::posts::types::TeamGroupPermission>>,
}

#[cfg(feature = "server")]
impl From<(crate::features::posts::models::Team, i64)> for TeamResponse {
    fn from((team, permissions): (crate::features::posts::models::Team, i64)) -> Self {
        let perms: crate::features::posts::types::TeamGroupPermissions = permissions.into();
        Self {
            pk: team.pk.to_string(),
            created_at: team.created_at,
            updated_at: team.updated_at,
            nickname: team.display_name,
            username: team.username,
            profile_url: Some(team.profile_url),
            dao_address: team.dao_address,
            html_contents: team.description,
            thumbnail_url: team.thumbnail_url,
            permissions: Some(perms.0),
        }
    }
}
