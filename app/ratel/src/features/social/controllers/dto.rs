use crate::features::social::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
    /// `None` means the current viewer is not a member of the team (or is
    /// logged out). `Some(role)` = actual team membership with that role.
    /// Clients should use this for all access control decisions; never
    /// derive from `permissions` on the client.
    #[serde(default)]
    pub role: Option<crate::features::social::pages::member::dto::TeamRole>,
    /// Recognized parent's team id, if this team has been formally
    /// accepted as a sub-team. Exposed publicly so the bylaws page can
    /// pull the parent's required documents without needing the
    /// admin-only `/parent` relationship endpoint.
    #[serde(default)]
    pub parent_team_id: Option<String>,
    /// Parent team's `username` (handle) — resolved server-side from
    /// `parent_team_id` so the bylaws page can call the public
    /// `list_team_posts_handler(parent_username, "Bylaws")` without
    /// a second lookup roundtrip.
    #[serde(default)]
    pub parent_username: Option<String>,
}

#[cfg(feature = "server")]
impl From<(crate::features::posts::models::Team, Option<crate::features::social::pages::member::dto::TeamRole>)>
    for TeamResponse
{
    fn from(
        (team, role): (
            crate::features::posts::models::Team,
            Option<crate::features::social::pages::member::dto::TeamRole>,
        ),
    ) -> Self {
        let permissions_i64: i64 = role.map(|r| r.to_legacy_permissions()).unwrap_or(0);
        let perms: crate::features::posts::types::TeamGroupPermissions = permissions_i64.into();
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
            role,
            parent_team_id: team.parent_team_id,
            parent_username: None,
        }
    }
}

#[cfg(feature = "server")]
impl TeamResponse {
    /// Resolve `parent_username` from `parent_team_id` by fetching the
    /// parent's Team row. Caller-supplied client keeps this side-effect
    /// out of the basic `From` impl and avoids forcing every code path
    /// that builds a `TeamResponse` into the lookup.
    pub async fn with_parent_username(
        mut self,
        cli: &aws_sdk_dynamodb::Client,
    ) -> Self {
        let Some(parent_id) = self.parent_team_id.clone() else {
            return self;
        };
        if parent_id.is_empty() {
            return self;
        }
        let parent_pk = crate::common::types::Partition::Team(parent_id);
        if let Ok(Some(parent)) = crate::features::posts::models::Team::get(
            cli,
            &parent_pk,
            Some(crate::common::types::EntityType::Team),
        )
        .await
        {
            self.parent_username = Some(parent.username);
        }
        self
    }
}
