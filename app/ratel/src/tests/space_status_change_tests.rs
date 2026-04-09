use super::*;
use crate::common::models::notification::Notification;
use crate::common::models::space::{SpaceCommon, SpaceStatusChangeEvent};
use crate::common::types::EntityType;
use crate::common::types::Partition;
use crate::common::types::SpaceStatus;
use crate::common::types::{NotificationData, SpacePublishState, SpaceVisibility};
use crate::features::auth::UserTeamGroup;
use crate::features::posts::models::{Team, TeamOwner};
use crate::features::posts::types::TeamGroupPermissions;
use crate::common::models::space::SpaceParticipant;
use crate::features::spaces::space_common::services::handle_space_status_change;

/// Smoke test: handler accepts an event for an unknown transition and returns Ok.
#[tokio::test]
async fn test_handle_unknown_transition_is_noop() {
    let ctx = TestContext::setup().await;
    let _ = ctx; // force setup so DynamoDB schema exists

    let event = SpaceStatusChangeEvent::new(
        Partition::Space("nonexistent".to_string()),
        Some(SpaceStatus::Finished),
        SpaceStatus::Open,
    );

    // Unknown/illegal transition → handler short-circuits before loading the space.
    let result = handle_space_status_change(event).await;
    assert!(result.is_ok(), "expected Ok, got {:?}", result);
}

/// Helper: insert a minimal team-owned space directly into DynamoDB.
async fn insert_team_space(
    ctx: &TestContext,
    team_pk: Partition,
    status: Option<SpaceStatus>,
) -> SpaceCommon {
    let post_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let space_pk = Partition::Space(post_id.clone());
    let post_pk = Partition::Feed(post_id);

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = status;
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = team_pk;
    space.author_display_name = "team".to_string();
    space.author_profile_url = String::new();
    space.author_username = "team".to_string();

    space.create(&ctx.ddb).await.unwrap();

    // Also create a minimal Post row so the handler can load it.
    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "Test Space".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    space
}

async fn create_team_with_members(
    ctx: &TestContext,
    member_count: usize,
) -> (Partition, Vec<crate::features::auth::User>) {
    let owner = &ctx.test_user.0;
    let team_pk = Team::create_new_team(
        owner,
        &ctx.ddb,
        format!("team{}", uuid::Uuid::new_v4()),
        String::new(),
        format!("team-{}", uuid::Uuid::new_v4().simple()),
        "desc".to_string(),
    )
    .await
    .unwrap();

    let mut members = Vec::new();
    for _ in 0..member_count {
        let member = create_test_user(&ctx.ddb).await;
        let admin_group_sk = EntityType::TeamGroup(format!("{}#Admin", team_pk));
        let utg = UserTeamGroup::new(
            member.pk.clone(),
            admin_group_sk,
            i64::from(TeamGroupPermissions::member()),
            team_pk.clone(),
        );
        utg.create(&ctx.ddb).await.unwrap();
        members.push(member);
    }

    (team_pk, members)
}

/// Shared scan helper used by every assertion in this file. DynamoDB scans are
/// slow but acceptable in tests against the `ratel-local` table.
async fn scan_items_with_sk_prefix<T: serde::de::DeserializeOwned>(
    ctx: &TestContext,
    sk_prefix: &str,
) -> Vec<T> {
    use aws_sdk_dynamodb::types::AttributeValue;

    let table_name = format!(
        "{}-main",
        option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local")
    );

    let mut out: Vec<T> = Vec::new();
    let mut esk = None;
    loop {
        let mut req = ctx
            .ddb
            .scan()
            .table_name(&table_name)
            .filter_expression("begins_with(sk, :p)")
            .expression_attribute_values(":p", AttributeValue::S(sk_prefix.to_string()));
        if let Some(k) = esk {
            req = req.set_exclusive_start_key(Some(k));
        }
        let page = req.send().await.expect("scan failed");
        for item in page.items.unwrap_or_default() {
            if let Ok(parsed) = serde_dynamo::from_item::<_, T>(item) {
                out.push(parsed);
            }
        }
        match page.last_evaluated_key {
            Some(k) => esk = Some(k),
            None => break,
        }
    }
    out
}

async fn notifications_matching(
    ctx: &TestContext,
    filter: impl Fn(&Notification) -> bool,
) -> Vec<Notification> {
    let rows: Vec<Notification> = scan_items_with_sk_prefix(ctx, "NOTIFICATION#").await;
    rows.into_iter().filter(|n| filter(n)).collect()
}

