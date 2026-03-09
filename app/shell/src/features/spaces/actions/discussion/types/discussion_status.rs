use crate::features::spaces::actions::discussion::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum DiscussionStatus {
    #[default]
    NotStarted,
    InProgress,
    Finish,
}
