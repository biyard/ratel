use crate::controllers::v3::posts::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::boards::{
    CreateSpacePostResponse, DeleteSpacePostResponse, SpaceLikeSpaceCommentResponse,
};
use crate::features::spaces::boards::dto::list_space_posts_response::ListSpacePostsResponse;
use crate::features::spaces::boards::dto::space_post_response::SpacePostResponse;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::tests::v3_setup::TestContextV3;
use crate::types::{Answer, ChoiceQuestion, EntityType, ListItemsResponse, Partition, Question};
use crate::*;

pub async fn setup_deliberation_space() -> (TestContextV3, Partition, Partition) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    // Create a post first
    let (_status, _headers, create_post_res) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let post_pk = create_post_res.post_pk;

    // Publish the post
    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Poll Post",
            "content": "<p>This is a poll post</p>",
            "publish": true
        }
    };

    // Create a deliberation space
    let (status, _headers, create_space_res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": 1,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    assert_eq!(status, 200);

    let space_pk = create_space_res.space_pk;

    let (status, _headers, create_space_post_res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "space boards title".to_string(),
            "html_contents": "<div>space boards desc</div>".to_string(),
            "category_name": "space_category".to_string(),
            "urls": []
        },
        response_type: CreateSpacePostResponse
    };
    assert_eq!(status, 200);

    (ctx, space_pk, create_space_post_res.space_post_pk)
}

#[tokio::test]
async fn test_add_space_comment() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": "comment 1".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": "comment 2".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };
    assert_eq!(status, 200);
    assert_eq!(body.comments.len(), 2);
    assert_eq!(body.number_of_comments, 2);
}

#[tokio::test]
async fn test_like_space_comment() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": "comment 1".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": "comment 2".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments/{}/likes", space_pk.to_string(), space_post_pk.to_string(), body.sk),
        headers: test_user.1.clone(),
        body: {
            "like": true
        },
        response_type: SpaceLikeSpaceCommentResponse
    };
    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };
    assert_eq!(status, 200);
    assert_eq!(body.comments.len(), 2);
    assert!(body.comments[1].liked == true || body.comments[0].liked == true);
}

#[tokio::test]
async fn test_reply_space_comment() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": "comment 1".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "content": "comment 2".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments/{}", space_pk.to_string(), space_post_pk.to_string(), body.sk),
        headers: test_user.1.clone(),
        body: {
            "content": "reply contents".to_string()
        },
        response_type: SpacePostComment
    };
    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}/comments/{}", space_pk.to_string(), space_post_pk.to_string(), body.sk),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<SpacePostComment>
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };
    assert_eq!(status, 200);
    assert_eq!(body.number_of_comments, 3);
}
