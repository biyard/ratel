use crate::common::*;
use crate::features::character::dto::CharacterResponse;
use crate::features::character::leveling;
use crate::features::character::types::{CharacterError, SkillId};

#[post("/api/me/skills/{skill_id}/level-up", user: crate::features::auth::User)]
pub async fn level_up_handler(skill_id: String) -> Result<CharacterResponse> {
    use crate::features::character::models::{CharacterSkill, CharacterXp};

    let id = SkillId::from_str(&skill_id).ok_or(CharacterError::SkillNotFound)?;
    if !id.is_mvp() {
        return Err(CharacterError::SkillNotReleased.into());
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (xp_pk, xp_sk) = CharacterXp::user_keys(&user.pk);
    let mut xp = CharacterXp::get(cli, &xp_pk, Some(&xp_sk))
        .await?
        .unwrap_or_else(|| CharacterXp::new(user.pk.clone()));

    let cur_level = CharacterSkill::level_or_zero(cli, &user.pk, id).await?;
    let cost = leveling::skill_cost_next(cur_level).ok_or(CharacterError::AlreadyMaxLevel)?;
    if xp.unspent_sp() < cost {
        return Err(CharacterError::InsufficientSp.into());
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Persist new skill level (insert if first, update otherwise).
    let (sk_pk, sk_sk) = CharacterSkill::keys(&user.pk, id);
    if cur_level == 0 {
        let row = CharacterSkill {
            pk: sk_pk.clone(),
            sk: sk_sk.clone(),
            level: 1,
            created_at: now,
            updated_at: now,
        };
        row.create(cli).await?;
    } else {
        CharacterSkill::updater(&sk_pk, &sk_sk)
            .with_level(cur_level + 1)
            .with_updated_at(now)
            .execute(cli)
            .await?;
    }

    // Bump total_sp_spent (and persist a fresh CharacterXp row if this is
    // the user's first SP spend — they may not have had a row yet because
    // apply_character_xp_delta only inserts on first XP, not first action).
    xp.total_sp_spent += cost;
    if xp.created_at == 0 {
        xp.created_at = now;
        xp.updated_at = now;
        xp.create(cli).await?;
    } else {
        CharacterXp::updater(&xp_pk, &xp_sk)
            .with_total_sp_spent(xp.total_sp_spent)
            .with_updated_at(now)
            .execute(cli)
            .await?;
    }

    // Re-read assembled state for the response. Two parallel reads (~1 RTT).
    let (xp_res, skill_rows) = tokio::try_join!(
        CharacterXp::get(cli, &xp_pk, Some(&xp_sk)),
        CharacterSkill::list_for_user(cli, &user.pk),
    )?;
    let xp = xp_res.ok_or_else(|| Error::NotFound("character row missing after spend".into()))?;
    let skills = CharacterSkill::levels_by_id(&skill_rows);

    Ok(CharacterResponse::from_parts(&xp, skills))
}
