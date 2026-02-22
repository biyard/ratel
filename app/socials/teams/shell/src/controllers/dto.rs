use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub permissions: Option<i64>,
}

#[cfg(feature = "server")]
impl From<(ratel_post::models::Team, i64)> for TeamResponse {
    fn from((team, permissions): (ratel_post::models::Team, i64)) -> Self {
        Self {
            pk: team.pk.to_string(),
            created_at: team.created_at,
            updated_at: team.updated_at,
            nickname: team.display_name,
            username: team.username,
            profile_url: Some(team.profile_url),
            dao_address: team.dao_address,
            html_contents: team.description,
            permissions: Some(permissions),
        }
    }
}
