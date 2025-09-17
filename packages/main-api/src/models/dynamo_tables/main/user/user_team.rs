use crate::{models::team::Team, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserTeam {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    // NOTE: Sort teams for a user by last_used_at in descending order.
    pub last_used_at: i64,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    #[dynamo(prefix = "TEAM_PK", name = "find_by_team_pk", index = "gsi1", pk)]
    pub team_pk: Partition,
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
            sk: EntityType::UserTeam,
            last_used_at,
            display_name,
            profile_url,
            username,
            team_pk,
        }
    }
}
