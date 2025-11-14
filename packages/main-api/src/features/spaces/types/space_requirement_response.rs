use crate::{
    features::spaces::{SpaceRequirement, polls::PollUserAnswer},
    *,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum SpaceRequirementResponse {
    PrePoll(PollUserAnswer),
}

impl SpaceRequirementResponse {
    pub fn pk(&self) -> String {
        match self {
            SpaceRequirementResponse::PrePoll(d) => d.pk.to_string(),
        }
    }

    pub fn sk(&self) -> String {
        match self {
            SpaceRequirementResponse::PrePoll(d) => d.sk.to_string(),
        }
    }
}
