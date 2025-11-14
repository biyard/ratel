use crate::controllers::v3::posts::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::boards::tests::setup_deliberation_space;
use crate::controllers::v3::spaces::boards::{
    CreateSpacePostResponse, DeleteSpacePostResponse, SpaceLikeSpaceCommentResponse,
};
use crate::features::spaces::boards::dto::list_space_posts_response::ListSpacePostsResponse;
use crate::features::spaces::boards::dto::space_post_response::SpacePostResponse;
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::tests::v3_setup::TestContextV3;
use crate::types::{Answer, ChoiceQuestion, EntityType, ListItemsResponse, Partition, Question};
use crate::*;

#[tokio::test]
async fn test_add_space_comment() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Add 200 comments
    for i in 0..200 {
        let (status, _headers, _body) = post! {
            app: app,
            path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
            headers: test_user.1.clone(),
            body: {
                "content": format!("comment {}", i + 1)
            },
            response_type: SpacePostComment
        };
        assert_eq!(status, 200, "Failed to create comment {}", i + 1);
    }

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };
    println!(
        "{}",
        format!(
            "/v3/spaces/{}/boards/{}",
            space_pk.to_string(),
            space_post_pk.to_string()
        )
    );
    println!("Body: {:?}", body);
    assert_eq!(status, 200);
    assert_eq!(body.number_of_comments, 200);
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

// #[tokio::test]
// async fn test_add_comment_with_started_space() {
//     let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
//     let TestContextV3 { app, test_user, .. } = ctx;

//     let (status, _, _res) = patch! {
//         app: app,
//         path: format!("/v3/spaces/{}", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "start": true,
//         }
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, _body) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "content": "comment".to_string()
//         },
//         response_type: SpacePostComment
//     };

//     assert_eq!(status, 400);
// }

// #[tokio::test]
// async fn test_like_comment_with_started_space() {
//     let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
//     let TestContextV3 { app, test_user, .. } = ctx;

//     let (status, _headers, body) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "content": "comment".to_string()
//         },
//         response_type: SpacePostComment
//     };

//     assert_eq!(status, 200);

//     let (status, _, _res) = patch! {
//         app: app,
//         path: format!("/v3/spaces/{}", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "start": true,
//         }
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, _body) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/boards/{}/comments/{}/likes", space_pk.to_string(), space_post_pk.to_string(), body.sk),
//         headers: test_user.1.clone(),
//         body: {
//             "like": true
//         },
//         response_type: SpaceLikeSpaceCommentResponse
//     };
//     assert_eq!(status, 400);
// }

// #[tokio::test]
// async fn test_reply_space_comment_with_started_space() {
//     let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
//     let TestContextV3 { app, test_user, .. } = ctx;

//     let (status, _headers, body) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/boards/{}/comments", space_pk.to_string(), space_post_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "content": "comment".to_string()
//         },
//         response_type: SpacePostComment
//     };

//     assert_eq!(status, 200);

//     let (status, _, _res) = patch! {
//         app: app,
//         path: format!("/v3/spaces/{}", space_pk.to_string()),
//         headers: test_user.1.clone(),
//         body: {
//             "start": true,
//         }
//     };

//     assert_eq!(status, 200);

//     let (status, _headers, _body) = post! {
//         app: app,
//         path: format!("/v3/spaces/{}/boards/{}/comments/{}", space_pk.to_string(), space_post_pk.to_string(), body.sk),
//         headers: test_user.1.clone(),
//         body: {
//             "content": "reply contents".to_string()
//         },
//         response_type: SpacePostComment
//     };
//     assert_eq!(status, 400);
// }
