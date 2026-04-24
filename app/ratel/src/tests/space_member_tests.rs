use super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility};

// Seed a public space owned by the test user (→ Creator role) plus `count`
// SpaceParticipant rows whose display_name and username are injected by the
// caller. Records are written directly to DDB instead of going through the
// HTTP join flow to keep the 100-participant setup cheap (~seconds vs
// ~minutes through the API).
async fn seed_space_with_participants(
    ctx: &TestContext,
    participants: Vec<(String, String)>, // (display_name, username)
) -> String {
    let space_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(space_id.clone());

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Ongoing);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = ctx.test_user.0.pk.clone();
    space.author_display_name = ctx.test_user.0.display_name.clone();
    space.author_profile_url = ctx.test_user.0.profile_url.clone();
    space.author_username = ctx.test_user.0.username.clone();
    space.create(&ctx.ddb).await.expect("create space");

    let post = crate::features::posts::models::Post {
        pk: post_pk.clone(),
        sk: EntityType::Post,
        title: "Member Test".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    for (i, (display_name, username)) in participants.into_iter().enumerate() {
        let user_pk = Partition::User(format!("member-{}-{}", space_id, i));
        let mut p = SpaceParticipant::new_non_anonymous(
            space_pk.clone(),
            crate::common::models::auth::User {
                pk: user_pk,
                display_name,
                username,
                profile_url: String::new(),
                user_type: crate::common::types::UserType::Individual,
                ..Default::default()
            },
        );
        // Make sure the created_at ordering is deterministic across the batch
        // so tests that assert "first 50" behave predictably.
        p.created_at = now + i as i64;
        p.create(&ctx.ddb).await.expect("create participant");
    }

    space_id
}

fn url(space_id: &str, query: Option<&str>) -> String {
    match query {
        None => format!("/api/spaces/{}/members", space_id),
        Some(q) => format!("/api/spaces/{}/members?query={}", space_id, q),
    }
}

#[tokio::test]
async fn test_empty_query_returns_first_page() {
    let ctx = TestContext::setup().await;
    let participants = (0..100)
        .map(|i| (format!("User {:03}", i), format!("user_{:03}", i)))
        .collect();
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, None),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "list: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(
        items.len(),
        50,
        "empty-query response is page-capped at 50: got {}",
        items.len()
    );
}

#[tokio::test]
async fn test_username_prefix_uses_gsi3_fast_path() {
    let ctx = TestContext::setup().await;
    // Mix of usernames: 3 "john*", 2 "jordan*", and 95 unrelated.
    let mut participants = vec![
        ("John Doe".to_string(), "john_doe".to_string()),
        ("Johnny Smith".to_string(), "johnny_smith".to_string()),
        ("John Appleseed".to_string(), "john_appleseed".to_string()),
        ("Jordan A".to_string(), "jordan_a".to_string()),
        ("Jordan B".to_string(), "jordan_b".to_string()),
    ];
    for i in 0..95 {
        participants.push((format!("Filler {}", i), format!("filler_{:03}", i)));
    }
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("john")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(
        items.len(),
        3,
        "exactly 3 usernames start with john: got {:?}",
        items
    );
    for it in items {
        let u = it["username"].as_str().unwrap();
        assert!(u.starts_with("john"), "unexpected username: {u}");
    }
}

#[tokio::test]
async fn test_display_name_prefix_via_scan_fallback() {
    let ctx = TestContext::setup().await;
    // display_name starts with "Alice" but username is transliterated/aliased,
    // so the GSI3 fast path MUST miss and the scan fallback has to fill in.
    let mut participants = vec![
        ("Alice Park".to_string(), "apark42".to_string()),
        ("Alice Johnson".to_string(), "aj".to_string()),
    ];
    for i in 0..50 {
        participants.push((format!("Filler {}", i), format!("filler_{:03}", i)));
    }
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("alice")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    let names: Vec<&str> = items.iter().map(|i| i["display_name"].as_str().unwrap()).collect();
    assert_eq!(names.len(), 2, "expected 2 Alice* matches via fallback: {:?}", names);
    assert!(names.contains(&"Alice Park"));
    assert!(names.contains(&"Alice Johnson"));
}

#[tokio::test]
async fn test_non_ascii_display_name_prefix() {
    let ctx = TestContext::setup().await;
    // Korean display_names paired with transliterated usernames — the
    // realistic case for why the scan fallback exists at all.
    let mut participants = vec![
        ("김철수".to_string(), "kim_cs".to_string()),
        ("김영희".to_string(), "kim_yh".to_string()),
        ("박민수".to_string(), "park_ms".to_string()),
    ];
    for i in 0..30 {
        participants.push((format!("Filler {}", i), format!("filler_{:03}", i)));
    }
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("김")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    let names: Vec<&str> = items.iter().map(|i| i["display_name"].as_str().unwrap()).collect();
    assert_eq!(names.len(), 2, "expected 2 김* matches: {:?}", names);
    assert!(names.contains(&"김철수"));
    assert!(names.contains(&"김영희"));
}

#[tokio::test]
async fn test_case_insensitive_prefix() {
    let ctx = TestContext::setup().await;
    let participants = vec![
        ("John Doe".to_string(), "john_doe".to_string()),
        ("Johnny".to_string(), "johnny".to_string()),
    ];
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("JOHN")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(items.len(), 2, "uppercase query should still match: {:?}", items);
}

#[tokio::test]
async fn test_result_cap_at_20() {
    let ctx = TestContext::setup().await;
    // 30 usernames starting with "bob" should still get capped at 20.
    let participants = (0..30)
        .map(|i| (format!("Bob {:02}", i), format!("bob_{:02}", i)))
        .collect();
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("bob")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(
        items.len(),
        20,
        "cap at SEARCH_MAX_RESULTS (20): got {}",
        items.len()
    );
}

#[tokio::test]
async fn test_no_match_returns_empty() {
    let ctx = TestContext::setup().await;
    let participants = vec![
        ("Alice".to_string(), "alice".to_string()),
        ("Bob".to_string(), "bob".to_string()),
    ];
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("zzz")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert!(items.is_empty(), "no match should yield empty list: {:?}", items);
}

#[tokio::test]
async fn test_dedup_between_gsi_and_scan() {
    // If a participant matches BOTH username prefix (GSI3) and display_name
    // prefix (scan), they must not appear twice.
    let ctx = TestContext::setup().await;
    let participants = vec![
        // display_name AND username both start with "ann" → hit both paths
        ("Ann Park".to_string(), "ann_park".to_string()),
        ("Ann Kim".to_string(), "ann_kim".to_string()),
    ];
    let space_id = seed_space_with_participants(&ctx, participants).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &url(&space_id, Some("ann")),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "search: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(items.len(), 2, "dedup expected, got {:?}", items);

    let pks: Vec<&str> = items.iter().map(|i| i["user_id"].as_str().unwrap()).collect();
    let unique: std::collections::HashSet<_> = pks.iter().collect();
    assert_eq!(
        unique.len(),
        pks.len(),
        "user_ids must be unique across GSI/scan merge"
    );
}
