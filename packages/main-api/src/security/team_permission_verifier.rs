use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct TeamPermissionVerifier {
    team_id: i64,
}

impl TeamPermissionVerifier {
    pub fn new(team_id: i64) -> Self {
        Self { team_id }
    }
}

impl PermissionVerifier for TeamPermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool {
        if user.id == self.team_id {
            return true;
        }

        user.groups
            .iter()
            .filter(|x| x.user_id == self.team_id && x.permissions & (1_i64 << (perm as i32)) != 0)
            .count()
            > 0
    }
}
