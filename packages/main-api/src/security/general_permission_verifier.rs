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
            .filter(|x| x.permissions & (perm as i64) != 0)
            .count()
            > 0
    }
}
