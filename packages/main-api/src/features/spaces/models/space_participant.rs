use crate::features::spaces::panels::{
    SpacePanel, SpacePanelParticipant, SpacePanelQueryOption, SpacePanelResponse,
};
use crate::types::*;
use crate::utils::aws::DynamoClient;
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
        let username = display_name.replace(' ', "-").to_lowercase();

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

    pub fn keys(space_pk: Partition, user_pk: Partition) -> (CompositePartition, EntityType) {
        (
            CompositePartition(space_pk, user_pk),
            EntityType::SpaceParticipant,
        )
    }

    pub async fn verify_credential(
        dynamo: &DynamoClient,
        space_pk: &Partition,
        user: User,
    ) -> bool {
        // FIXME: fix to real credential info
        let age = 26;
        let gender = "male";
        let gender = Self::parse_gender(gender);

        let mut bookmark = None::<String>;

        loop {
            let opt = match &bookmark {
                Some(b) => SpacePanelQueryOption::builder()
                    .sk("SPACE_PANEL#".into())
                    .bookmark(b.clone()),
                None => SpacePanelQueryOption::builder().sk("SPACE_PANEL#".into()),
            };

            let (panels, next) =
                match SpacePanel::query(&dynamo.client, space_pk.clone(), opt).await {
                    Ok(v) => v,
                    Err(_) => return false,
                };

            for p in panels {
                if p.participants >= p.quotas {
                    continue;
                }
                if Self::attributes_match(age, gender.clone(), &p.attributes) {
                    let res: SpacePanelResponse = p.into();
                    let participants =
                        SpacePanelParticipant::new(space_pk.clone(), res.clone().pk, user);
                    let _ = match participants.create(&dynamo.client).await {
                        Ok(v) => v,
                        Err(_) => return false,
                    };
                    let (pk, sk) = SpacePanel::keys(&space_pk, &res.pk);
                    let _ = match SpacePanel::updater(pk, sk)
                        .increase_participants(1)
                        .execute(&dynamo.client)
                        .await
                    {
                        Ok(v) => v,
                        Err(_) => return false,
                    };

                    return true;
                }
            }

            if let Some(b) = next {
                bookmark = Some(b);
            } else {
                break;
            }
        }
        false
    }

    fn attributes_match(age: u8, gender: Option<Gender>, attrs: &[Attribute]) -> bool {
        if attrs.is_empty() {
            return true;
        }

        let mut age_rules: Vec<&Age> = Vec::new();
        let mut gender_rules: Vec<&Gender> = Vec::new();

        for attr in attrs {
            match attr {
                Attribute::Age(a) => age_rules.push(a),
                Attribute::Gender(g) => gender_rules.push(g),
            }
        }

        let age_ok = if age_rules.is_empty() {
            true
        } else {
            age_rules.iter().any(|rule| match rule {
                Age::Specific(a) => age == *a,
                Age::Range {
                    inclusive_min,
                    inclusive_max,
                } => age >= *inclusive_min && age <= *inclusive_max,
            })
        };

        let gender_ok = if gender_rules.is_empty() {
            true
        } else {
            match gender {
                Some(ref g) => gender_rules.iter().any(|rule| *rule == g),
                None => false,
            }
        };
        age_ok && gender_ok
    }

    fn parse_gender(s: &str) -> Option<Gender> {
        match s {
            "male" | "Male" => Some(Gender::Male),
            "female" | "Female" => Some(Gender::Female),
            _ => None,
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
