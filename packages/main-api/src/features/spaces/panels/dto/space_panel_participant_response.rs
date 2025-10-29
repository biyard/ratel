use crate::{features::spaces::panels::SpacePanelParticipant, types::Partition};
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
pub struct SpacePanelParticipantResponse {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<SpacePanelParticipant> for SpacePanelParticipantResponse {
    fn from(p: SpacePanelParticipant) -> Self {
        Self {
            user_pk: p.clone().user_pk,
            display_name: p.clone().display_name,
            profile_url: p.clone().profile_url,
            username: p.clone().username,
        }
    }
}
