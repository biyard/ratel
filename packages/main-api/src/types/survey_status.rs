use bdk::prelude::*;

#[derive(
    Debug, Clone, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, Default, DynamoEnum,
)]
pub enum SurveyStatus {
    #[default]
    Ready = 1,
    InProgress = 2,
    Finish = 3,
}
