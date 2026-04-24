use crate::features::spaces::pages::actions::actions::meet::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum UpdateMeetRequest {
    Mode { mode: MeetMode },
    StartTime { start_time: i64 },
    DurationMin { duration_min: i32 },
}

#[mcp_tool(
    name = "update_meet",
    description = "Update meet-specific fields (mode, start_time, duration_min). Requires creator role."
)]
#[post("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole)]
pub async fn update_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
    #[mcp(description = "Meet sort key (e.g. 'SpaceMeet#<uuid>')")] meet_sk: SpaceMeetEntityType,
    #[mcp(description = "Meet update data as JSON. Supported variants: {\"Mode\": {\"mode\": \"Instant\"}}, {\"StartTime\": {\"start_time\": 123}}, {\"DurationMin\": {\"duration_min\": 60}}")]
    req: UpdateMeetRequest,
) -> Result<String> {
    SpaceMeet::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let meet_sk_entity: EntityType = meet_sk.into();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut meet_updater = SpaceMeet::updater(&space_pk, &meet_sk_entity).with_updated_at(now);

    match req {
        UpdateMeetRequest::Mode { mode } => {
            meet_updater = meet_updater.with_mode(mode);
        }
        UpdateMeetRequest::StartTime { start_time } => {
            meet_updater = meet_updater.with_start_time(start_time);
        }
        UpdateMeetRequest::DurationMin { duration_min } => {
            if !(15..=1440).contains(&duration_min) {
                return Err(MeetActionError::InvalidDuration(duration_min).into());
            }
            meet_updater = meet_updater.with_duration_min(duration_min);
        }
    }

    meet_updater.execute(cli).await.map_err(|e| {
        crate::error!("update meet failed: {e}");
        MeetActionError::UpdateFailed
    })?;

    Ok("success".to_string())
}
