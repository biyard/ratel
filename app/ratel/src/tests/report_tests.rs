//! Server-function tests for the report list/create/get endpoints.
//!
//! These exercise the DynamoDB-backed flow end-to-end through the
//! Axum router: seed a space, POST a report, list it back, fetch the
//! detail, then assert the not-found branch. The space row is written
//! straight to DDB (no API hop) since the report controllers are
//! authenticated-but-not-role-gated for now — the goal is to verify
//! the report layer in isolation.

use super::*;
use crate::common::models::space::SpaceCommon;
use crate::common::types::{
    EntityType, ListResponse, Partition, SpacePublishState, SpaceStatus, SpaceVisibility,
};
use crate::features::spaces::pages::report::controllers::{
    CreateReportResponse, DeleteReportResponse, GetReportResponse,
};
use crate::features::spaces::pages::report::types::{ReportListItem, ReportStatus};

/// Seed a published public space owned by the test user, return its
/// bare ulid (no `SPACE#` prefix). Mirrors the lightweight setup used
/// by `space_member_tests.rs` — no Post row needed since the report
/// endpoints only touch `SpaceCommon`-keyed rows on the same partition.
async fn seed_space(ctx: &TestContext) -> String {
    let space_id = uuid::Uuid::now_v7().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(space_id.clone());

    let mut space = SpaceCommon::default();
    space.pk = space_pk;
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Ongoing);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk;
    space.user_pk = ctx.test_user.0.pk.clone();
    space.author_display_name = ctx.test_user.0.display_name.clone();
    space.author_profile_url = ctx.test_user.0.profile_url.clone();
    space.author_username = ctx.test_user.0.username.clone();
    space.create(&ctx.ddb).await.expect("seed space");

    space_id
}

#[tokio::test]
async fn test_list_reports_empty_initially() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };

    assert_eq!(status, 200, "list_reports empty: {:?}", body);
    assert!(
        body.items.is_empty(),
        "fresh space should have no reports, got {} items",
        body.items.len()
    );
}

#[tokio::test]
async fn test_create_then_list_returns_the_report() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    // Create with full payload so we can assert every projected field
    // round-trips through the model.
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        body: {
            "req": {
                "title": "탄소 상쇄 결과",
                "description": "first draft"
            }
        },
        response_type: CreateReportResponse,
    };
    assert_eq!(status, 200, "create_report: {:?}", body);
    let report_id = body.id;
    assert!(!report_id.is_empty(), "create_report must return an id");

    // List should now contain exactly the created row.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(status, 200, "list_reports after create: {:?}", body);
    assert_eq!(
        body.items.len(),
        1,
        "expected one report, got {:?}",
        body.items
    );

    let item = &body.items[0];
    assert_eq!(item.id, report_id);
    assert_eq!(item.title, "탄소 상쇄 결과");
    assert_eq!(item.description, "first draft");
    assert_eq!(item.status, ReportStatus::Draft);
}

#[tokio::test]
async fn test_get_report_returns_full_detail() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "title": "퀴즈 통과율" } },
        response_type: CreateReportResponse,
    };
    assert_eq!(status, 200, "create_report: {:?}", body);
    let report_id = body.id;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports/{}", space_id, report_id),
        headers: ctx.test_user.1.clone(),
        response_type: GetReportResponse,
    };
    assert_eq!(status, 200, "get_report: {:?}", body);
    assert_eq!(body.id, report_id);
    assert_eq!(body.title, "퀴즈 통과율");
    assert!(body.html_contents.is_none());
}

#[tokio::test]
async fn test_get_missing_report_errors() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    let (status, _, _body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports/{}", space_id, uuid::Uuid::now_v7()),
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "missing report must not 200");
}

#[tokio::test]
async fn test_list_reports_orders_newest_first() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    // Insert three reports in order; uuid_v7 sort keys + the
    // controller's `scan_index_forward(false)` should surface them
    // newest-first.
    let mut ids = Vec::new();
    for i in 0..3 {
        let (status, _, body) = crate::test_post! {
            app: ctx.app.clone(),
            path: &format!("/v3/spaces/{}/reports", space_id),
            headers: ctx.test_user.1.clone(),
            body: { "req": { "title": format!("Report {i}") } },
            response_type: CreateReportResponse,
        };
        assert_eq!(status, 200, "seed report {i}: {:?}", body);
        ids.push(body.id);
        // uuid_v7 has millisecond resolution — a brief sleep
        // guarantees distinct timestamps so the ordering test isn't
        // brittle under fast hardware.
        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
    }

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(status, 200, "list reports: {:?}", body);
    assert_eq!(body.items.len(), 3);
    let returned: Vec<&str> = body.items.iter().map(|r| r.id.as_str()).collect();
    let expected: Vec<&str> = ids.iter().rev().map(String::as_str).collect();
    assert_eq!(returned, expected, "expected newest-first ordering");
}

#[tokio::test]
async fn test_create_report_forbidden_for_non_creator() {
    // Owner seeds + the second user authenticates as a non-creator.
    // The space is public so the extractor will resolve the second
    // user's role to `Viewer`; `SpaceReport::can_edit` then rejects
    // with `Error::NoPermission`.
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;
    let (_other_user, other_headers) = ctx.create_another_user().await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: other_headers.clone(),
        body: { "req": { "title": "should fail" } }
    };
    assert_ne!(
        status, 200,
        "non-creator must not be allowed to create reports, got {:?}",
        body
    );

    // Sanity: the owner still can — proves the test exercises the
    // role gate, not a broken-everywhere endpoint.
    let (owner_status, _, owner_body) = crate::test_post! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "title": "owner ok" } },
        response_type: CreateReportResponse,
    };
    assert_eq!(
        owner_status, 200,
        "owner should still succeed: {:?}",
        owner_body
    );
}

