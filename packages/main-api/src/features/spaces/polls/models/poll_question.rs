use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PollQuestion {
    pub pk: Partition,
    pub sk: EntityType,

    pub questions: Vec<Question>, // Questions in the survey

                                  // pub question : Question,
}

impl PollQuestion {
    pub fn new(space_pk: Partition, questions: Vec<Question>) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpacePollQuestion,
            questions,
        }
    }
}
