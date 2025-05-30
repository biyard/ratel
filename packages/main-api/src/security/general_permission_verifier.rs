use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct GeneralPermissionVerifier {}

impl GeneralPermissionVerifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl PermissionVerifier for GeneralPermissionVerifier {
    fn has_permission(&self, user: &User, perm: GroupPermission) -> bool {
        user.groups
            .iter()
            .filter(|x| x.permissions & (1_i64 << (perm as i32)) != 0)
            .count()
            > 0
    }
}
