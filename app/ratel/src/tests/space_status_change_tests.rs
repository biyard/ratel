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
