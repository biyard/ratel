use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    JsonSchema,
    DynamoEnum,
    Default,
)]
pub enum NotificationStatus {
    #[default]
    Unread,
    Read,
}
