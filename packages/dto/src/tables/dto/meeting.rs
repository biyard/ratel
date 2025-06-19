use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip)]
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

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip)]
#[serde(rename_all = "PascalCase")]
pub struct MeetingInfo {
    pub meeting_id: String,
    pub media_placement: MediaPlacementInfo,
    pub media_region: String,
}

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip)]
#[serde(rename_all = "PascalCase")]
pub struct AttendeeInfo {
    pub attendee_id: String,
    pub join_token: String,
    pub external_user_id: String,
}

#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip, read_action = find_one)]
pub struct MeetingData {
    pub meeting: MeetingInfo,
    pub attendee: AttendeeInfo,
    pub record: Option<String>,
}
