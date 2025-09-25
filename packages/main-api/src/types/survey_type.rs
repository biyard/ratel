use bdk::prelude::*;

#[derive(
    Debug, Clone, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, Default, DynamoEnum,
)]
pub enum SurveyType {
    #[default]
    Sample,
    Survey,
}
