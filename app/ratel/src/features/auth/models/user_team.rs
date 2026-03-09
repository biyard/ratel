use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserTeam {
    pub pk: Partition,
    #[dynamo(prefix = "TEAM_PK", index = "gsi1", name = "find_by_team", pk)]
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub last_used_at: i64,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    pub dao_address: Option<String>,
}

#[cfg(feature = "server")]
impl UserTeam {
    pub fn new(
        user_pk: Partition,
        team_pk: Partition,
        display_name: String,
        profile_url: String,
        username: String,
        dao_address: Option<String>,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();

        Self {
            pk: user_pk,
            sk: EntityType::UserTeam(team_pk.to_string()),
            last_used_at: now,
            display_name,
            profile_url,
            username,
            dao_address,
        }
    }
}

#[derive(Default, Serialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct UserTeamResponse {
    pub nickname: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub dao_address: Option<String>,
}

impl From<UserTeam> for UserTeamResponse {
    fn from(user_team: UserTeam) -> Self {
        Self {
            nickname: user_team.display_name,
            profile_url: user_team.profile_url,
            username: user_team.username,
            dao_address: user_team.dao_address,
            user_type: UserType::Team,
        }
    }
}
