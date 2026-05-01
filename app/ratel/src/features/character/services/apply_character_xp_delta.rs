use crate::common::*;
use crate::features::activity::models::SpaceScore;
use crate::features::character::leveling;
use crate::features::character::models::{CharacterXp, CharacterXpSource};

/// Apply the change in `SpaceScore.total_score` for a (user, space) into the
/// user's CharacterXp. Idempotent under stream replay: a re-delivered MODIFY
/// event with the same `score.total_score` produces zero delta and is a no-op.
///
/// `score`: the *new* SpaceScore (post-MODIFY image, or post-INSERT image).
pub async fn apply_character_xp_delta(
    cli: &aws_sdk_dynamodb::Client,
    score: SpaceScore,
) -> crate::common::Result<()> {
    let user_pk: Partition = match score.user_pk.clone() {
        crate::features::activity::types::AuthorPartition::User(id) => Partition::User(id),
        crate::features::activity::types::AuthorPartition::Team(id) => Partition::Team(id),
        crate::features::activity::types::AuthorPartition::Unknown => {
            tracing::warn!(
                space_pk = %score.space_pk,
                "apply_character_xp_delta: unknown author partition; skipping"
            );
            return Ok(());
        }
    };
    let space_pk_str = match &score.space_pk {
        Partition::Space(s) => s.clone(),
        _ => {
            tracing::warn!(
                user_pk = %user_pk,
                space_pk = %score.space_pk,
                "apply_character_xp_delta: unexpected space_pk variant; skipping"
            );
            return Ok(());
        }
    };

    let (src_pk, src_sk) = CharacterXpSource::keys(&user_pk, &space_pk_str);
    let last_seen = CharacterXpSource::get(cli, &src_pk, Some(&src_sk))
        .await?
        .map(|r| r.last_seen_score)
        .unwrap_or(0);

    let new_total = score.total_score;
    let delta = new_total - last_seen;

    if delta == 0 {
        // Replay; nothing to do.
        return Ok(());
    }

    if delta < 0 {
        // Score decreased — spec Q3 says XP is monotonic. Don't debit, but
        // do advance last_seen so we don't re-apply the same negative delta.
        tracing::warn!(
            user_pk = %user_pk,
            space = %space_pk_str,
            last_seen,
            new_total,
            "negative SpaceScore delta — last_seen advanced, CharacterXp unchanged"
        );
        let new_src = CharacterXpSource::new(user_pk.clone(), space_pk_str.clone(), new_total);
        let _ = new_src.create(cli).await;
        return Ok(());
    }

    // Read current CharacterXp, default to a fresh row.
    let (xp_pk, xp_sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(cli, &xp_pk, Some(&xp_sk)).await?;
    let xp = xp.unwrap_or_else(|| CharacterXp::new(user_pk.clone()));

    let new_total_xp = xp.total_xp + delta;
    let new_level = leveling::level_from_xp(new_total_xp);
    let new_sp_granted = leveling::total_sp_granted(new_level);
    let now = crate::common::utils::time::get_now_timestamp_millis();

    if xp.total_xp == 0 && xp.created_at == 0 {
        // First-ever XP for this user; insert the row.
        let mut fresh = CharacterXp::new(user_pk.clone());
        fresh.total_xp = new_total_xp;
        fresh.level = new_level;
        fresh.total_sp_granted = new_sp_granted;
        fresh.updated_at = now;
        fresh.create(cli).await?;
    } else {
        CharacterXp::updater(&xp_pk, &xp_sk)
            .with_total_xp(new_total_xp)
            .with_level(new_level)
            .with_total_sp_granted(new_sp_granted)
            .with_updated_at(now)
            .execute(cli)
            .await?;
    }

    // Record last_seen so future deltas are correct.
    let new_src = CharacterXpSource::new(user_pk.clone(), space_pk_str, new_total);
    new_src.create(cli).await?;

    if new_level != xp.level {
        tracing::info!(
            user_pk = %user_pk,
            old_level = xp.level,
            new_level,
            new_sp = new_sp_granted - xp.total_sp_granted,
            "character level up"
        );
    }

    Ok(())
}
