use crate::{models::team::Team, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserTeam {
    pub pk: Partition,
    #[dynamo(prefix = "TEAM_PK", index = "gsi1", name = "find_by_team" pk)]
    pub sk: EntityType,

    // NOTE: Sort teams for a user by last_used_at in descending order.
    #[dynamo(index = "gsi1", name = "find_by_team" sk)]
    pub last_used_at: i64,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl UserTeam {
    pub fn new(
        pk: Partition,
        Team {
            display_name,
            profile_url,
            username,
            pk: team_pk,
            ..
        }: Team,
    ) -> Self {
        let last_used_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk: EntityType::UserTeam(team_pk.to_string()),
            last_used_at,
            display_name,
            profile_url,
            username,
        }
    }
}
