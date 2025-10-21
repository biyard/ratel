use crate::features::spaces::discussions::models::space_discussion_participant::SpaceDiscussionParticipant;
use crate::types::Partition;
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceDiscussionParticipantResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub participant_id: String,
}

impl From<SpaceDiscussionParticipant> for SpaceDiscussionParticipantResponse {
    fn from(p: SpaceDiscussionParticipant) -> Self {
        Self {
            user_pk: p.clone().user_pk,
            author_display_name: p.clone().author_display_name,
            author_profile_url: p.clone().author_profile_url,
            author_username: p.clone().author_username,
            participant_id: p.clone().participant_id.unwrap_or_default(),
        }
    }
}
