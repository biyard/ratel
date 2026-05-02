use crate::common::*;
use crate::features::character::dto::CharacterResponse;

#[get("/api/me/character", user: crate::features::auth::User)]
pub async fn get_character_handler() -> Result<CharacterResponse> {
    use crate::features::character::models::{CharacterSkill, CharacterXp};

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (xp_pk, xp_sk) = CharacterXp::user_keys(&user.pk);

    // Fan out the two reads in parallel — one CharacterXp single-row + the
    // entire CharacterSkill collection via begins_with(sk) Query. ~1 RTT total.
    let (xp_res, skill_rows) = tokio::try_join!(
        CharacterXp::get(cli, &xp_pk, Some(&xp_sk)),
        CharacterSkill::list_for_user(cli, &user.pk),
    )?;

    let xp = xp_res.unwrap_or_else(|| CharacterXp::new(user.pk.clone()));
    let skills = CharacterSkill::levels_by_id(&skill_rows);

    Ok(CharacterResponse::from_parts(&xp, skills))
}
