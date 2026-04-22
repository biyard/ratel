use crate::features::spaces::pages::actions::*;

#[derive(Debug, Clone, Default, DynamoEnum, Translate, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SpaceActionStatus {
    #[default]
    #[translate(en = "Designing", ko = "설계중")]
    Designing = 1,
    #[translate(en = "Ongoing", ko = "진행중")]
    Ongoing = 2,
    #[translate(en = "Finish", ko = "종료")]
    Finish = 3,
}

impl SpaceActionStatus {
    pub fn allows_transition(from: Option<&SpaceActionStatus>, to: &SpaceActionStatus) -> bool {
        match (from, to) {
            (None, SpaceActionStatus::Designing) => true,
            (Some(SpaceActionStatus::Designing), SpaceActionStatus::Ongoing) => true,
            (Some(SpaceActionStatus::Ongoing), SpaceActionStatus::Finish) => true,
            _ => false,
        }
    }
}
