use crate::features::activity::models::SpaceActivity;
use crate::features::activity::*;

/// Fixed XP values per activity type
#[cfg(feature = "server")]
pub const XP_FOLLOW: i64 = 1_000;
#[cfg(feature = "server")]
pub const XP_QUIZ_PASSED: i64 = 20_000;
#[cfg(feature = "server")]
pub const XP_QUIZ_FAILED: i64 = 1_000;
#[cfg(feature = "server")]
pub const XP_DISCUSSION_REPLY: i64 = 5_000;
#[cfg(feature = "server")]
pub const XP_POLL: i64 = 50_000;

#[cfg(feature = "server")]
fn calculate_xp(data: &SpaceActivityData) -> i64 {
    match data {
        SpaceActivityData::Poll { .. } => XP_POLL,
        SpaceActivityData::Follow { .. } => XP_FOLLOW,
        SpaceActivityData::Quiz { passed, .. } => {
            if *passed { XP_QUIZ_PASSED } else { XP_QUIZ_FAILED }
        }
        SpaceActivityData::Discussion { .. } => XP_DISCUSSION_REPLY,
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
    data: SpaceActivityData,
    user_name: String,
    user_avatar: String,
) -> crate::common::Result<()> {
    let xp = calculate_xp(&data);

    let activity = SpaceActivity::new(
        space_id,
        author,
        action_id,
        action_type,
        data,
        xp,
        0,
        user_name,
        user_avatar,
    );
    activity.create(cli).await?;
    Ok(())
}
