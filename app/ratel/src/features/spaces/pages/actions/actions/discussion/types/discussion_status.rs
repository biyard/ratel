use crate::features::spaces::pages::actions::actions::discussion::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum DiscussionStatus {
    #[default]
    NotStarted,
    InProgress,
    Finish,
}
