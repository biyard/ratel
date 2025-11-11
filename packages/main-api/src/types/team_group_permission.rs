use std::sync::Arc;

use crate::{
    models::{SpaceCommon, Team, UserTeamGroup},
    *,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    Translate,
    JsonSchema,
)]
#[repr(u8)]
// NOTE: If you add a new permission, you must update @/ts-packages/web/src/features/utils/tem-group-permissions.tsx too.
pub enum TeamGroupPermission {
    //Avaliable Permission Value: 0 ~ 63
    #[default]
    // Post Permissions
    PostRead = 0,
    PostWrite = 1, // When user want to create a post with team, they need both [PostWrite, PostEdit] permission.
    PostEdit = 2,
    PostDelete = 3,

    // Space Permissions
    SpaceRead = 10,
    SpaceWrite = 11,
    SpaceEdit = 12,
    SpaceDelete = 13,

    //Team Permission
    TeamAdmin = 20, // Change Group Permissions + All Other Permissions
    TeamEdit = 21,  // Edit Team Info, Add/Remove Group
    GroupEdit = 22, // Edit Group Members (Invite/Kick), Change Group Info
    // TeamDelete, //  Only Team Owner can delete the team.

    // Admin
    ManagePromotions = 62,
    ManageNews = 63,
}

#[derive(Debug, Copy, Clone)]
pub struct Permissions(pub i64);

impl Permissions {
    pub fn is_admin(&self) -> bool {
        (self.0 & (1 << TeamGroupPermission::TeamAdmin as i32)) != 0
    }

    pub fn all() -> Self {
        Self(i64::MAX)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn permitted(&self, perm: TeamGroupPermission) -> PermCheck<'_> {
        PermCheck { perms: self, perm }
    }

    pub fn contains(&self, permission: TeamGroupPermission) -> bool {
        (self.0 & (1 << permission as i32)) != 0
    }

    pub fn permitted(&self, permission: TeamGroupPermission) -> Result<()> {
        if !self.contains(permission) {
            Err(Error::NoPermission)
        } else {
            Ok(())
        }
    }

    pub fn read() -> Self {
        let mut perms = 0;
        for permission in [
            TeamGroupPermission::PostRead,
            TeamGroupPermission::SpaceRead,
        ] {
            perms |= 1 << permission as i32;
        }

        Self(perms)
    }
}

impl From<i64> for Permissions {
    fn from(permissions: i64) -> Self {
        Self(permissions)
    }
}

impl Into<i64> for Permissions {
    fn into(self) -> i64 {
        self.0
    }
}

impl std::ops::BitOr for Permissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::Add for Permissions {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone)]
pub struct TeamGroupPermissions(pub Vec<TeamGroupPermission>);

impl TeamGroupPermissions {
    pub fn all() -> Self {
        Self(vec![
            TeamGroupPermission::PostRead,
            TeamGroupPermission::PostWrite,
            TeamGroupPermission::PostEdit,
            TeamGroupPermission::PostDelete,
            TeamGroupPermission::SpaceRead,
            TeamGroupPermission::SpaceWrite,
            TeamGroupPermission::SpaceEdit,
            TeamGroupPermission::SpaceDelete,
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::TeamEdit,
            TeamGroupPermission::GroupEdit,
            TeamGroupPermission::ManagePromotions,
            TeamGroupPermission::ManageNews,
        ])
    }

    pub fn is_admin(&self) -> bool {
        self.contains(TeamGroupPermission::TeamAdmin)
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn read() -> Self {
        Self(vec![
            TeamGroupPermission::PostRead,
            TeamGroupPermission::SpaceRead,
        ])
    }

    pub fn contains(&self, permission: TeamGroupPermission) -> bool {
        self.0.contains(&permission)
    }
}

pub struct PermCheck<'a> {
    perms: &'a Permissions,
    perm: TeamGroupPermission,
}

impl<'a> PermCheck<'a> {
    #[inline]
    pub fn require(self) -> Result<()> {
        if self.perms.contains(self.perm) {
            Ok(())
        } else {
            Err(Error::NoPermission)
        }
    }
}

impl<'a> std::ops::Not for PermCheck<'a> {
    type Output = Result<()>;

    #[inline]
    fn not(self) -> Self::Output {
        if !self.perms.contains(self.perm) {
            Ok(())
        } else {
            Err(Error::NoPermission)
        }
    }
}

impl<'a> From<PermCheck<'a>> for Result<()> {
    #[inline]
    fn from(p: PermCheck<'a>) -> Self {
        p.require()
    }
}

impl Default for TeamGroupPermissions {
    fn default() -> Self {
        Self(vec![
            TeamGroupPermission::PostRead,
            TeamGroupPermission::PostWrite,
            TeamGroupPermission::PostEdit,
            TeamGroupPermission::PostDelete,
            TeamGroupPermission::SpaceRead,
            TeamGroupPermission::SpaceWrite,
            TeamGroupPermission::SpaceEdit,
            TeamGroupPermission::SpaceDelete,
        ])
    }
}

impl std::ops::BitOr for TeamGroupPermissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut combined = self.0;
        for perm in rhs.0 {
            if !combined.contains(&perm) {
                combined.push(perm);
            }
        }
        Self(combined)
    }
}

