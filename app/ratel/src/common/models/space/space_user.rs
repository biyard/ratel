use crate::common::models::auth::User;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{CompositePartition, EntityType, Partition};
use crate::common::*;
#[cfg(feature = "server")]
use tower_sessions::Session;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpaceUser {
    pub pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<User> for SpaceUser {
    fn from(user: User) -> Self {
        Self {
            pk: user.pk,
            display_name: user.display_name,
            profile_url: user.profile_url,
            username: user.username,
        }
    }
}

impl From<SpaceParticipant> for SpaceUser {
    fn from(participant: SpaceParticipant) -> Self {
        Self {
            pk: participant.user_pk,
            display_name: participant.display_name,
            profile_url: participant.profile_url,
            username: participant.username,
        }
    }
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for SpaceUser
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        if let Some(author) = parts.extensions.get::<SpaceUser>() {
            return Ok(author.clone());
        }

        let user = User::from_request_parts(parts, state).await?;
        let space = SpaceCommon::from_request_parts(parts, state).await?;

        let author: SpaceUser = if space.anonymous_participation {
            if let Some(participant) = parts.extensions.get::<SpaceParticipant>() {
                participant.clone().into()
            } else {
                let config = ServerConfig::default();
                let cli = config.dynamodb();
                let participant = SpaceParticipant::get(
                    cli,
                    CompositePartition(space.pk.clone(), user.pk.clone()),
                    Some(EntityType::SpaceParticipant),
                )
                .await?;

                participant.map(Into::into).unwrap_or_else(|| user.into())
            }
        } else {
            user.into()
        };

        parts.extensions.insert(author.clone());
        Ok(author)
    }
}

/// Backward-compatible alias
pub type SpaceAuthor = SpaceUser;
