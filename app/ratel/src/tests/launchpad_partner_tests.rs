use super::*;
use crate::common::types::{EntityType, Partition};
use crate::features::launchpad_partner::models::LaunchpadDeduction;

#[tokio::test]
async fn deduction_row_round_trips() {
    let ctx = TestContext::setup().await;
    let user = format!("u-{}", uuid::Uuid::new_v4());
    let key = format!("lp_{}", uuid::Uuid::new_v4());

    let row = LaunchpadDeduction::new(&user, &key, 500, "tx_demo", 740);
    row.create(&ctx.ddb).await.expect("create");

    let fetched = LaunchpadDeduction::get(
        &ctx.ddb,
        Partition::User(user.clone()),
        Some(EntityType::LaunchpadDeduction(key.clone())),
    )
    .await
    .expect("get")
    .expect("row present");

    assert_eq!(fetched.point_amount, 500);
    assert_eq!(fetched.brand_tx_id, "tx_demo");
    assert_eq!(fetched.remaining_points, 740);
}

use crate::axum::body::Body;
use crate::axum::http::Request;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tower::ServiceExt;

const TEST_PROJECT_ID: &str = "launchpad-demo";
const TEST_SECRET: &str = "dev-demo-shared-secret-change-me";

fn sign(ts: &str, body: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
    mac.update(ts.as_bytes());
    mac.update(b".");
    mac.update(body.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

async fn post_callback(
    ctx: &TestContext,
    path: &str,
    body: &str,
    signed: bool,
    project_id: &str,
) -> u16 {
    let ts = "1717459200000";
    let sig = if signed {
        sign(ts, body)
    } else {
        "deadbeef".to_string()
    };
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .header("x-launchpad-timestamp", ts)
        .header("x-launchpad-project-id", project_id)
        .header("x-launchpad-signature", sig)
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = ctx.app.clone().oneshot(req).await.unwrap();
    resp.status().as_u16()
}

#[tokio::test]
async fn health_ok_with_valid_signature() {
    let ctx = TestContext::setup().await;
    let body = format!(
        r#"{{"project_id":"{TEST_PROJECT_ID}","check":"launchpad_company_point_health"}}"#
    );
    let status = post_callback(&ctx, "/launchpad/health", &body, true, TEST_PROJECT_ID).await;
    assert_eq!(status, 200, "valid health should be 200");
}

#[tokio::test]
async fn callback_rejects_bad_signature() {
    let ctx = TestContext::setup().await;
    let body = format!(
        r#"{{"project_id":"{TEST_PROJECT_ID}","check":"launchpad_company_point_health"}}"#
    );
    let status = post_callback(&ctx, "/launchpad/health", &body, false, TEST_PROJECT_ID).await;
    assert_eq!(status, 401, "bad signature should be 401");
}

#[tokio::test]
async fn callback_rejects_project_mismatch() {
    let ctx = TestContext::setup().await;
    let body = r#"{"project_id":"wrong-project","check":"launchpad_company_point_health"}"#;
    let status = post_callback(&ctx, "/launchpad/health", body, true, "wrong-project").await;
    assert_eq!(status, 403, "project mismatch should be 403");
}

#[tokio::test]
async fn deduct_is_idempotent_without_console() {
    // Pre-seed the idempotency row; the second deduct must return the stored
    // result WITHOUT calling the console (which is unreachable in tests).
    let ctx = TestContext::setup().await;
    let user = format!("u-{}", uuid::Uuid::new_v4());
    let key = format!("lp_{}", uuid::Uuid::new_v4());
    LaunchpadDeduction::new(&user, &key, 500, "tx_seed", 740)
        .create(&ctx.ddb)
        .await
        .expect("seed");

    let body = format!(
        r#"{{"project_id":"{TEST_PROJECT_ID}","company_user_key":"{user}","point_amount":500,"idempotency_key":"{key}"}}"#
    );
    let status = post_callback(&ctx, "/launchpad/points/deduct", &body, true, TEST_PROJECT_ID).await;
    assert_eq!(status, 200, "idempotent replay should be 200 without console");
}