#[tokio::test]
async fn test_list_reports_allowed_for_non_creator_on_public_space() {
    // The list endpoint uses `can_view`, which is permissive for any
    // resolved space role. A non-creator on a public space should be
    // able to read but not write.
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;
    let (_other_user, other_headers) = ctx.create_another_user().await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: other_headers,
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(
        status, 200,
        "non-creator should still read the list on a public space: {:?}",
        body
    );
}

/// Helper for the status-filter test: write a `SpaceReport` row
/// directly to DDB with the given status so we can exercise the GSI
/// query path without going through the create endpoint (which only
/// creates Drafts).
async fn seed_report_with_status(
    ctx: &TestContext,
    space_id: &str,
    title: &str,
    status: crate::features::spaces::pages::report::types::ReportStatus,
) -> String {
    use crate::common::utils::time::get_now_timestamp_millis;
    use crate::features::spaces::pages::report::models::SpaceReport;

    let now = get_now_timestamp_millis();
    let report = SpaceReport {
        pk: Partition::Space(space_id.to_string()),
        sk: EntityType::SpaceReport(uuid::Uuid::now_v7().to_string()),
        created_at: now,
        updated_at: now,
        status,
        title: title.to_string(),
        description: String::new(),
        html_contents: None,
    };
    let id = match &report.sk {
        EntityType::SpaceReport(id) => id.clone(),
        _ => String::new(),
    };
    report.create(&ctx.ddb).await.expect("seed report");
    // Ensure later uuid_v7 ids sort after earlier ones.
    tokio::time::sleep(std::time::Duration::from_millis(2)).await;
    id
}

#[tokio::test]
async fn test_list_reports_filters_by_status_via_gsi() {
    use crate::features::spaces::pages::report::types::ReportStatus;
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    // Mix of drafts and published, interleaved so a naive non-filter
    // implementation can't pass by accident.
    let draft1 = seed_report_with_status(&ctx, &space_id, "draft 1", ReportStatus::Draft).await;
    let pub1 = seed_report_with_status(&ctx, &space_id, "pub 1", ReportStatus::Published).await;
    let draft2 = seed_report_with_status(&ctx, &space_id, "draft 2", ReportStatus::Draft).await;
    let pub2 = seed_report_with_status(&ctx, &space_id, "pub 2", ReportStatus::Published).await;

    // ── No filter: all four rows surface ─────────────────────
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(status, 200, "list (no filter): {:?}", body);
    assert_eq!(body.items.len(), 4, "all reports should surface");

    // ── ?status=Draft: only the two drafts ───────────────────
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports?status=Draft", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(status, 200, "list (status=Draft): {:?}", body);
    let ids: Vec<&str> = body.items.iter().map(|r| r.id.as_str()).collect();
    // newest-first ordering inside the status half: draft2 then draft1
    assert_eq!(ids, vec![draft2.as_str(), draft1.as_str()]);
    assert!(
        body.items
            .iter()
            .all(|r| r.status == ReportStatus::Draft),
        "GSI query must not leak Published rows: {:?}",
        body.items
    );

    // ── ?status=Published: only the two published rows ──────
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports?status=Published", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(status, 200, "list (status=Published): {:?}", body);
    let ids: Vec<&str> = body.items.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, vec![pub2.as_str(), pub1.as_str()]);
    assert!(
        body.items
            .iter()
            .all(|r| r.status == ReportStatus::Published),
        "GSI query must not leak Draft rows: {:?}",
        body.items
    );
}

#[tokio::test]
async fn test_delete_report_removes_row_and_refreshes_list() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;

    // Create a report so we have a row to delete.
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "title": "delete me" } },
        response_type: CreateReportResponse,
    };
    assert_eq!(status, 200, "create_report: {:?}", body);
    let report_id = body.id;

    // Delete it as the creator.
    let (status, _, body) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports/{}", space_id, report_id),
        headers: ctx.test_user.1.clone(),
        response_type: DeleteReportResponse,
    };
    assert_eq!(status, 200, "delete_report: {:?}", body);
    assert_eq!(body.id, report_id);

    // The list should now be empty again.
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<ReportListItem>,
    };
    assert_eq!(status, 200, "list after delete: {:?}", body);
    assert!(
        body.items.is_empty(),
        "deleted row should no longer surface, got {:?}",
        body.items
    );
}

#[tokio::test]
async fn test_delete_report_forbidden_for_non_creator() {
    let ctx = TestContext::setup().await;
    let space_id = seed_space(&ctx).await;
    let (_other_user, other_headers) = ctx.create_another_user().await;

    // Owner creates a row.
    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports", space_id),
        headers: ctx.test_user.1.clone(),
        body: { "req": { "title": "protected" } },
        response_type: CreateReportResponse,
    };
    let report_id = body.id;

    // Non-creator on the same (public) space can read but must not
    // delete — `SpaceReport::can_delete` rejects with NoPermission.
    let (status, _, _) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/v3/spaces/{}/reports/{}", space_id, report_id),
        headers: other_headers,
    };
    assert_ne!(
        status, 200,
        "non-creator must not be allowed to delete reports"
    );

    // Sanity: the row is still there because the delete was rejected.
    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/v3/spaces/{}/reports/{}", space_id, report_id),
        headers: ctx.test_user.1.clone(),
        response_type: GetReportResponse,
    };
    assert_eq!(
        status, 200,
        "owner should still see the row: {:?}",
        body
    );
    assert_eq!(body.id, report_id);
}
