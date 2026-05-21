//! Integration tests for POST /api/posts/:post_id/ai-draft.
//!
//! Bedrock is never hit — `TestContext::setup()` forces
//! `RATEL_AI_WRITER_TYPE=fixture`, which selects the `FixtureWriter`
//! compiled in under `--features bypass`.

use super::*;
use crate::common::DynamoEntity;
use crate::common::types::{ContentBody, EntityType, Partition};
use crate::features::membership::models::{
    Membership, MembershipStatus, MembershipTier, UserMembership,
};
use crate::features::posts::models::Post;
use crate::features::posts::types::{Author, PostType};

const PRO_CREDITS: i64 = 10_000;
const PRO_DURATION_DAYS: i32 = 30;
const TOPIC: &str = "공공장소 흡연 구역 재배치";
const BACKGROUND: &str = "주거지·학교 근처 흡연 구역 민원이 늘었습니다.";
const FEEDBACK: &str = "위치 적정성, 대안 후보지, 운영 가치에 대한 의견";

fn pro_membership_pk() -> Partition {
    Partition::Membership(MembershipTier::Pro.to_string())
}

async fn ensure_pro_membership(cli: &aws_sdk_dynamodb::Client) {
    let now = chrono::Utc::now().timestamp_millis();
    let m = Membership {
        pk: pro_membership_pk(),
        sk: EntityType::Membership,
        created_at: now,
        updated_at: now,
        credits: PRO_CREDITS,
        tier: MembershipTier::Pro,
        price_dollars: 10,
        price_won: 13000,
        display_order: 2,
        duration_days: PRO_DURATION_DAYS,
        max_credits_per_space: 100,
        is_active: true,
        display_order_indexed: 2,
    };
    // Tests share the table across runs — Pro row may already exist. Ignore
    // a ConditionalCheckFailedException from `.create()`'s attribute_not_exists
    // guard; surface anything else.
    if let Err(e) = m.create(cli).await {
        let msg = format!("{e}");
        assert!(
            msg.contains("ConditionalCheckFailed"),
            "pro membership create: {e}"
        );
    }
}

async fn ensure_free_membership(cli: &aws_sdk_dynamodb::Client) {
    let now = chrono::Utc::now().timestamp_millis();
    let m = Membership {
        pk: Partition::Membership(MembershipTier::Free.to_string()),
        sk: EntityType::Membership,
        created_at: now,
        updated_at: now,
        credits: 100,
        tier: MembershipTier::Free,
        price_dollars: 0,
        price_won: 0,
        display_order: 1,
        duration_days: 30,
        max_credits_per_space: 10,
        is_active: true,
        display_order_indexed: 1,
    };
    if let Err(e) = m.create(cli).await {
        let msg = format!("{e}");
        assert!(
            msg.contains("ConditionalCheckFailed"),
            "free membership create: {e}"
        );
    }
}

async fn make_user_paid(cli: &aws_sdk_dynamodb::Client, user_pk: &Partition) {
    ensure_pro_membership(cli).await;
    let now = chrono::Utc::now().timestamp_millis();
    let expired_at = now + (PRO_DURATION_DAYS as i64) * 24 * 3600 * 1000;
    let um = UserMembership {
        pk: user_pk.clone(),
        sk: EntityType::UserMembership,
        created_at: now,
        updated_at: now,
        expired_at,
        membership_pk: pro_membership_pk().into(),
        status: MembershipStatus::Active,
        total_credits: PRO_CREDITS,
        remaining_credits: PRO_CREDITS,
        auto_renew: false,
        next_membership: None,
        monthly_refill_credits: 0,
        next_refill_at: 0,
    };
    if let Err(e) = um.create(cli).await {
        let msg = format!("{e}");
        assert!(
            msg.contains("ConditionalCheckFailed"),
            "user membership create: {e}"
        );
    }
}

async fn create_draft_post(ctx: &TestContext) -> Post {
    let author = Author {
        pk: ctx.test_user.0.pk.clone(),
        display_name: ctx.test_user.0.display_name.clone(),
        profile_url: ctx.test_user.0.profile_url.clone(),
        username: ctx.test_user.0.username.clone(),
        user_type: ctx.test_user.0.user_type.clone(),
    };
    let post = Post::new("", "", PostType::Post, author);
    post.create(&ctx.ddb).await.expect("post create");
    post
}

