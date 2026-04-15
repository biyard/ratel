use super::super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "lowercase")]
pub enum TeamRole {
    #[translate(en = "Owner", ko = "소유자")]
    Owner,
    #[translate(en = "Admin", ko = "관리자")]
    Admin,
    #[default]
    #[translate(en = "Member", ko = "멤버")]
    Member,
}

impl TeamRole {
    /// Owner / Admin can edit team settings, manage members, manage groups,
    /// invite/remove members, etc. Member is read-only.
    pub fn is_admin_or_owner(self) -> bool {
        matches!(self, TeamRole::Owner | TeamRole::Admin)
    }

    pub fn is_owner(self) -> bool {
        matches!(self, TeamRole::Owner)
    }

    /// Derive a legacy `TeamGroupPermissions` bitmask from a role for
    /// backward-compatible API responses (DTOs that still expose
    /// `permissions: i64`). Owner gets the full bitmask, Admin gets the
    /// editing bits, Member only gets read bits.
    pub fn to_legacy_permissions(self) -> i64 {
        use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
        let perms: TeamGroupPermissions = match self {
            TeamRole::Owner => TeamGroupPermissions::all(),
            TeamRole::Admin => TeamGroupPermissions(vec![
                TeamGroupPermission::PostRead,
                TeamGroupPermission::PostWrite,
                TeamGroupPermission::PostEdit,
                TeamGroupPermission::PostDelete,
                TeamGroupPermission::SpaceRead,
                TeamGroupPermission::SpaceWrite,
                TeamGroupPermission::SpaceEdit,
                TeamGroupPermission::SpaceDelete,
                TeamGroupPermission::TeamEdit,
                TeamGroupPermission::GroupEdit,
            ]),
            TeamRole::Member => TeamGroupPermissions::read(),
        };
        perms.into()
    }

    /// Inverse mapping used by the migration: derive a role from an existing
    /// permissions bitmask. TeamAdmin bit → Owner, TeamEdit bit → Admin,
    /// otherwise Member.
    pub fn from_legacy_permissions(mask: i64) -> Self {
        use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
        let perms: TeamGroupPermissions = mask.into();
        if perms.contains(TeamGroupPermission::TeamAdmin) {
            TeamRole::Owner
        } else if perms.contains(TeamGroupPermission::TeamEdit) {
            TeamRole::Admin
        } else {
            TeamRole::Member
        }
    }
}

impl std::fmt::Display for TeamRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamRole::Owner => write!(f, "Owner"),
            TeamRole::Admin => write!(f, "Admin"),
            TeamRole::Member => write!(f, "Member"),
        }
    }
}
