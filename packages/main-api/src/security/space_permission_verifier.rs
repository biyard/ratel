use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct SpacePermissionVerifier {
    space_id: i64,
    team_id: i64,
    team: Team,
    space: Space,
}

// FIXME: fix space permission correctly
impl SpacePermissionVerifier {
    pub async fn new(team_id: i64, space_id: i64, pool: &sqlx::Pool<sqlx::Postgres>) -> Self {
        let space = Space::query_builder(team_id)
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_optional(pool)
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        let team = Team::query_builder()
            .id_equals(space.owner_id)
            .query()
            .map(Team::from)
            .fetch_one(pool)
            .await
            .expect("Failed to get team by ID");

        Self {
            space_id,
            team_id,
            team,
            space,
        }
    }
}

impl PermissionVerifier for SpacePermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool {
        let owner_id = self.space.clone().owner_id;

        if user.id == owner_id || user.id == self.team.parent_id {
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
