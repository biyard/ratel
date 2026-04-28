use super::*;
use crate::common::models::notification::{Notification, UserInboxNotification};
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{
    EntityType, InboxKind, InboxPayload, NotificationData, Partition, SpacePartition,
    SpacePublishState, SpaceStatus, SpaceVisibility,
};
use crate::features::posts::models::Post;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::actions::services::notify_action_ongoing;
use crate::features::spaces::pages::actions::types::{SpaceActionStatus, SpaceActionType};

/// Insert a minimal space with a given status. Returns the SpaceCommon plus
/// the space partition for convenience.
async fn insert_space(
    ctx: &TestContext,
    user_pk: Partition,
    status: Option<SpaceStatus>,
) -> (SpaceCommon, SpacePartition) {
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

    let post = Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "Test Space Title".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    let space_id: SpacePartition = space_pk.into();
    (space, space_id)
}

/// Build an Ongoing-status SpaceAction. The action is whatever the caller
/// passes to the notify handler; the row itself doesn't need to be in the DB
/// for the test (the handler operates on the value it receives).
fn make_ongoing_action(
    space_id: SpacePartition,
    title: &str,
    action_type: SpaceActionType,
) -> SpaceAction {
    let mut action = SpaceAction::new(space_id, uuid::Uuid::new_v4().to_string(), action_type);
    action.title = title.to_string();
    action.status = Some(SpaceActionStatus::Ongoing);
    action
}

async fn join_space(ctx: &TestContext, space_pk: &Partition, user_pk: Partition) {
    let participant = SpaceParticipant::new(space_pk.clone(), user_pk);
    participant.create(&ctx.ddb).await.unwrap();
}

async fn inbox_rows_for(
    ctx: &TestContext,
    user_pk: Partition,
) -> Vec<UserInboxNotification> {
    let (rows, _) = UserInboxNotification::query(
        &ctx.ddb,
        user_pk,
        UserInboxNotification::opt().sk("USER_INBOX_NOTIFICATION".to_string()),
    )
    .await
    .unwrap();
    rows
}

async fn notifications_matching(
    ctx: &TestContext,
    sk_prefix: &str,
    filter: impl Fn(&Notification) -> bool,
) -> Vec<Notification> {
    use aws_sdk_dynamodb::types::AttributeValue;
    let table_name = format!(
        "{}-main",
        option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local")
    );
    let mut out: Vec<Notification> = Vec::new();
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
            if let Ok(parsed) = serde_dynamo::from_item::<_, Notification>(item) {
                if filter(&parsed) {
                    out.push(parsed);
                }
            }
        }
        match page.last_evaluated_key {
            Some(k) => esk = Some(k),
            None => break,
        }
    }
    out
}

/// Happy path: ongoing space + one participant → inbox row + email Notification row.
#[tokio::test]
async fn test_notify_action_ongoing_creates_inbox_and_email() {
    let ctx = TestContext::setup().await;
    let owner_pk = ctx.test_user.0.pk.clone();

    let (space, space_id) = insert_space(&ctx, owner_pk, Some(SpaceStatus::Ongoing)).await;

    let participant = create_test_user(&ctx.ddb).await;
    join_space(&ctx, &space.pk, participant.pk.clone()).await;

    let action = make_ongoing_action(space_id, "Vote on the proposal", SpaceActionType::Poll);

    notify_action_ongoing(action.clone()).await.expect("handler failed");

    let rows = inbox_rows_for(&ctx, participant.pk.clone()).await;
    let action_rows: Vec<_> = rows
        .iter()
        .filter(|r| r.kind == InboxKind::SpaceActionOngoing)
        .collect();
    assert_eq!(
        action_rows.len(),
        1,
        "expected exactly one SpaceActionOngoing inbox row, got {} (all rows: {:?})",
        action_rows.len(),
        rows.iter().map(|r| r.kind).collect::<Vec<_>>()
    );

    match &action_rows[0].payload {
        InboxPayload::SpaceActionOngoing {
            action_id,
            action_title,
            ..
        } => {
            assert_eq!(action_id, &action.pk.1);
            assert_eq!(action_title, "Vote on the proposal");
        }
        other => panic!("expected SpaceActionOngoing payload, got {:?}", other),
    }

    let emails = notifications_matching(&ctx, "NOTIFICATION#", |n| {
        if let NotificationData::SpaceActionOngoing {
            emails,
            action_title,
            ..
        } = &n.data
        {
            action_title == "Vote on the proposal" && emails.contains(&participant.email)
        } else {
            false
        }
    })
    .await;
    assert!(
        !emails.is_empty(),
        "expected at least one SpaceActionOngoing email Notification row covering the participant"
    );
}

/// Dedup: the same action firing twice produces only one inbox row per participant.
#[tokio::test]
async fn test_notify_action_ongoing_is_idempotent_per_action() {
    let ctx = TestContext::setup().await;
    let owner_pk = ctx.test_user.0.pk.clone();

    let (space, space_id) = insert_space(&ctx, owner_pk, Some(SpaceStatus::Ongoing)).await;

    let participant = create_test_user(&ctx.ddb).await;
    join_space(&ctx, &space.pk, participant.pk.clone()).await;

    let action = make_ongoing_action(space_id, "Quiz me", SpaceActionType::Quiz);

    notify_action_ongoing(action.clone()).await.expect("first call failed");
    notify_action_ongoing(action.clone()).await.expect("second call failed");

    let rows = inbox_rows_for(&ctx, participant.pk.clone()).await;
    let action_rows: Vec<_> = rows
        .iter()
        .filter(|r| r.kind == InboxKind::SpaceActionOngoing)
        .collect();
    assert_eq!(
        action_rows.len(),
        1,
        "second invocation should be deduped; got {} rows",
        action_rows.len()
    );
}

/// Guard: parent space not Ongoing → no inbox row created.
#[tokio::test]
async fn test_notify_action_ongoing_skipped_when_space_not_ongoing() {
    let ctx = TestContext::setup().await;
    let owner_pk = ctx.test_user.0.pk.clone();

    let (space, space_id) = insert_space(&ctx, owner_pk, Some(SpaceStatus::Open)).await;

    let participant = create_test_user(&ctx.ddb).await;
    join_space(&ctx, &space.pk, participant.pk.clone()).await;

    let action = make_ongoing_action(space_id, "Should not fire", SpaceActionType::Follow);

    notify_action_ongoing(action).await.expect("handler failed");

    let rows = inbox_rows_for(&ctx, participant.pk.clone()).await;
    let any_action_rows = rows
        .iter()
        .any(|r| r.kind == InboxKind::SpaceActionOngoing);
    assert!(
        !any_action_rows,
        "expected zero SpaceActionOngoing inbox rows when parent space is not Ongoing"
    );
}

/// Guard: zero participants → handler returns Ok and writes nothing.
#[tokio::test]
async fn test_notify_action_ongoing_no_participants_is_noop() {
    let ctx = TestContext::setup().await;
    let owner_pk = ctx.test_user.0.pk.clone();

    let (_space, space_id) = insert_space(&ctx, owner_pk, Some(SpaceStatus::Ongoing)).await;

    let action = make_ongoing_action(space_id, "Nobody hears", SpaceActionType::Meet);

    let result = notify_action_ongoing(action).await;
    assert!(result.is_ok(), "handler should return Ok with no participants");
}
