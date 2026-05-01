use super::helpers::*;

#[tokio::test]
async fn test_level_up_money_tree_l1_success() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
        response_type: crate::features::character::dto::CharacterResponse,
    };
    assert_eq!(status, 200, "{:?}", body);
    let mt = body
        .skills
        .iter()
        .find(|s| matches!(s.skill_id, crate::features::character::types::SkillId::MoneyTree))
        .unwrap();
    assert_eq!(mt.level, 1);
    assert_eq!(mt.multiplier_permille, 1050);
    assert_eq!(body.unspent_sp, 0); // L1 char = 5 SP, spent 5
    assert_eq!(body.total_sp_spent, 5);
}

#[tokio::test]
async fn test_level_up_insufficient_sp_rejected() {
    let ctx = TestContext::setup().await;
    // Brand new user has 5 SP. Buying MoneyTree L1 (5) is fine; trying L2 (cost 9) without more XP should fail.
    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(status, 400, "second level-up without XP should be rejected");
}

#[tokio::test]
async fn test_level_up_unknown_skill_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/no_such_skill/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_level_up_v2_skill_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/influencer/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(status, 400, "v2 skill must be gated");
}

#[tokio::test]
async fn test_level_up_max_level_rejected() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    // Pump in enough XP to easily afford max-out: 230 SP needs char L46, so 7M XP is plenty.
    award_xp(&ctx, &user_pk, 8_000_000).await;
    for _ in 0..10 {
        let (status, _, _) = crate::test_post! {
            app: ctx.app.clone(),
            path: "/api/me/skills/money_tree/level-up",
            headers: ctx.test_user.1.clone(),
            body: {}
        };
        assert_eq!(status, 200);
    }
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };
    assert_eq!(status, 400, "11th level-up must be rejected");
}

#[tokio::test]
async fn test_level_up_unauthenticated_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        body: {}
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_money_tree_boosts_user_reward_amount() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    // Buy MoneyTree L1.
    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };

    // Fabricate a SpaceReward and call award directly.
    use crate::features::spaces::space_common::models::space_reward::SpaceReward;

    let space_id = SpacePartition("space-fixture".to_string());
    let reward = SpaceReward::new(
        space_id.clone(),
        "action-1".into(),
        RewardUserBehavior::RespondPoll,
        "test reward".into(),
        1,      // credits
        10_000, // point
        RewardPeriod::Once,
        RewardCondition::None,
    );
    reward.create(&ctx.ddb).await.unwrap();

    let user_reward = SpaceReward::award(&ctx.ddb, &reward, user_pk.clone(), None)
        .await
        .unwrap();

    // L1 = +5%, so 10_000 → 10_500
    assert_eq!(user_reward.total_points, 10_500, "+5% boost expected");
}

#[tokio::test]
async fn test_ranker_boosts_additional_score() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/ranker/level-up",
        headers: ctx.test_user.1.clone(),
        body: {}
    };

    use crate::features::activity::models::SpaceActivity;
    use crate::features::activity::types::{AuthorPartition, SpaceActivityData};
    use crate::features::spaces::pages::actions::types::SpaceActionType;

    let author: AuthorPartition = AuthorPartition::from(user_pk);

    let activity = SpaceActivity::new_with_dedup(
        &ctx.ddb,
        SpacePartition("space-fixture".into()),
        author,
        "action-1".into(),
        SpaceActionType::Poll,
        SpaceActivityData::default(),
        100, // base
        50,  // additional, boosted
        "u".into(),
        "".into(),
        "dedup-1".into(),
    )
    .await;

    // Ranker L1 = +5% → 50 × 1.05 = 53 (rounded). total_score = 100 + 53 = 153.
    assert_eq!(activity.additional_score, 53);
    assert_eq!(activity.total_score, 153);
    assert_eq!(activity.base_score, 100, "base unchanged");
}
