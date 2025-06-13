use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TeamPermissionVerifier {
    team_id: i64,
    team: Team,
}

impl TeamPermissionVerifier {
    pub async fn new(team_id: i64, pool: &sqlx::Pool<sqlx::Postgres>) -> Self {
        let team = Team::query_builder()
            .id_equals(team_id)
            .query()
            .map(Team::from)
            .fetch_one(pool)
            .await
            .expect("Failed to get team by ID");

        Self { team_id, team }
    }
}

impl PermissionVerifier for TeamPermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool {
        if user.id == self.team_id || user.id == self.team.parent_id {
            return true;
        }

        user.groups
            .iter()
            .filter(|x| {
                x.creator_id == self.team_id && x.permissions & (1_i64 << (perm as i32)) != 0
            })
            .count()
            > 0
    }
}
