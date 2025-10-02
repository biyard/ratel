use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    PartialEq,
    schemars::JsonSchema,
)]
pub enum SurveyType {
    #[default]
    Sample,
    Survey,
}
