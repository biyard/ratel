use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceDiscussionParticipant {
    pub pk: Partition, //discussion_pk
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType, //participant id

    pub participant_id: Option<String>,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl SpaceDiscussionParticipant {
    pub fn new(
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
        Self {
            pk: discussion_pk,
            sk: EntityType::SpaceDiscussionParticipant(participant_id.clone()),
            participant_id: Some(participant_id),
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
        }
    }

    pub fn id(&self) -> String {
        if let Some(id) = &self.participant_id {
            return id.clone();
        }
        if let EntityType::SpaceDiscussionParticipant(v) = &self.sk {
            return v.clone();
        }
        String::new()
    }

    pub fn discussion_user_pk_or_compute(&self) -> Partition {
        let discussion_id = match &self.pk {
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
