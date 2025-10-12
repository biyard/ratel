use crate::controllers::v3::posts::{CreatePostResponse, PostDetailResponse};
use crate::models::feed::PostComment;
use crate::tests::v3_setup::TestContextV3;
use crate::types::ListItemsResponse;
use crate::*;

#[tokio::test]
async fn test_add_comment() {
    let (ctx, post_pk) = setup_post().await;
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = ctx;

    let content = format!("<p>This is a comment {now}</p>");

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": &content
        },
        response_type: serde_json::Value,
    };

    assert_eq!(status, 200);
    assert_eq!(body["pk"].as_str().unwrap(), post_pk);
    assert_eq!(body["sk"].as_str().is_some(), true);
    assert_eq!(body["content"].as_str().unwrap(), content);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        response_type: PostDetailResponse,
    };

    assert_eq!(status, 200);
    assert!(body.comments.len() >= 1);
    assert_eq!(body.comments[0].content, content);
}

#[tokio::test]
async fn test_reply_to_comment() {
    let (ctx, post_pk) = setup_post().await;
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = ctx;

    let content = format!("<p>This is a comment {now}</p>");

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": &content
        },
        response_type: PostComment,
    };

    assert_eq!(status, 200);

    let comment_sk = body.sk;

    let reply_content = format!("<p>This is a reply to comment {now}</p>");

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments/{}", post_pk.to_string(), comment_sk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": &reply_content
        },
        response_type: serde_json::Value,
    };

    assert_eq!(status, 200);
    assert_eq!(
        body["pk"].as_str().unwrap(),
        format!("POST_REPLY#{}", post_pk.to_string())
    );
    assert_eq!(body["content"].as_str().unwrap(), reply_content);
    assert_eq!(
        body["parent_comment_sk"].as_str().unwrap(),
        comment_sk.to_string()
    );

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}/comments/{}", post_pk.to_string(), comment_sk.to_string()),
        response_type: ListItemsResponse<PostComment>,
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].content, reply_content);
    assert_eq!(body.items[0].parent_comment_sk, Some(comment_sk));
}

async fn setup_post() -> (TestContextV3, String) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    let (_status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let (_status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Comment addition",
            "content": "<p>Comment addition contents</p>",
            "publish": true
        }
    };

    return (ctx, post_pk);
}
