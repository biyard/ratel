//! Checks whether all actions in a chapter are cleared by the user,
//! and if so, applies the chapter's completion benefit.

#[cfg(feature = "server")]
use crate::common::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::gamification::services::apply_chapter_benefit;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::gamification::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::models::SpaceAction;

/// Determines whether every action in the given chapter is complete for
/// the specified user.  If all actions are cleared, the chapter's
/// completion benefit is applied and the function returns
/// `(true, Some(role))` when a role upgrade occurred, or
/// `(true, None)` for XP-only chapters.
///
/// If the chapter is not yet complete, returns `(false, None)`.
#[cfg(feature = "server")]
pub async fn check_chapter_complete(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
    chapter_entity: &SpaceChapterEntityType,
) -> Result<(bool, Option<SpaceUserRole>)> {
    let chapter_id_str = &chapter_entity.0;

    // Load the chapter entity.
    let (ch_pk, ch_sk) = SpaceChapter::keys(space_pk, chapter_id_str);
    let chapter = match SpaceChapter::get(cli, &ch_pk, Some(&ch_sk)).await? {
        Some(c) => c,
        None => {
            crate::error!(
                "check_chapter_complete: chapter not found: {}",
                chapter_id_str
            );
            return Ok((false, None));
        }
    };

    // Load all actions in this space.
    let (all_actions, _) = SpaceAction::find_by_space(cli, space_pk, SpaceAction::opt())
        .await
        .map_err(|e| {
            crate::error!("check_chapter_complete: failed to load actions: {e}");
            Error::InternalServerError("failed to load actions".into())
        })?;

    // Filter to only actions belonging to this chapter.
    let chapter_actions: Vec<&SpaceAction> = all_actions
        .iter()
        .filter(|a| {
            a.chapter_id
                .as_ref()
                .map(|cid| cid.0 == *chapter_id_str)
                .unwrap_or(false)
        })
        .collect();

    if chapter_actions.is_empty() {
        return Ok((false, None));
    }

    // Collect the user's cleared action ids using the same logic
    // employed by `get_quest_map`.  We reuse the helper from the
    // quest-map controller to keep the cleared-check logic centralised.
    let cleared_ids = crate::features::spaces::pages::actions::gamification::controllers::collect_cleared_action_ids(
        cli, space_pk, user_pk, &all_actions,
    )
    .await?;

    // Check whether every chapter action is cleared.
    let all_cleared = chapter_actions
        .iter()
        .all(|a| cleared_ids.contains(&a.pk.1));

    if !all_cleared {
        return Ok((false, None));
    }

    // All actions in this chapter are cleared — apply the benefit.
    let role_upgraded = apply_chapter_benefit(cli, space_pk, user_pk, &chapter).await?;
    Ok((true, role_upgraded))
}
