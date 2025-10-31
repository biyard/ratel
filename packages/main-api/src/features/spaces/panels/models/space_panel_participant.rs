use crate::models::User;
use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpacePanelParticipant {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl SpacePanelParticipant {
    pub fn new(
        panel_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let (panel_pk, sk) = Self::keys(&panel_pk, &pk);

        Self {
            pk: panel_pk,
            sk,
            user_pk: pk,
            display_name,
            profile_url,
            username,
        }
    }

    pub fn keys(panel_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            panel_pk.clone(),
            EntityType::SpacePanelParticipant(panel_pk.to_string(), user_pk.to_string()),
        )
    }

    pub async fn is_participant(
        cli: &aws_sdk_dynamodb::Client,
        discussion_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<bool, crate::Error> {
        let (pk, sk) = Self::keys(discussion_pk, user_pk);
        let participant = SpacePanelParticipant::get(&cli, pk, Some(sk)).await?;

        Ok(participant.is_some())
    }
}