#[tokio::test]
async fn test_handle_publish_to_open_notifies_team_members() {
    let ctx = TestContext::setup().await;

    let (team_pk, members) = create_team_with_members(&ctx, 2).await;
    let space = insert_team_space(&ctx, team_pk.clone(), None).await;

    let event = SpaceStatusChangeEvent::new(space.pk.clone(), None, SpaceStatus::Open);

    handle_space_status_change(event)
        .await
        .expect("handler failed");

    // Expect at least one Notification row whose data is SendSpaceStatusUpdate and
    // whose email list covers both members + the team owner.
    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;

    let all_emails: Vec<String> = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, .. } = &n.data {
                emails.clone()
            } else {
                vec![]
            }
        })
        .collect();

    assert!(
        all_emails.contains(&members[0].email),
        "expected notification to include first team member email. all={:?}",
        all_emails
    );
    assert!(
        all_emails.contains(&members[1].email),
        "expected notification to include second team member email. all={:?}",
        all_emails
    );
    assert!(
        all_emails.contains(&ctx.test_user.0.email),
        "expected notification to include team owner email. all={:?}",
        all_emails
    );
}

// ── Task 10 ──────────────────────────────────────────────────────────────────

/// Helper: insert a minimal user-owned (non-team) space directly into DynamoDB.
async fn insert_user_space(
    ctx: &TestContext,
    user_pk: Partition,
    status: Option<SpaceStatus>,
) -> SpaceCommon {
    let post_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let space_pk = Partition::Space(post_id.clone());
    let post_pk = Partition::Feed(post_id);

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = status;
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = user_pk;
    space.author_display_name = "user".to_string();
    space.author_profile_url = String::new();
    space.author_username = "user".to_string();
    space.create(&ctx.ddb).await.unwrap();

    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "User Space".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    space
}

#[tokio::test]
async fn test_handle_publish_to_open_skips_user_authored() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), None).await;

    let before = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        None,
        SpaceStatus::Open,
    ))
    .await
    .expect("handler failed");

    let after = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    assert_eq!(
        before,
        after,
        "expected zero new notifications for user-authored publish; before={} after={}",
        before,
        after
    );
}

// ── Task 11 ──────────────────────────────────────────────────────────────────

/// Helper: insert a SpaceParticipant row for the given user in the given space.
async fn insert_participant_for(
    ctx: &TestContext,
    space_pk: &Partition,
    user: &crate::features::auth::User,
) {
    let sp = SpaceParticipant::new_non_anonymous(space_pk.clone(), user.clone());
    sp.create(&ctx.ddb).await.unwrap();
}

#[tokio::test]
async fn test_handle_open_to_ongoing_notifies_participants() {
    let ctx = TestContext::setup().await;
    let space =
        insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;

    let p1 = create_test_user(&ctx.ddb).await;
    let p2 = create_test_user(&ctx.ddb).await;
    insert_participant_for(&ctx, &space.pk, &p1).await;
    insert_participant_for(&ctx, &space.pk, &p2).await;

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;
    let emails: Vec<String> = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, .. } = &n.data {
                emails.clone()
            } else {
                vec![]
            }
        })
        .collect();

    assert!(emails.contains(&p1.email), "emails={:?}", emails);
    assert!(emails.contains(&p2.email), "emails={:?}", emails);
}

// ── Task 12 ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_handle_ongoing_to_finished_notifies_participants() {
    let ctx = TestContext::setup().await;
    let space =
        insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Ongoing)).await;

    let p1 = create_test_user(&ctx.ddb).await;
    insert_participant_for(&ctx, &space.pk, &p1).await;

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Ongoing),
        SpaceStatus::Finished,
    ))
    .await
    .expect("handler failed");

    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;
    let emails: Vec<String> = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate {
                emails, headline, ..
            } = &n.data
            {
                if headline.contains("has ended") {
                    emails.clone()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        })
        .collect();

    assert!(emails.contains(&p1.email), "emails={:?}", emails);
}

// ── Task 13 ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_handle_no_recipients_is_noop_participants() {
    let ctx = TestContext::setup().await;
    let space =
        insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;
    // No participants inserted.

    let before = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    let after = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    assert_eq!(before, after, "expected zero new notifications");
}

// ── Task 13a ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_handle_dedupes_duplicate_emails() {
    let ctx = TestContext::setup().await;
    let space =
        insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;

    // Create two users with the same email to simulate an email collision.
    let p1 = create_test_user(&ctx.ddb).await;

    // p2 is created with the same email as p1 by passing it directly to User::new.
    let username2 = create_user_name();
    let p2 = crate::common::models::auth::User::new(
        "dupUser".to_string(),
        p1.email.clone(),
        String::new(),
        true,
        true,
        crate::common::types::UserType::Individual,
        username2,
        None,
    );
    p2.create(&ctx.ddb).await.unwrap();

    insert_participant_for(&ctx, &space.pk, &p1).await;
    insert_participant_for(&ctx, &space.pk, &p2).await;

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    let rows = notifications_matching(&ctx, |n| {
        if let NotificationData::SendSpaceStatusUpdate { headline, .. } = &n.data {
            headline.contains("is starting now")
        } else {
            false
        }
    })
    .await;

    let count_matching: usize = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, .. } = &n.data {
                emails.iter().filter(|e| **e == p1.email).cloned().collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .count();

    assert_eq!(
        count_matching,
        1,
        "expected duplicate email to appear once, got {}",
        count_matching
    );
}

