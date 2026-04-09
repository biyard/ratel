//! Phase 2 + Phase 4 integration tests for the gamification data model.
//!
//! These tests exercise the DynamoEntity round-trip for every new model
//! and verify that `SpaceAction` stays backward-compatible with records
//! that predate the `chapter_id` / `depends_on` fields.

use super::*;

use crate::common::types::{EntityType, Partition, SpacePartition, SpaceUserRole, UserPartition};
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::{
    ChapterBenefit, CreatorRecipient, SpaceChapter, SpaceCreatorEarnings, SpaceXpLedgerEntry,
    UserGlobalXp, UserSpaceCombo, UserStreak,
};
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::actions::types::SpaceActionType;

fn unique_id(prefix: &str) -> String {
    format!("{prefix}-{}", uuid::Uuid::new_v4())
}

#[tokio::test]
async fn test_create_chapter_entity() {
    let ctx = TestContext::setup().await;

    let space_id = SpacePartition(unique_id("space"));
    let chapter_id = unique_id("chapter");

    let chapter = SpaceChapter::new(
        space_id.clone(),
        chapter_id.clone(),
        0,
        "Qualify".to_string(),
        SpaceUserRole::Candidate,
        ChapterBenefit::RoleUpgradeAndXp(SpaceUserRole::Participant),
    );

    chapter
        .create(&ctx.ddb)
        .await
        .expect("SpaceChapter::create should succeed");

    let (pk, sk) = SpaceChapter::keys(&space_id.clone().into(), &chapter_id);
    let loaded = SpaceChapter::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("SpaceChapter::get should succeed")
        .expect("chapter should exist");

    assert_eq!(loaded.name, "Qualify");
    assert_eq!(loaded.order, 0);
    assert!(matches!(
        loaded.completion_benefit,
        ChapterBenefit::RoleUpgradeAndXp(SpaceUserRole::Participant)
    ));
}

#[test]
fn test_chapter_benefit_enum_serializes() {
    let cases = [
        ChapterBenefit::XpOnly,
        ChapterBenefit::RoleUpgradeTo(SpaceUserRole::Candidate),
        ChapterBenefit::RoleUpgradeAndXp(SpaceUserRole::Participant),
    ];
    for original in cases {
        let json = serde_json::to_string(&original).expect("serialize");
        let decoded: ChapterBenefit = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, original, "roundtrip {:?}", original);
    }
}

#[tokio::test]
async fn test_xp_ledger_entry_entity() {
    let ctx = TestContext::setup().await;

    let space_id = SpacePartition(unique_id("space"));
    let user_pk = Partition::User(unique_id("user"));

    let entry = SpaceXpLedgerEntry::new(
        space_id.clone(),
        user_pk.clone(),
        Some("action-42".to_string()),
        200,   // base_points
        1_200, // participants_snapshot
        2.0,   // combo_multiplier
        1.15,  // streak_multiplier
        552_000,
        false,
    );

    entry
        .create(&ctx.ddb)
        .await
        .expect("SpaceXpLedgerEntry::create should succeed");

    // Query by the primary pk (space partition).
    let space_pk: Partition = space_id.clone().into();
    let (items, _bookmark) = SpaceXpLedgerEntry::query(
        &ctx.ddb,
        space_pk.to_string(),
        SpaceXpLedgerEntry::opt().limit(10),
    )
    .await
    .expect("query should succeed");

    assert!(
        items.iter().any(|e| e.action_id.as_deref() == Some("action-42")),
        "ledger entry should be retrievable under the space pk"
    );
}

#[tokio::test]
async fn test_user_global_xp_upsert() {
    let ctx = TestContext::setup().await;

    let user_id = UserPartition(unique_id("user"));
    let user_pk: Partition = user_id.clone().into();

    let mut xp = UserGlobalXp::new(user_id.clone());
    xp.total_xp = 4_500;
    xp.total_points = 4_500;
    xp.level = UserGlobalXp::level_from_xp(4_500);
    xp.upsert(&ctx.ddb)
        .await
        .expect("UserGlobalXp::upsert should succeed");

    let (pk, sk) = UserGlobalXp::keys(&user_pk);
    let loaded = UserGlobalXp::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("UserGlobalXp should exist after upsert");

    assert_eq!(loaded.total_xp, 4_500);
    assert_eq!(loaded.level, UserGlobalXp::level_from_xp(4_500));

    // Upsert again with a larger total — the row should update in place.
    let mut bumped = loaded.clone();
    bumped.total_xp = 9_000;
    bumped.total_points = 9_000;
    bumped.level = UserGlobalXp::level_from_xp(9_000);
    bumped.updated_at = get_now_timestamp_millis();
    bumped
        .upsert(&ctx.ddb)
        .await
        .expect("second upsert should succeed");

    let reloaded = UserGlobalXp::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("UserGlobalXp should still exist");
    assert_eq!(reloaded.total_xp, 9_000);
}

