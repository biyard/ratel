use super::helpers::*;
use crate::features::character::models::CharacterXp;
use crate::features::character::services::apply_character_xp_delta;

#[tokio::test]
async fn test_apply_xp_first_score_inserts_xp_row() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    let score = make_score(&user_pk, "space-a", 5_000);

    apply_character_xp_delta(&ctx.ddb, score).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("xp row created");
    assert_eq!(xp.total_xp, 5_000);
    // cumulative_xp(4) = 220·3·4·7/6 = 3_080 < 5_000 < 6_600 = L5
    assert_eq!(xp.level, 4);
    assert_eq!(xp.total_sp_granted, 5 * 4);
    assert_eq!(xp.total_sp_spent, 0);
}

#[tokio::test]
async fn test_apply_xp_replay_idempotent() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    let score = make_score(&user_pk, "space-a", 5_000);

    apply_character_xp_delta(&ctx.ddb, score.clone()).await.unwrap();
    apply_character_xp_delta(&ctx.ddb, score).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000, "replay must not double-count");
}

#[tokio::test]
async fn test_apply_xp_increment_uses_delta() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 1_000)).await.unwrap();
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 1_500)).await.unwrap();
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 5_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000);
}

#[tokio::test]
async fn test_apply_xp_negative_delta_does_not_debit() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 5_000)).await.unwrap();
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 4_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000, "monotonic — negative deltas dropped");
}

#[tokio::test]
async fn test_apply_xp_level_up_grants_sp() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    // First: small score, ends at L1 (220 needed for L2).
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 100)).await.unwrap();
    // Then: enough to cross many levels.
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "s", 100_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert!(xp.level >= 12);
    assert_eq!(xp.total_sp_granted, 5 * xp.level);
}

#[tokio::test]
async fn test_apply_xp_per_space_independent() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "space-a", 1_000)).await.unwrap();
    apply_character_xp_delta(&ctx.ddb, make_score(&user_pk, "space-b", 2_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(&ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 3_000, "delta from each space accumulates");
}

#[tokio::test]
async fn test_get_character_unauthenticated_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/me/character",
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_get_character_brand_new_user_returns_default() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/me/character",
        headers: ctx.test_user.1.clone(),
        response_type: crate::features::character::dto::CharacterResponse,
    };
    assert_eq!(status, 200, "brand new user: {:?}", body);
    assert_eq!(body.total_xp, 0);
    assert_eq!(body.level, 1);
    assert_eq!(body.unspent_sp, 5);
    assert_eq!(body.skills.len(), 4);
    let mt = body
        .skills
        .iter()
        .find(|s| matches!(s.skill_id, crate::features::character::types::SkillId::MoneyTree))
        .unwrap();
    assert_eq!(mt.level, 0);
    assert_eq!(mt.next_level_cost, Some(5));
}

#[tokio::test]
async fn test_get_character_after_xp_delta() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    let score = make_score(&user_pk, "space-a", 5_000);
    crate::features::character::services::apply_character_xp_delta(&ctx.ddb, score)
        .await
        .unwrap();

    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/me/character",
        headers: ctx.test_user.1.clone(),
        response_type: crate::features::character::dto::CharacterResponse,
    };
    assert_eq!(body.total_xp, 5_000);
    assert_eq!(body.level, 4);
    assert_eq!(body.unspent_sp, 20);
}

#[tokio::test]
async fn test_get_public_character_returns_level_only() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    crate::features::character::services::apply_character_xp_delta(
        &ctx.ddb,
        make_score(&user_pk, "s", 5_000),
    )
    .await
    .unwrap();

    let username = ctx.test_user.0.username.clone();
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/users/{}/character", username),
        response_type: crate::features::character::dto::PublicCharacterResponse,
    };
    assert_eq!(status, 200, "{:?}", body);
    assert_eq!(body.level, 4);
}

#[tokio::test]
async fn test_get_public_character_unknown_user_404() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/users/no-such-user-asdf/character",
    };
    assert_eq!(status, 404);
}
