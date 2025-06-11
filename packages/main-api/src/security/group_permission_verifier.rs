use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct GroupPermissionVerifier {
    group_id: i64,
}

impl GroupPermissionVerifier {
    pub fn new(group_id: i64) -> Self {
        Self { group_id }
    }
}

impl PermissionVerifier for GroupPermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool {
        if user.id == self.group_id {
            return true;
        }

        let mut group = None;

        for g in user.groups.clone() {
            if g.id == self.group_id {
                group = Some(g);
                break;
            }
        }

        if group.is_none() {
            return false;
        }

        let group = group.unwrap();

        let is_member = group.members.iter().any(|member| member.id == user.id);

        let permission_check = group.permissions & (1_i64 << (perm as i32)) != 0;

        permission_check && is_member
    }
}