#[tokio::test]
async fn test_user_streak_increments() {
    let ctx = TestContext::setup().await;

    let user_id = UserPartition(unique_id("user"));
    let user_pk: Partition = user_id.clone().into();

    let mut streak = UserStreak::new(user_id.clone());
    streak.current_streak = 1;
    streak.longest_streak = 1;
    streak.last_active_date = "2026-04-08".to_string();
    streak
        .create(&ctx.ddb)
        .await
        .expect("UserStreak::create should succeed");

    let (pk, sk) = UserStreak::keys(&user_pk);
    let loaded = UserStreak::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("UserStreak should exist");
    assert_eq!(loaded.current_streak, 1);

    // Bump the streak the next day.
    let mut bumped = loaded.clone();
    bumped.current_streak = 2;
    bumped.longest_streak = 2;
    bumped.last_active_date = "2026-04-09".to_string();
    bumped.updated_at = get_now_timestamp_millis();
    bumped
        .upsert(&ctx.ddb)
        .await
        .expect("upsert should succeed");

    let reloaded = UserStreak::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("UserStreak should still exist");
    assert_eq!(reloaded.current_streak, 2);
    assert_eq!(reloaded.longest_streak, 2);
    assert_eq!(UserStreak::streak_multiplier(2), 1.0);
    assert_eq!(UserStreak::streak_multiplier(7), 1.15);
}

#[tokio::test]
async fn test_user_space_combo_roundtrip() {
    let ctx = TestContext::setup().await;

    let space_id = SpacePartition(unique_id("space"));
    let user_pk = Partition::User(unique_id("user"));

    let space_pk: Partition = space_id.clone().into();
    let combo = UserSpaceCombo::new(space_id, &user_pk);
    combo
        .create(&ctx.ddb)
        .await
        .expect("UserSpaceCombo::create should succeed");

    let (pk, sk) = UserSpaceCombo::keys(&space_pk, &user_pk);
    let loaded = UserSpaceCombo::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("UserSpaceCombo should exist");

    assert_eq!(loaded.current_streak_in_space, 0);
    assert_eq!(loaded.combo_multiplier, 1.0);
    assert_eq!(UserSpaceCombo::combo_multiplier(3), 2.0);
    assert_eq!(UserSpaceCombo::combo_multiplier(5), 3.0);
}

