use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct GroupPermissionVerifier {
    team_id: i64,
    group_id: i64,
    team: Option<Team>,
    group: Option<Group>,
    group_member: Option<GroupMember>,
}

impl GroupPermissionVerifier {
    pub async fn new(pool: &sqlx::Pool<sqlx::Postgres>, team_id: i64, group_id: i64) -> Self {
        let team = Team::query_builder()
            .id_equals(team_id)
            .query()
            .map(Team::from)
            .fetch_optional(pool)
            .await
            .unwrap_or_default();

        let group = Group::query_builder()
            .id_equals(group_id)
            .query()
            .map(Group::from)
            .fetch_optional(pool)
            .await
            .unwrap_or_default();

        let group_member = GroupMember::query_builder()
            .group_id_equals(group_id)
            .user_id_equals(team_id)
            .query()
            .map(GroupMember::from)
            .fetch_optional(pool)
            .await
            .unwrap_or_default();

        Self {
            team_id,
            group_id,
            team,
            group,
            group_member,
        }
    }
}

impl MainGroupPermissionVerifier for GroupPermissionVerifier {
    fn has_group_permission(&self, perm: GroupPermission) -> bool {
        if self.group.clone().is_none() {
            return false;
        }

        if self.team.is_none() {
            return false;
        }

        if self.group_member.is_none() {
            return false;
        }

        let group = self.group.clone().unwrap_or_default();
        let permission_check = group.permissions & (1_i64 << (perm as i32)) != 0;

        permission_check
    }
}
