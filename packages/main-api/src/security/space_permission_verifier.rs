use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct SpacePermissionVerifier {
    space_id: i64,
    space: Option<Space>,
}

impl SpacePermissionVerifier {
    pub async fn new(user_id: i64, space_id: i64, pool: &sqlx::Pool<sqlx::Postgres>) -> Self {
        let space = Space::query_builder()
            .id_equals(space_id)
            .owner_id_equals(user_id)
            .query()
            .map(Space::from)
            .fetch_optional(pool)
            .await
            .unwrap_or_default();
        Self { space_id, space }
    }
}

impl PermissionVerifier for SpacePermissionVerifier {
    fn has_permission(&self, _user: &User, _perm: GroupPermission) -> bool {
        // FIXME: fix detail permission depending on space groups.
        self.space.is_some()
    }
}