#[tokio::test]
async fn test_space_creator_earnings_roundtrip() {
    let ctx = TestContext::setup().await;

    let space_id = SpacePartition(unique_id("space"));
    let recipient = CreatorRecipient::User(UserPartition(unique_id("user")));

    let earnings = SpaceCreatorEarnings::new(space_id.clone(), recipient.clone());
    earnings
        .create(&ctx.ddb)
        .await
        .expect("SpaceCreatorEarnings::create should succeed");

    let space_pk: Partition = space_id.into();
    let (pk, sk) = SpaceCreatorEarnings::keys(&space_pk);
    let loaded = SpaceCreatorEarnings::get(&ctx.ddb, &pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("SpaceCreatorEarnings should exist");
    assert_eq!(loaded.recipient, recipient);
    assert_eq!(loaded.total_xp, 0);
}

#[tokio::test]
async fn test_space_action_with_chapter_id() {
    let ctx = TestContext::setup().await;

    let space_id = SpacePartition(unique_id("space"));
    let action_id = unique_id("action");

    // Create an action with chapter_id set — verifies forward-compat.
    let mut action = SpaceAction::new(space_id.clone(), action_id.clone(), SpaceActionType::Poll);
    action.title = "Chapter-bound action".to_string();
    action.chapter_id = Some("qualify".to_string().into());
    action.depends_on = vec!["parent-1".to_string(), "parent-2".to_string()];
    action
        .create(&ctx.ddb)
        .await
        .expect("SpaceAction::create should succeed");

    let sk = action.sk.clone();
    let loaded = SpaceAction::get(&ctx.ddb, &action.pk, Some(&sk))
        .await
        .expect("get should succeed")
        .expect("SpaceAction should exist");
    assert_eq!(loaded.title, "Chapter-bound action");
    assert!(loaded.chapter_id.is_some(), "chapter_id should roundtrip");
    assert_eq!(loaded.depends_on.len(), 2);

    // Create another action WITHOUT chapter_id / depends_on —
    // verifies backward compat: existing records still insert cleanly.
    let space_id_b = SpacePartition(unique_id("space"));
    let action_id_b = unique_id("action");
    let action_b = SpaceAction::new(space_id_b.clone(), action_id_b.clone(), SpaceActionType::Poll);
    assert!(action_b.chapter_id.is_none());
    assert!(action_b.depends_on.is_empty());
    action_b
        .create(&ctx.ddb)
        .await
        .expect("SpaceAction without chapter_id should still create");
}

// ── Phase 4: access control unit tests ───────────────────────────────────────

use crate::features::spaces::pages::actions::can_execute_space_action;
use crate::common::types::SpaceStatus;

fn make_chapter(actor_role: SpaceUserRole) -> SpaceChapter {
    SpaceChapter {
        actor_role,
        ..Default::default()
    }
}

#[test]
fn test_can_execute_space_action_role_match() {
    // Creator always passes (regardless of deps / prior chapter state).
    let chapter = make_chapter(SpaceUserRole::Participant);
    assert!(
        can_execute_space_action(
            SpaceUserRole::Creator,
            &chapter,
            false,
            false,
            Some(SpaceStatus::Ongoing),
            false,
        ),
        "creator should bypass all gates"
    );

    // Participant meets Participant chapter.
    assert!(can_execute_space_action(
        SpaceUserRole::Participant,
        &chapter,
        true,
        true,
        Some(SpaceStatus::Ongoing),
        false,
    ));

    // Candidate does NOT meet Participant chapter.
    assert!(!can_execute_space_action(
        SpaceUserRole::Candidate,
        &chapter,
        true,
        true,
        Some(SpaceStatus::Ongoing),
        false,
    ));

    // Viewer is always blocked.
    assert!(!can_execute_space_action(
        SpaceUserRole::Viewer,
        &chapter,
        true,
        true,
        Some(SpaceStatus::Ongoing),
        false,
    ));
}

#[test]
fn test_can_execute_space_action_dag_gate() {
    let chapter = make_chapter(SpaceUserRole::Participant);

    // DAG deps not met → blocked even with correct role and prior chapters.
    assert!(!can_execute_space_action(
        SpaceUserRole::Participant,
        &chapter,
        false, // deps NOT met
        true,
        Some(SpaceStatus::Ongoing),
        false,
    ));

    // DAG deps met → allowed.
    assert!(can_execute_space_action(
        SpaceUserRole::Participant,
        &chapter,
        true,
        true,
        Some(SpaceStatus::Ongoing),
        false,
    ));
}

#[test]
fn test_can_execute_space_action_prior_chapter_gate() {
    let chapter = make_chapter(SpaceUserRole::Participant);

    // Prior chapters not complete → blocked even with correct role and deps met.
    assert!(!can_execute_space_action(
        SpaceUserRole::Participant,
        &chapter,
        true,
        false, // prior chapters NOT complete
        Some(SpaceStatus::Ongoing),
        false,
    ));

    // Prior chapters complete → allowed.
    assert!(can_execute_space_action(
        SpaceUserRole::Participant,
        &chapter,
        true,
        true,
        Some(SpaceStatus::Ongoing),
        false,
    ));
}

// ── Phase 4: HTTP integration test for get_quest_map ─────────────────────────

#[tokio::test]
async fn test_get_quest_map_response() {
    let ctx = TestContext::setup().await;

    // Create a space via the HTTP API.
    let (status, _, space_body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces",
        headers: ctx.test_user.1.clone(),
        body: {
            "req": {
                "title": "Quest Map Test Space",
                "description": "phase-4 test",
                "logo": ""
            }
        }
    };
    assert_eq!(status, 200, "create space: {:?}", space_body);
    let space_pk_str = space_body["pk"]
        .as_str()
        .expect("space pk should be a string");
    // space_pk_str includes "SPACE#" prefix; strip it for the SubPartition path param.
    let space_id = space_pk_str
        .trim_start_matches("SPACE#");

    // GET /api/spaces/{space_id}/quest-map — expect 200 even with no chapters.
    let (status, _, quest_map_body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/quest-map", space_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "get_quest_map: {:?}", quest_map_body);

    // Response must contain the expected top-level keys.
    assert!(
        quest_map_body["chapters"].is_array(),
        "chapters should be an array: {:?}",
        quest_map_body
    );
    assert!(
        quest_map_body["current_user_state"].is_object(),
        "current_user_state should be an object: {:?}",
        quest_map_body
    );
    // With no chapters created, the array should be empty.
    assert_eq!(
        quest_map_body["chapters"].as_array().unwrap().len(),
        0,
        "no chapters should exist yet: {:?}",
        quest_map_body
    );
}
