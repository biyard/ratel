//! Core XP computation service and its helpers.
//!
//! `award_xp` is the single entry-point called by every action-
//! submission controller (poll, quiz, discussion, follow) after a
//! successful response is persisted.  It orchestrates the full XP
//! pipeline: load the action, snapshot participants, bump combo +
//! streak, compute final XP, write the ledger, update the user's
//! global aggregate, check chapter completion, mint the creator share,
//! and return an `XpGainResponse` for the Completion Overlay.

#[cfg(feature = "server")]
use crate::common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use crate::common::utils::time::get_now_timestamp_millis;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::gamification::services::check_chapter_complete;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::gamification::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::models::SpaceAction;

#[cfg(feature = "server")]
use crate::common::*;

// ── Main entry point ─────────────────────────────────────────────────────────

/// Awards XP to a user for completing an action in a space.
///
/// This is the single function called by every action-submission
/// controller after the user's response has been successfully persisted.
/// It returns an `XpGainResponse` that the controller can forward to
/// the client for the Completion Overlay animation.
#[cfg(feature = "server")]
pub async fn award_xp(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    space_id: SpacePartition,
    action_id: String,
) -> Result<XpGainResponse> {
    let space_pk: Partition = space_id.clone().into();

    // 1. Load SpaceAction
    let action_pk = CompositePartition(space_id.clone(), action_id.clone());
    let action = SpaceAction::get(cli, &action_pk, Some(EntityType::SpaceAction))
        .await
        .map_err(|e| {
            crate::error!("award_xp: failed to load action: {e}");
            Error::SpaceActionNotFound
        })?
        .ok_or(Error::SpaceActionNotFound)?;
    let base_points = action.total_points as i64;

    // 2. Snapshot participant count
    let participants = snapshot_participant_count(cli, &space_pk).await?.max(1);

    // 3. Bump combo + streak
    let (combo_mult, _combo_streak) = bump_combo(cli, &space_pk, user_pk).await?;
    let (streak_mult, _streak_days) = bump_streak(cli, user_pk).await?;

    // 4. Compute XP
    let xp =
        (base_points as f64 * participants as f64 * combo_mult as f64 * streak_mult as f64) as i64;

    // 5. Write ledger entry
    let ledger = SpaceXpLedgerEntry::new(
        space_id.clone(),
        user_pk.clone(),
        Some(action_id.clone()),
        base_points,
        participants,
        combo_mult,
        streak_mult,
        xp,
        false,
    );
    ledger.create(cli).await.map_err(|e| {
        crate::error!("award_xp: ledger write failed: {e}");
        Error::InternalServerError("ledger write failed".into())
    })?;

    // 6. Update UserGlobalXp
    let user_id: UserPartition = user_pk.clone().into();
    let (gxp_pk, gxp_sk) = UserGlobalXp::keys(user_pk);
    let old_global = UserGlobalXp::get(cli, &gxp_pk, Some(&gxp_sk))
        .await
        .map_err(|e| {
            crate::error!("award_xp: failed to load global xp: {e}");
            Error::InternalServerError("global xp read failed".into())
        })?
        .unwrap_or_else(|| UserGlobalXp::new(user_id.clone()));
    let old_level = old_global.level;

    let mut new_global = old_global.clone();
    new_global.total_xp += xp;
    new_global.total_points += base_points;
    new_global.level = UserGlobalXp::level_from_xp(new_global.total_xp);
    new_global.updated_at = get_now_timestamp_millis();
    new_global.upsert(cli).await.map_err(|e| {
        crate::error!("award_xp: global xp upsert failed: {e}");
        Error::InternalServerError("global xp upsert failed".into())
    })?;

    // 7. Check chapter completion
    let (chapter_completed, role_upgraded) = if let Some(ref chapter_entity) = action.chapter_id {
        check_chapter_complete(cli, &space_pk, user_pk, chapter_entity).await?
    } else {
        (false, None)
    };

    // 8. Creator share (10%)
    mint_creator_share(cli, &space_pk, space_id.clone(), base_points, xp).await?;

    // 9. Find newly-unlocked DAG children
    let unlocked =
        find_unlocked_children(cli, &space_pk, user_pk, &action_id, &action).await;

    Ok(XpGainResponse {
        xp_earned: xp,
        base_points,
        participants_snapshot: participants,
        combo_multiplier: combo_mult,
        streak_multiplier: streak_mult,
        old_level,
        new_level: new_global.level,
        unlocked_actions: unlocked,
        chapter_completed,
        role_upgraded,
    })
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Reads the `SpaceCommon.participants` counter as a snapshot of
/// participant count for the XP formula.
#[cfg(feature = "server")]
pub async fn snapshot_participant_count(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<u32> {
    let space = SpaceCommon::get(cli, space_pk, Some(EntityType::SpaceCommon))
        .await
        .map_err(|e| {
            crate::error!("snapshot_participant_count: failed to load space: {e}");
            Error::SpaceNotFound
        })?
        .ok_or(Error::SpaceNotFound)?;

    Ok(space.participants.max(1) as u32)
}

/// Increments the user's in-space combo streak and returns the new
/// `(combo_multiplier, streak_count)`.  The combo resets if the last
/// completion was more than 24 hours ago.
#[cfg(feature = "server")]
pub async fn bump_combo(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) -> Result<(f32, u32)> {
    let now = get_now_timestamp_millis();
    let twenty_four_hours_ms: i64 = 24 * 60 * 60 * 1000;

    let (pk, sk) = UserSpaceCombo::keys(space_pk, user_pk);
    let existing = UserSpaceCombo::get(cli, &pk, Some(&sk))
        .await
        .map_err(|e| {
            crate::error!("bump_combo: failed to load combo: {e}");
            Error::InternalServerError("combo read failed".into())
        })?;

    let mut combo = match existing {
        Some(mut c) => {
            if now - c.last_completion_at <= twenty_four_hours_ms {
                c.current_streak_in_space += 1;
            } else {
                c.current_streak_in_space = 1;
            }
            c
        }
        None => {
            let space_id: SpacePartition = space_pk.clone().into();
            let mut c = UserSpaceCombo::new(space_id, user_pk);
            c.current_streak_in_space = 1;
            c
        }
    };

    combo.combo_multiplier = UserSpaceCombo::combo_multiplier(combo.current_streak_in_space);
    combo.last_completion_at = now;
    combo.updated_at = now;

    combo.upsert(cli).await.map_err(|e| {
        crate::error!("bump_combo: upsert failed: {e}");
        Error::InternalServerError("combo upsert failed".into())
    })?;

    Ok((combo.combo_multiplier, combo.current_streak_in_space))
}

/// Increments the user's global daily streak and returns the new
/// `(streak_multiplier, day_count)`.  If the user already submitted
/// today, the current values are returned unchanged.
#[cfg(feature = "server")]
pub async fn bump_streak(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> Result<(f32, u32)> {
    let now = get_now_timestamp_millis();
    let today = format_date_utc(now);

    let (pk, sk) = UserStreak::keys(user_pk);
    let existing = UserStreak::get(cli, &pk, Some(&sk))
        .await
        .map_err(|e| {
            crate::error!("bump_streak: failed to load streak: {e}");
            Error::InternalServerError("streak read failed".into())
        })?;

    let mut streak = match existing {
        Some(s) => s,
        None => {
            let user_id: UserPartition = user_pk.clone().into();
            UserStreak::new(user_id)
        }
    };

    if streak.last_active_date == today {
        // Already bumped today — return current values.
        let mult = UserStreak::streak_multiplier(streak.current_streak);
        return Ok((mult, streak.current_streak));
    }

    let yesterday = format_date_utc(now - 24 * 60 * 60 * 1000);
    if streak.last_active_date == yesterday {
        streak.current_streak += 1;
        if streak.current_streak > streak.longest_streak {
            streak.longest_streak = streak.current_streak;
        }
    } else {
        streak.current_streak = 1;
    }

    streak.last_active_date = today;
    streak.updated_at = now;

    streak.upsert(cli).await.map_err(|e| {
        crate::error!("bump_streak: upsert failed: {e}");
        Error::InternalServerError("streak upsert failed".into())
    })?;

    let mult = UserStreak::streak_multiplier(streak.current_streak);
    Ok((mult, streak.current_streak))
}

/// Mints a 10% creator share: upserts `SpaceCreatorEarnings` and, if
/// the recipient is a `User`, also bumps their `UserGlobalXp`.
#[cfg(feature = "server")]
async fn mint_creator_share(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    space_id: SpacePartition,
    base_points: i64,
    total_xp: i64,
) -> Result<()> {
    let creator_xp = (total_xp as f64 * 0.10).round() as i64;
    let creator_points = (base_points as f64 * 0.10).round() as i64;

    if creator_xp == 0 && creator_points == 0 {
        return Ok(());
    }

    // Load the space to discover the author (creator).
    let space = SpaceCommon::get(cli, space_pk, Some(EntityType::SpaceCommon))
        .await
        .map_err(|e| {
            crate::error!("mint_creator_share: failed to load space: {e}");
            Error::InternalServerError("space read failed".into())
        })?
        .ok_or(Error::SpaceNotFound)?;

    let recipient = CreatorRecipient::User(space.user_pk.clone().into());

    // Upsert SpaceCreatorEarnings
    let (ce_pk, ce_sk) = SpaceCreatorEarnings::keys(space_pk);
    let mut earnings = SpaceCreatorEarnings::get(cli, &ce_pk, Some(&ce_sk))
        .await
        .map_err(|e| {
            crate::error!("mint_creator_share: failed to load earnings: {e}");
            Error::InternalServerError("creator earnings read failed".into())
        })?
        .unwrap_or_else(|| SpaceCreatorEarnings::new(space_id.clone(), recipient.clone()));

    earnings.total_xp += creator_xp;
    earnings.total_points += creator_points;
    earnings.updated_at = get_now_timestamp_millis();

    earnings.upsert(cli).await.map_err(|e| {
        crate::error!("mint_creator_share: earnings upsert failed: {e}");
        Error::InternalServerError("creator earnings upsert failed".into())
    })?;

    // Write creator ledger entry
    let creator_ledger = SpaceXpLedgerEntry::new(
        space_id,
        space.user_pk.clone(),
        None,
        creator_points,
        0,
        1.0,
        1.0,
        creator_xp,
        true,
    );
    creator_ledger.create(cli).await.map_err(|e| {
        crate::error!("mint_creator_share: creator ledger write failed: {e}");
        Error::InternalServerError("creator ledger failed".into())
    })?;

    // If recipient is a User, also bump their UserGlobalXp.
    if let CreatorRecipient::User(ref user_sub) = recipient {
        let creator_user_pk: Partition = user_sub.clone().into();
        let (gxp_pk, gxp_sk) = UserGlobalXp::keys(&creator_user_pk);
        let mut creator_global = UserGlobalXp::get(cli, &gxp_pk, Some(&gxp_sk))
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| UserGlobalXp::new(user_sub.clone()));

        creator_global.total_xp += creator_xp;
        creator_global.total_points += creator_points;
        creator_global.level = UserGlobalXp::level_from_xp(creator_global.total_xp);
        creator_global.updated_at = get_now_timestamp_millis();
        creator_global.upsert(cli).await.map_err(|e| {
            crate::error!("mint_creator_share: creator global xp upsert failed: {e}");
            Error::InternalServerError("creator global xp failed".into())
        })?;
    }

    Ok(())
}

/// V1 placeholder: attempts to find DAG children whose all parents are
/// now cleared.  Returns an empty vec on any error to avoid blocking
/// the main XP award pipeline.
#[cfg(feature = "server")]
async fn find_unlocked_children(
    _cli: &aws_sdk_dynamodb::Client,
    _space_pk: &Partition,
    _user_pk: &Partition,
    _action_id: &str,
    _action: &SpaceAction,
) -> Vec<String> {
    // V1: return empty — the Quest Map re-fetches the full map on each
    // navigation so newly-unlocked children are discovered client-side.
    // Phase 8 may implement a DAG walk here.
    Vec::new()
}

/// Formats a unix-millis timestamp into a `YYYY-MM-DD` UTC date string.
#[cfg(feature = "server")]
fn format_date_utc(millis: i64) -> String {
    let secs = millis / 1000;
    let days_since_epoch = secs / 86400;
    // Civil date from days since 1970-01-01 using a well-known algorithm.
    let z = days_since_epoch + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", y, m, d)
}
