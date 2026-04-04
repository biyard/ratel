use crate::features::activity::models::SpaceActivity;
use crate::features::activity::*;

#[cfg(feature = "server")]
fn calculate_base_score(data: &SpaceActivityData, activity_score: i64) -> i64 {
    match data {
        SpaceActivityData::Poll { .. } => activity_score,
        SpaceActivityData::Follow { .. } => activity_score,
        SpaceActivityData::Quiz { passed, .. } => {
            if *passed {
                activity_score
            } else {
                0
            }
        }
        SpaceActivityData::Discussion {
            is_first_contribution,
            ..
        } => {
            if *is_first_contribution {
                activity_score
            } else {
                0
            }
        }
        SpaceActivityData::Unknown => 0,
    }
}

#[cfg(feature = "server")]
fn calculate_additional_score(data: &SpaceActivityData, additional_score_per_item: i64) -> i64 {
    match data {
        SpaceActivityData::Poll {
            answered_optional_count,
            ..
        } => additional_score_per_item * (*answered_optional_count as i64),
        SpaceActivityData::Quiz { correct_count, .. } => {
            additional_score_per_item * (*correct_count as i64)
        }
        SpaceActivityData::Discussion { .. } => additional_score_per_item,
        SpaceActivityData::Follow { .. } => 0,
        SpaceActivityData::Unknown => 0,
    }
}

#[cfg(feature = "server")]
pub(crate) async fn record_activity(
    cli: &aws_sdk_dynamodb::Client,
    space_id: SpacePartition,
    author: AuthorPartition,
    action_id: String,
    action_type: crate::features::spaces::pages::actions::types::SpaceActionType,
    activity_score: i64,
    additional_score_per_item: i64,
    data: SpaceActivityData,
    user_name: String,
    user_avatar: String,
) -> crate::common::Result<()> {
    let base = calculate_base_score(&data, activity_score);
    let additional = calculate_additional_score(&data, additional_score_per_item);

    let activity = SpaceActivity::new(
        space_id,
        author,
        action_id,
        action_type,
        data,
        base,
        additional,
        user_name,
        user_avatar,
    );
    activity.create(cli).await?;
    Ok(())
}
