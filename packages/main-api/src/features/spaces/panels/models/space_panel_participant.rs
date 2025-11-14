use crate::models::User;
use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpacePanelParticipant {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(index = "gsi2", name = "find_by_space_and_user", pk)]
    pub space_pk: Partition,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    #[dynamo(index = "gsi2", sk)]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl SpacePanelParticipant {
    pub fn new(
        space_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let (space_pk, sk) = Self::keys(&space_pk, &pk);

        Self {
            pk: space_pk.clone(),
            sk,
            space_pk,
            user_pk: pk,
            display_name,
            profile_url,
            username,
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpacePanelParticipant(user_pk.to_string()),
        )
    }

    pub async fn get_participant_in_space(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
    ) -> Option<Self> {
        let (items, _) = SpacePanelParticipant::find_by_space_and_user(
            cli,
            space_pk.clone(),
            SpacePanelParticipantQueryOption::builder()
                .sk(user_pk.to_string())
                .limit(1),
        )
        .await
        .unwrap_or_default();

        if items.is_empty() {
            None
        } else {
            Some(items[0].clone())
        }
    }
}
