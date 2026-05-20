use crate::common::types::UserType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamItem {
    pub pk: String,
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
    pub user_type: UserType,
    #[serde(default)]
    pub permissions: Vec<u8>,
    #[serde(default)]
    pub description: String,
    /// Epoch-ms the team was created. `0` for legacy rows. Used by
    /// the sub-team apply page to live-evaluate the parent's
    /// `min_sub_team_age_days` requirement against the picked team.
    #[serde(default)]
    pub created_at: i64,
    /// Member count (UserTeam rows). `0` for legacy rows. Used by
    /// the sub-team apply page to live-evaluate the parent's
    /// `min_sub_team_members` requirement against the picked team.
    #[serde(default)]
    pub member_count: i64,
}

impl TeamItem {
    pub fn permission_mask(&self) -> i64 {
        let mut mask = 0i64;
        for v in &self.permissions {
            mask |= 1i64 << (*v as i32);
        }
        mask
    }

    pub fn has_permission(
        &self,
        permission: crate::features::posts::types::TeamGroupPermission,
    ) -> bool {
        let permissions: crate::features::posts::types::TeamGroupPermissions =
            self.permission_mask().into();
        permissions.contains(permission)
    }
}
