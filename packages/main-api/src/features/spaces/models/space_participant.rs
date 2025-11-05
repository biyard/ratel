use crate::types::*;
use crate::*;

use super::SpaceCommon;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct SpaceParticipant {
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub created_at: i64,
    pub display_name: String,
    #[dynamo(index = "gsi3", sk)]
    pub username: String,
    pub profile_url: String,

    pub user_type: UserType,

    #[dynamo(prefix = "SP", name = "find_by_space", index = "gsi2", pk)]
    #[dynamo(prefix = "SP", name = "search_users_by_space", index = "gsi3", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "SP", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
}

impl SpaceParticipant {
    pub fn new(space_pk: Partition, user_pk: Partition, display_name: String) -> Self {
        let created_at = time::get_now_timestamp_millis();
        let username = display_name.replace(' ', "_").to_lowercase();

        Self {
            pk: CompositePartition(space_pk.clone(), user_pk.clone()),
            sk: EntityType::SpaceParticipant,
            created_at,
            display_name,
            username,
            profile_url: "https://metadata.ratel.foundation/ratel/default-profile.png".to_string(),
            user_type: UserType::AnonymousSpaceUser,
            space_pk,
            user_pk,
        }
    }
}

impl From<(Partition, User)> for SpaceParticipant {
    fn from((space_pk, user): (Partition, User)) -> Self {
        SpaceParticipant {
            pk: CompositePartition(space_pk.clone(), user.pk.clone()),
            sk: EntityType::SpaceParticipant,
            created_at: user.created_at,
            display_name: user.display_name,
            username: user.username,
            user_type: user.user_type,
            profile_url: user.profile_url,
            space_pk,
            user_pk: user.pk,
        }
    }
}

impl FromRequestParts<AppState> for Option<SpaceParticipant> {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        Ok(SpaceParticipant::from_request_parts(parts, state)
            .await
            .ok())
    }
}

impl FromRequestParts<AppState> for SpaceParticipant {
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self> {
        let user = User::from_request_parts(parts, _state).await?;
        let space: &SpaceCommon = parts.extensions.get().ok_or(Error::SpaceNotFound)?;

        if let Some(sp) = parts.extensions.get::<SpaceParticipant>() {
            return Ok(sp.clone());
        }

        let user: SpaceParticipant = if space.should_explicit_participation() {
            SpaceParticipant::get(
                &_state.dynamo.client,
                CompositePartition(space.pk.clone(), user.pk),
                Some(EntityType::SpaceParticipant),
            )
            .await
            .map_err(|_| Error::NoUserFound)?
            .ok_or(Error::NoUserFound)?
        } else {
            // Auto-create participation record for spaces that do not require explicit participation
            (space.pk.clone(), user).into()
        };

        parts.extensions.insert(user.clone());

        Ok(user)
    }
}