fn post_id_from(post: &Post) -> String {
    match &post.pk {
        Partition::Feed(id) => id.clone(),
        other => panic!("unexpected post pk variant: {other:?}"),
    }
}

#[tokio::test]
async fn paid_user_generates_draft_success() {
    let ctx = TestContext::setup().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_eq!(status, 200, "expected success: {body:?}");
    let title = body["title"].as_str().expect("title");
    let body_html = body["body_html"].as_str().expect("body_html");
    assert!(!title.is_empty(), "title must be non-empty");
    for heading in [
        "추진배경",
        "추진목적",
        "추진내용",
        "의견수렴 사항",
        "참여 안내",
    ] {
        assert!(
            body_html.contains(heading),
            "body must contain heading {heading:?}: {body_html}"
        );
    }

    let loaded = Post::get(&ctx.ddb, &post.pk, Some(EntityType::Post))
        .await
        .unwrap()
        .expect("post row");
    assert!(loaded.ai_draft_used, "ai_draft_used must be set");
    assert_eq!(loaded.title, title);
    assert_eq!(loaded.body, ContentBody::html(body_html.to_string()));
}

#[tokio::test]
async fn free_user_is_rejected() {
    let ctx = TestContext::setup().await;
    ensure_free_membership(&ctx.ddb).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_eq!(status, 403, "free user must be rejected with 403: body={body:?}");

    let loaded = Post::get(&ctx.ddb, &post.pk, Some(EntityType::Post))
        .await
        .unwrap()
        .expect("post row");
    assert!(!loaded.ai_draft_used, "free-user rejection must not flip flag");
}

#[tokio::test]
async fn already_used_post_is_rejected() {
    let ctx = TestContext::setup().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_eq!(status, 200);

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_eq!(status, 409, "second call must be rejected with 409 Conflict");
}

#[tokio::test]
async fn unauthenticated_is_rejected() {
    let ctx = TestContext::setup().await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_ne!(status, 200, "unauthenticated must not succeed");
}

#[tokio::test]
async fn empty_topic_is_invalid_input() {
    let ctx = TestContext::setup().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": "",
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_eq!(status, 400, "empty topic must yield 400");
}

#[tokio::test]
async fn empty_background_is_invalid_input() {
    let ctx = TestContext::setup().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": "",
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_eq!(status, 400, "empty background must yield 400");
}

#[tokio::test]
async fn empty_feedback_request_is_invalid_input() {
    let ctx = TestContext::setup().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": "",
            "language": "ko"
        } }
    };
    assert_eq!(status, 400, "empty feedback_request must yield 400");
}

#[tokio::test]
async fn cannot_generate_for_another_users_post() {
    let ctx = TestContext::setup().await;
    let (_other_user, other_headers) = ctx.create_another_user().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: other_headers,
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "ko"
        } }
    };
    assert_ne!(
        status, 200,
        "another user must not be able to AI-draft this post"
    );
}

#[tokio::test]
async fn english_language_returns_english_headings() {
    let ctx = TestContext::setup().await;
    make_user_paid(&ctx.ddb, &ctx.test_user.0.pk).await;
    let post = create_draft_post(&ctx).await;
    let path = format!("/api/posts/{}/ai-draft", post_id_from(&post));

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &path,
        headers: ctx.test_user.1.clone(),
        body: { "req": {
            "template": "opinion_gathering",
            "topic": TOPIC,
            "background": BACKGROUND,
            "feedback_request": FEEDBACK,
            "language": "en"
        } }
    };
    assert_eq!(status, 200, "english draft must succeed: {body:?}");
    let body_html = body["body_html"].as_str().expect("body_html");
    for heading in [
        "Background",
        "Purpose",
        "Content",
        "Topics for Input",
        "How to Participate",
    ] {
        assert!(
            body_html.contains(heading),
            "english body must contain heading {heading:?}: {body_html}"
        );
    }
}
