use crate::features::spaces::pages::actions::*;
use crate::common::chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Weekday};
use crate::features::spaces::actions::discussion::controllers::{UpdateDiscussionRequest, update_discussion};
use crate::features::spaces::actions::poll::controllers::{UpdatePollRequest, update_poll};

use super::reward_cards::RewardPreviewData;

pub fn selected_actions(actions: &[SpaceAction], selected_ids: &[String]) -> Vec<SpaceAction> {
    selected_ids
        .iter()
        .filter_map(|selected_id| {
            actions
                .iter()
                .find(|action| supports_action_settings(action) && &action.action_id == selected_id)
                .cloned()
        })
        .collect()
}

pub fn available_actions(actions: &[SpaceAction], selected_ids: &[String]) -> Vec<SpaceAction> {
    actions
        .iter()
        .filter(|action| supports_action_settings(action))
        .filter(|action| {
            !selected_ids
                .iter()
                .any(|selected_id| selected_id == &action.action_id)
        })
        .cloned()
        .collect()
}

pub fn supports_action_settings(action: &SpaceAction) -> bool {
    !matches!(action.action_type, SpaceActionType::Subscription)
}

pub fn action_label(action: &SpaceAction, lang: &Language, untitled: &str) -> String {
    let title = if action.title.trim().is_empty() {
        untitled.to_string()
    } else {
        action.title.trim().to_string()
    };

    format!("{}: {}", action.action_type.translate(lang), title)
}

pub fn reward_preview_items(actions: &[SpaceAction]) -> Vec<RewardPreviewData> {
    if actions.is_empty() {
        return Vec::new();
    }

    // TODO: Replace these placeholder reward preview values with the actual
    // action-settings rewards API response once that endpoint is implemented.
    vec![RewardPreviewData {
        total_reward: 100_000,
        credits: 10,
        points: 10_000,
    }]
}

pub fn reward_credit_summary() -> (i64, i64) {
    // TODO: Replace these placeholder credit values with the actual membership
    // and reward usage API responses once action-settings reward integration is implemented.
    (2, 50)
}

pub async fn apply_selected_action_dates(
    space_id: SpacePartition,
    actions: Vec<SpaceAction>,
    started_at: i64,
    ended_at: i64,
) -> Result<()> {
    for action in actions {
        match action.action_type {
            SpaceActionType::Poll => {
                let entity_type: EntityType = action
                    .action_id
                    .parse()
                    .map_err(|_| Error::BadRequest("Invalid poll action id".to_string()))?;
                let poll_id: SpacePollEntityType = entity_type
                    .try_into()
                    .map_err(|_| Error::BadRequest("Invalid poll action id".to_string()))?;

                update_poll(
                    space_id.clone(),
                    poll_id,
                    UpdatePollRequest::Time {
                        started_at,
                        ended_at,
                    },
                )
                .await?;
            }
            SpaceActionType::TopicDiscussion => {
                let entity_type: EntityType = action
                    .action_id
                    .parse()
                    .map_err(|_| Error::BadRequest("Invalid discussion action id".to_string()))?;
                let discussion_id: SpacePostEntityType = entity_type
                    .try_into()
                    .map_err(|_| Error::BadRequest("Invalid discussion action id".to_string()))?;

                update_discussion(
                    space_id.clone(),
                    discussion_id,
                    UpdateDiscussionRequest {
                        title: None,
                        html_contents: None,
                        category_name: None,
                        started_at: Some(started_at),
                        ended_at: Some(ended_at),
                    },
                )
                .await?;
            }
            SpaceActionType::Subscription | SpaceActionType::Quiz => {
                return Err(Error::NotSupported(
                    "This action type is not supported yet.".to_string(),
                ));
            }
        }
    }

    Ok(())
}

pub fn resolve_action_time_range(
    start_date: &str,
    end_date: &str,
    time_zone: &str,
    select_dates_error: &str,
    invalid_date_range_error: &str,
    unsupported_time_zone_error: &str,
) -> Result<(i64, i64)> {
    let start = parse_local_date(start_date)
        .ok_or_else(|| Error::BadRequest(select_dates_error.to_string()))?;
    let end = parse_local_date(end_date)
        .ok_or_else(|| Error::BadRequest(select_dates_error.to_string()))?;

    let started_at = local_datetime_to_utc_millis(
        start
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::BadRequest(select_dates_error.to_string()))?,
        time_zone,
        unsupported_time_zone_error,
    )?;
    let ended_at = local_datetime_to_utc_millis(
        end.and_hms_milli_opt(23, 59, 59, 999)
            .ok_or_else(|| Error::BadRequest(select_dates_error.to_string()))?,
        time_zone,
        unsupported_time_zone_error,
    )?;

    if started_at >= ended_at {
        return Err(Error::BadRequest(invalid_date_range_error.to_string()));
    }

    Ok((started_at, ended_at))
}

fn parse_local_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}

fn local_datetime_to_utc_millis(
    local_datetime: NaiveDateTime,
    time_zone: &str,
    unsupported_time_zone_error: &str,
) -> Result<i64> {
    let offset_seconds =
        time_zone_offset_seconds(local_datetime, time_zone, unsupported_time_zone_error)?;
    let utc_datetime = local_datetime - Duration::seconds(offset_seconds.into());

    Ok(utc_datetime.and_utc().timestamp_millis())
}

fn time_zone_offset_seconds(
    local_datetime: NaiveDateTime,
    time_zone: &str,
    unsupported_time_zone_error: &str,
) -> Result<i32> {
    match time_zone {
        "UTC" => Ok(0),
        "Asia/Seoul" => Ok(9 * 60 * 60),
        "America/New_York" => Ok(new_york_offset_seconds(local_datetime)),
        _ => Err(Error::BadRequest(unsupported_time_zone_error.to_string())),
    }
}

fn new_york_offset_seconds(local_datetime: NaiveDateTime) -> i32 {
    let year = local_datetime.date().year();
    let dst_start = nth_weekday_of_month(year, 3, Weekday::Sun, 2)
        .and_hms_opt(2, 0, 0)
        .expect("valid DST start");
    let dst_end = nth_weekday_of_month(year, 11, Weekday::Sun, 1)
        .and_hms_opt(2, 0, 0)
        .expect("valid DST end");

    if local_datetime >= dst_start && local_datetime < dst_end {
        -4 * 60 * 60
    } else {
        -5 * 60 * 60
    }
}

fn nth_weekday_of_month(year: i32, month: u32, weekday: Weekday, nth: u8) -> NaiveDate {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("valid month");
    let first_weekday = first_day.weekday().num_days_from_monday() as i32;
    let target_weekday = weekday.num_days_from_monday() as i32;
    let days_until_weekday = (target_weekday - first_weekday).rem_euclid(7) as u32;
    let day = 1 + days_until_weekday + 7 * (nth as u32 - 1);

    NaiveDate::from_ymd_opt(year, month, day).expect("valid weekday occurrence")
}
