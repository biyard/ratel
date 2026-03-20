use crate::common::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    // DynamoEnum,
    // JsonSchema,
    // OperationIo,
    Translate,
    PartialEq,
    Eq,
)]
pub enum SpaceUserRole {
    #[default]
    #[translate(ko = "뷰어")]
    Viewer,
    #[translate(ko = "참가자")]
    Participant,
    #[translate(ko = "참가후보")]
    Candidate,
    #[translate(ko = "관리자")]
    Creator,
}

impl SpaceUserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, SpaceUserRole::Creator)
    }

    pub fn can_edit(&self) -> bool {
        matches!(self, SpaceUserRole::Creator)
    }

    pub fn can_act(&self) -> bool {
        matches!(self, SpaceUserRole::Participant | SpaceUserRole::Creator)
    }
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for SpaceUserRole
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        use crate::common::models::auth::User;
        use crate::common::models::space::{SpaceCommon, SpaceParticipant};
        use crate::common::types::{CompositePartition, EntityType};
        tracing::debug!("extracting space from request parts. Path: {:?}", parts.uri);

        if let Some(space_role) = parts.extensions.get::<SpaceUserRole>() {
            return Ok(space_role.clone());
        }

        let space = SpaceCommon::from_request_parts(parts, state).await?;

        let user = User::from_request_parts(parts, state).await.ok();

        let public_space = space.is_public();

        if user.is_none() {
            if public_space {
                parts.extensions.insert(SpaceUserRole::Viewer);
                return Ok(SpaceUserRole::Viewer);
            } else {
                return Err(Error::UnauthorizedAccess);
            }
        }

        let user = user.unwrap();

        // Individual creator check
        if user.pk == space.user_pk {
            parts.extensions.insert(SpaceUserRole::Creator);
            return Ok(SpaceUserRole::Creator);
        }

        let conf = config::ServerConfig::default();
        let cli = conf.dynamodb();

        // Team admin check: if the space is owned by a team, check if the user
        // has TeamAdmin permission within that team.
        if matches!(&space.user_pk, Partition::Team(_)) {
            use crate::features::posts::models::Team;
            use crate::features::posts::types::TeamGroupPermission;

            if Team::has_permission(
                cli,
                &space.user_pk,
                &user.pk,
                TeamGroupPermission::TeamAdmin,
            )
            .await
            .unwrap_or(false)
            {
                parts.extensions.insert(SpaceUserRole::Creator);
                return Ok(SpaceUserRole::Creator);
            }
        }

        // Check participant
        let participant = SpaceParticipant::get(
            cli,
            CompositePartition(space.pk.clone(), user.pk.clone()),
            Some(EntityType::SpaceParticipant),
        )
        .await
        .ok()
        .flatten();

        if participant.is_some() {
            parts.extensions.insert(SpaceUserRole::Participant);
            return Ok(SpaceUserRole::Participant);
        }

        // For public spaces, unauthenticated users are Viewers (handled above),
        // but authenticated non-participants are also Viewers.
        if public_space {
            parts.extensions.insert(SpaceUserRole::Viewer);
            return Ok(SpaceUserRole::Viewer);
        }

        Err(Error::UnauthorizedAccess)
    }
}
