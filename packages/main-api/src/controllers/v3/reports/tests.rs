use crate::controllers::v3::posts::PostDetailResponse;
use crate::controllers::v3::reports::ReportContentResponse;
use crate::controllers::v3::spaces::boards::CreateSpacePostResponse;
use crate::controllers::v3::spaces::{CreateSpaceResponse, GetSpaceResponse};
use crate::features::spaces::boards::dto::space_post_comment_response::SpacePostCommentResponse;
use crate::features::spaces::boards::dto::space_post_response::SpacePostResponse;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::PostComment;
use crate::posts::CreatePostResponse;
use crate::{tests::v3_setup::TestContextV3, *};

#[tokio::test]
async fn test_report_post() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status_create, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status_create, 200);
    assert!(!create_body.post_pk.to_string().is_empty());

    let (status_report, _headers, _report_body) = post! {
        app: app,
        path: "/v3/reports",
        headers: test_user.1.clone(),
        body: { "post_pk": create_body.post_pk },
        response_type: ReportContentResponse
    };

    assert_eq!(status_report, 200);

    let (status_get, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status_get, 200);
    assert_eq!(body.is_report, true);
    assert_eq!(body.post.unwrap_or_default().reports, 1);
}

#[tokio::test]
async fn test_report_post_comment() {
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = TestContextV3::setup().await;

    let (status_create, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status_create, 200);
    assert!(!create_body.post_pk.to_string().is_empty());

    let post_pk = create_body.post_pk.to_string();

    let comment_content = format!("<p>Test comment {}</p>", now);
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments", post_pk.clone()),
        headers: test_user.1.clone(),
        body: {
            "content": &comment_content
        },
        response_type: PostComment
    };
    assert_eq!(status, 200);

    tracing::debug!("comment body: {:?}", body.clone());

    let (status_report, _headers, _report_body) = post! {
        app: app,
        path: "/v3/reports",
        headers: test_user.1.clone(),
        body: { "post_pk": post_pk.clone(), "comment_sk": body.sk },
        response_type: ReportContentResponse
    };

    assert_eq!(status_report, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    assert!(body.comments.len() >= 1);

    assert_eq!(body.comments[0].is_report, true);
    assert_eq!(body.comments[0].reports, 1);
}

#[tokio::test]
async fn test_report_space() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status_create_post, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status_create_post, 200);
    assert!(!create_body.post_pk.to_string().is_empty());

    let (status_create_space, _headers, create_space) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": 2,
            "post_pk": create_body.post_pk,
        },
        response_type: CreateSpaceResponse
    };

    assert_eq!(status_create_space, 200);

    let (status_report, _headers, _report_body) = post! {
        app: app,
        path: "/v3/reports",
        headers: test_user.1.clone(),
        body: { "space_pk": create_space.space_pk },
        response_type: ReportContentResponse
    };

    assert_eq!(status_report, 200);

    let (status_get, _headers, space_res) = get! {
        app: app,
        path: format!("/v3/spaces/{}", create_space.space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: GetSpaceResponse,
    };

    assert_eq!(status_get, 200);
    assert_eq!(space_res.is_report, true);
    assert_eq!(space_res.reports, 1);
}

#[tokio::test]
async fn test_report_space_post() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status_report, _headers, _report_body) = post! {
        app: app,
        path: "/v3/reports",
        headers: test_user.1.clone(),
        body: { "space_pk": space_pk, "space_post_pk": space_post_pk },
        response_type: ReportContentResponse
    };

    assert_eq!(status_report, 200);

    let (status_get, _headers, body) = get! {
        app: app,
        path: format!(
            "/v3/spaces/{}/boards/{}",
            space_pk.to_string(),
            space_post_pk.to_string()
        ),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };

    assert_eq!(status_get, 200);
    assert_eq!(body.is_report, true);
    assert_eq!(body.reports, 1);
}

#[tokio::test]
async fn test_report_space_post_comment() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status_comment, _headers, comment) = post! {
        app: app,
        path: format!(
            "/v3/spaces/{}/boards/{}/comments",
            space_pk.to_string(),
            space_post_pk.to_string()
        ),
        headers: test_user.1.clone(),
        body: {
            "content": "comment".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status_comment, 200);

    let (status_report, _headers, _report_body) = post! {
        app: app,
        path: "/v3/reports",
        headers: test_user.1.clone(),
        body: { "space_post_pk": comment.pk, "comment_sk": comment.sk },
        response_type: ReportContentResponse
    };

    assert_eq!(status_report, 200);

    let (status_get, _headers, body) = get! {
        app: app,
        path: format!(
            "/v3/spaces/{}/boards/{}/comments",
            space_pk.to_string(),
            space_post_pk.to_string()
        ),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<SpacePostCommentResponse>
    };
    assert_eq!(status_get, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].is_report, true);
    assert_eq!(body.items[0].reports, 1);
}

#[tokio::test]
async fn test_add_space_comment() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status_comment, _headers, body) = post! {
        app: app,
        path: format!(
            "/v3/spaces/{}/boards/{}/comments",
            space_pk.to_string(),
            space_post_pk.to_string()
        ),
        headers: test_user.1.clone(),
        body: {
            "content": "comment".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status_comment, 200);

    let (status_report, _headers, _report_body) = post! {
        app: app,
        path: "/v3/reports",
        headers: test_user.1.clone(),
        body: { "space_post_pk": body.pk, "comment_sk": body.sk },
        response_type: ReportContentResponse
    };

    assert_eq!(status_report, 200);

    let (status_get, _headers, body) = get! {
        app: app,
        path: format!(
            "/v3/spaces/{}/boards/{}/comments",
            space_pk.to_string(),
            space_post_pk.to_string()
        ),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<SpacePostCommentResponse>
    };
    assert_eq!(status_get, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].is_report, true);
    assert_eq!(body.items[0].reports, 1);
}

pub async fn setup_deliberation_space() -> (TestContextV3, Partition, Partition) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let (_status_post, _headers, create_post_res) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let post_pk = create_post_res.post_pk;

    let (_status_patch, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Deliberation Post",
            "content": "<p>This is a deliberation post</p>",
            "publish": true
        }
    };

    let (status_space, _headers, create_space_res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": SpaceType::Deliberation,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    assert_eq!(status_space, 200);

    let space_pk = create_space_res.space_pk;
    let now = chrono::Utc::now().timestamp();

    let (status_space_post, _headers, create_space_post_res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "space boards title".to_string(),
            "html_contents": "<div>space boards desc</div>".to_string(),
            "category_name": "space_category".to_string(),
            "urls": [],
            "files": [],
            "started_at": now,
            "ended_at": now
        },
        response_type: CreateSpacePostResponse
    };
    assert_eq!(status_space_post, 200);

    let (status_publish_space, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PUBLIC",
        }
    };
    assert_eq!(status_publish_space, 200);

    (ctx, space_pk, create_space_post_res.space_post_pk)
}