// ── Tasks 16 + 18: Controller integration tests ─────────────────────────────

/// Create a draft space via the HTTP API. Returns (post_pk_str, space_id_str).
/// The test user will be the Creator of this space.
async fn create_draft_space(ctx: &TestContext) -> (String, String) {
    // Step 1: Create a draft post
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/posts",
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "create_post failed: {:?}", body);
    let post_pk_str = body["post_pk"].as_str().unwrap().to_string();

    // Extract just the ID from "FEED#<uuid>"
    let feed_id = post_pk_str
        .strip_prefix("FEED#")
        .unwrap_or(&post_pk_str)
        .to_string();

    // Step 2: Create a space on that post
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/create",
        headers: ctx.test_user.1.clone(),
        body: { "req": { "post_id": feed_id } }
    };
    assert_eq!(status, 200, "create_space failed: {:?}", body);
    let space_id = body["space_id"].as_str().unwrap().to_string();

    (post_pk_str, space_id)
}

/// Scan DynamoDB for SpaceStatusChangeEvent rows whose space_pk matches.
async fn find_status_change_events_for(
    ctx: &TestContext,
    space_pk: &Partition,
) -> Vec<SpaceStatusChangeEvent> {
    let rows: Vec<SpaceStatusChangeEvent> =
        scan_items_with_sk_prefix(ctx, "SPACE_STATUS_CHANGE_EVENT#").await;
    rows.into_iter().filter(|r| &r.space_pk == space_pk).collect()
}

#[tokio::test]
async fn test_publish_creates_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk_str, space_id) = create_draft_space(&ctx).await;
    let space_pk = Partition::Space(space_id.clone());

    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "publish": true, "visibility": "Public" } }
    };
    assert_eq!(status, 200, "publish failed: {:?}", body);

    let events = find_status_change_events_for(&ctx, &space_pk).await;
    assert_eq!(events.len(), 1, "expected 1 event, got {:?}", events);
    assert_eq!(events[0].old_status, None);
    assert_eq!(events[0].new_status, SpaceStatus::Open);
}

#[tokio::test]
async fn test_start_creates_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk_str, space_id) = create_draft_space(&ctx).await;
    let space_pk = Partition::Space(space_id.clone());

    // Publish first (Designing → Open)
    let (s, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "publish": true, "visibility": "Public" } }
    };
    assert_eq!(s, 200, "publish failed");

    // Start (Open → Ongoing)
    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "start": true } }
    };
    assert_eq!(status, 200, "start failed: {:?}", body);

    let events = find_status_change_events_for(&ctx, &space_pk).await;
    assert!(events.len() >= 2, "expected >= 2 events, got {:?}", events);
    assert!(events.iter().any(|e| e.old_status == Some(SpaceStatus::Open)
        && e.new_status == SpaceStatus::Ongoing));
}

#[tokio::test]
async fn test_finish_creates_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk_str, space_id) = create_draft_space(&ctx).await;
    let space_pk = Partition::Space(space_id.clone());

    // Publish
    let (s, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "publish": true, "visibility": "Public" } }
    };
    assert_eq!(s, 200, "publish failed");

    // Start
    let (s, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "start": true } }
    };
    assert_eq!(s, 200, "start failed");

    // Finish (Ongoing → Finished)
    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "finished": true } }
    };
    assert_eq!(status, 200, "finish failed: {:?}", body);

    let events = find_status_change_events_for(&ctx, &space_pk).await;
    assert!(
        events.iter().any(|e| e.old_status == Some(SpaceStatus::Ongoing)
            && e.new_status == SpaceStatus::Finished),
        "expected Ongoing→Finished event, got {:?}",
        events
    );
}

#[tokio::test]
async fn test_title_update_creates_no_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk_str, space_id) = create_draft_space(&ctx).await;
    let space_pk = Partition::Space(space_id.clone());

    let before = find_status_change_events_for(&ctx, &space_pk).await.len();

    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "title": "Renamed" } }
    };
    assert_eq!(status, 200, "title update failed: {:?}", body);

    let after = find_status_change_events_for(&ctx, &space_pk).await.len();
    assert_eq!(before, after, "expected no event on title update");
}
