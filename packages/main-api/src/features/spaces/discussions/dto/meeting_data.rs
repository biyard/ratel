use bdk::prelude::*;

use crate::features::spaces::discussions::dto::discussion_user::DiscussionUser;
use crate::types::attendee_info::AttendeeInfo;
use crate::types::meeting_info::MeetingInfo;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "PascalCase")]
pub struct MeetingData {
    pub meeting: MeetingInfo,
    pub attendee: AttendeeInfo,
    pub participants: Vec<DiscussionUser>,
    pub record: Option<String>,
}
