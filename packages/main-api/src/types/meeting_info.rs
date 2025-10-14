use validator::Validate;
use super::media_placement_info::MediaPlacementInfo;
use bdk::prelude::*;

#[derive(Validate,serde::Serialize, serde::Deserialize, schemars::JsonSchema, aide::OperationIo)]
#[serde(rename_all = "PascalCase")]
pub struct MeetingInfo {
    pub meeting_id: String,
    pub media_placement: MediaPlacementInfo,
    pub media_region: String,
}
