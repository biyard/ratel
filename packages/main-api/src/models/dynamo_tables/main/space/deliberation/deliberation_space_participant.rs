use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceParticipant {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub sk: EntityType,

    pub participant_id: Option<String>,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    #[serde(default)]
    #[dynamo(
        prefix = "DISCUSSION_USER_PK",
        name = "find_by_discussion_user_pk",
        index = "gsi6",
        pk
    )]
    pub discussion_user_pk: Option<Partition>,

    #[dynamo(
        prefix = "DISCUSSION_PK",
        name = "find_by_discussion_pk",
        index = "gsi2",
        pk
    )]
    pub discussion_pk: Partition,
}

impl DeliberationSpaceParticipant {
    pub fn new(
        deliberation_pk: Partition,
        discussion_pk: Partition,
        participant_id: String,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let discussion_id = match &discussion_pk {
            Partition::Discussion(v) => v.as_str(),
            _ => "",
        };
        let user_id = match &pk {
            Partition::User(v) | Partition::Team(v) => v.as_str(),
            _ => "",
        };

        Self {
            pk: deliberation_pk,
            sk: EntityType::DeliberationSpaceParticipant(uid),
            participant_id: Some(participant_id),
            discussion_user_pk: Some(Partition::DiscussionUser(format!(
                "{discussion_id}#{user_id}"
            ))),
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            discussion_pk,
        }
    }

    pub fn id(&self) -> String {
        if let Some(id) = &self.participant_id {
            return id.clone();
        }
        if let EntityType::DeliberationSpaceParticipant(v) = &self.sk {
            return v.clone();
        }
        String::new()
    }

    pub fn discussion_user_pk_or_compute(&self) -> Partition {
        if let Some(pk) = &self.discussion_user_pk {
            return pk.clone();
        }
        let discussion_id = match &self.discussion_pk {
            Partition::Discussion(v) => v.as_str(),
            _ => "",
        };
        let user_id = match &self.user_pk {
            Partition::User(v) | Partition::Team(v) => v.as_str(),
            _ => "",
        };
        Partition::DiscussionUser(format!("{discussion_id}#{user_id}"))
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, schemars::JsonSchema)]
pub struct DiscussionParticipantResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub participant_id: String,
}

impl From<DeliberationSpaceParticipant> for DiscussionParticipantResponse {
    fn from(p: DeliberationSpaceParticipant) -> Self {
        Self {
            user_pk: p.clone().user_pk,
            author_display_name: p.clone().author_display_name,
            author_profile_url: p.clone().author_profile_url,
            author_username: p.clone().author_username,
            participant_id: p.id(),
        }
    }
}