impl AsRef<[TeamGroupPermission]> for TeamGroupPermissions {
    fn as_ref(&self) -> &[TeamGroupPermission] {
        &self.0
    }
}

impl From<TeamGroupPermissions> for i64 {
    fn from(permissions: TeamGroupPermissions) -> Self {
        let mut result = 0;
        for permission in permissions.0 {
            result |= 1 << permission as i32;
        }
        result
    }
}

impl From<i64> for TeamGroupPermissions {
    fn from(permissions: i64) -> Self {
        let mut vec = Vec::new();
        for i in TeamGroupPermission::VARIANTS {
            if permissions & (1 << (*i as i32)) != 0 {
                vec.push(*i);
            }
        }
        Self(vec)
    }
}

impl Into<i64> for &TeamGroupPermissions {
    fn into(self) -> i64 {
        let mut result = 0;
        for permission in &self.0 {
            result |= 1 << *permission as i32;
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceOwnership {
    User(Partition),
    Team(Partition),
}

impl From<Partition> for ResourceOwnership {
    fn from(partition: Partition) -> Self {
        match partition {
            Partition::User(_) => ResourceOwnership::User(partition),
            Partition::Team(_) => ResourceOwnership::Team(partition),
            _ => panic!("Invalid partition for ResourceOwnership"),
        }
    }
}

#[async_trait::async_trait]
pub trait ResourcePermissions: Send + Sync {
    fn viewer_permissions(&self) -> Permissions;
    fn participant_permissions(&self) -> Permissions;
    fn resource_owner(&self) -> ResourceOwnership;
    async fn is_participant(&self, cli: &aws_sdk_dynamodb::Client, requester: &Partition) -> bool;
    async fn can_participate(&self, cli: &aws_sdk_dynamodb::Client, requester: &Partition) -> bool;
}

/// NoopPermissions will be used for accessing team pages.
#[derive(Debug, Clone, Copy)]
pub struct NoopPermissions;

#[async_trait::async_trait]
impl ResourcePermissions for NoopPermissions {
    fn viewer_permissions(&self) -> Permissions {
        Permissions::empty()
    }

    fn participant_permissions(&self) -> Permissions {
        Permissions::empty()
    }

    fn resource_owner(&self) -> ResourceOwnership {
        ResourceOwnership::Team(Partition::Team("noop".to_string()))
    }

    async fn is_participant(
        &self,
        _cli: &aws_sdk_dynamodb::Client,
        _requester: &Partition,
    ) -> bool {
        false
    }

    async fn can_participate(
        &self,
        _cli: &aws_sdk_dynamodb::Client,
        _requester: &Partition,
    ) -> bool {
        false
    }
}

#[async_trait::async_trait]
pub trait EntityPermissions: Send + Sync {
    async fn get_permissions_for(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        requester: &Partition,
    ) -> Permissions;
}

impl FromRequestParts<AppState> for Permissions {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        tracing::debug!("extracting user from request parts");
        if let Some(permissions) = parts.extensions.get::<Self>() {
            return Ok(permissions.clone());
        }

        // Select Resource (Space, Post, ..)
        let resource: Arc<dyn ResourcePermissions + Send + Sync> =
            if let Some(space) = parts.extensions.get::<SpaceCommon>() {
                Arc::new(space.clone())
            } else {
                Arc::new(NoopPermissions {})
            };

        tracing::debug!("resource found: {:?}", resource.resource_owner());

        // Check Participant permission
        let requester = User::from_request_parts(parts, state).await.ok();
        tracing::debug!("requester: {:?}", requester);
        let participant_permissions = match requester {
            Some(ref user) => {
                if resource
                    .can_participate(&state.dynamo.client, &user.pk)
                    .await
                    || resource
                        .is_participant(&state.dynamo.client, &user.pk)
                        .await
                {
                    resource.participant_permissions()
                } else {
                    Permissions::empty()
                }
            }
            _ => Permissions::empty(),
        };

        tracing::debug!("participant_permissions: {:?}", participant_permissions);

        // Check if requester permissions for the owner entity
        let owner_entity: Arc<dyn EntityPermissions + Send + Sync> = match resource.resource_owner()
        {
            // NOTE: now we don't get real owner data from database because we don't allow users to deligate their permission to others.
            // Therefore, it just compare user.pk with owner_pk.
            ResourceOwnership::User(owner_pk) => Arc::new(User::default().with_pk(owner_pk)),

            // If team resource, it must be injected beforehand
            ResourceOwnership::Team(_team_pk) => match parts.extensions.get::<Team>() {
                Some(team) => Arc::new(team.clone()),
                _ => {
                    return Err(Error::InternalServerError(
                        "Team resource must have Team injected in request parts".to_string(),
                    ));
                }
            },
        };

        let entity_permissions = match requester {
            Some(ref user) => {
                owner_entity
                    .get_permissions_for(&state.dynamo.client, &user.pk)
                    .await
            }
            _ => Permissions::empty(),
        };

        let combined_permissions =
            entity_permissions + participant_permissions + resource.viewer_permissions();
        parts.extensions.insert(combined_permissions);

        Ok(combined_permissions)
    }
}
