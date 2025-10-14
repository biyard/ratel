use validator::Validate;
use bdk::prelude::*;

#[derive(Validate, serde::Serialize, serde::Deserialize, schemars::JsonSchema, aide::OperationIo)]
#[serde(rename_all = "PascalCase")]
pub struct MediaPlacementInfo {
    pub audio_host_url: String,
    pub audio_fallback_url: String,
    pub screen_data_url: String,
    pub screen_sharing_url: String,
    pub screen_viewing_url: String,
    pub signaling_url: String,
    pub turn_control_url: String,
}

