use percent_encoding::NON_ALPHANUMERIC;
use tokio::time::sleep;
use validator::ValidateLength;

use crate::{
    controllers::v3::posts::{PostDetailResponse, create_post::CreatePostResponse},
    tests::v3_setup::TestContextV3,
};

use crate::*;

#[tokio::test]
async fn test_create_post_by_user() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    assert_eq!(status, 200, "get post response {:?}", body);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();
    let _images = vec!["https://example.com/image1.png".to_string()];

    let title = "Updated Title";
    let content = "<p>Updated Content</p>";

    let path = format!("/v3/posts/{}", post_pk.to_string());

    // Writing
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: test_user.1.clone(),
        body: {
            "title": title,
            "content": content
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["title"], title);
    assert_eq!(body["html_contents"], content);

    // Info
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: test_user.1.clone(),
        body: {
            "visibility": "PUBLIC"
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["visibility"], "PUBLIC");

    // Publish
    let (status, _headers, body) = patch! {
        app: app,
        path: &path,
        headers: test_user.1.clone(),
        body: {
            "title": title,
            "content": title,
            "publish": true
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
    };
    assert_eq!(status, 200, "get post response {:?}", body);
    assert_eq!(body["post"]["pk"], post_pk);
}

#[tokio::test]
async fn test_block_read_draft_post_from_guest() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", body.post_pk.to_string()),
    };
    assert_eq!(status, 401);
    assert_eq!(body["code"], 110);
}

#[tokio::test]
async fn test_create_post_with_invalid_team() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1,
        body: {
            "team_pk": "TEAM#invalid"
        }
    };

    assert_eq!(status, 404);
    assert_eq!(body["code"], 4000);
}

#[tokio::test]
async fn test_post_like() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    assert_eq!(status, 200, "get post response {:?}", body);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/posts/{}/likes", post_pk),
        headers: test_user.1.clone(),
        body: { "like": true }
    };
    assert_eq!(status, 200);
    assert_eq!(body["like"], true);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: serde_json::Value,
    };

    assert_eq!(status, 200);
    assert_eq!(body["post"]["likes"], 1);
    assert_eq!(body["is_liked"], true);
}

#[tokio::test]
async fn test_delete_draft() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    assert_eq!(status, 200, "get post response {:?}", body);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 1);
}

#[tokio::test]
async fn test_delete_post() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone()
    };
    assert_eq!(status, 200, "get post response {:?}", body);
    assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

    let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();

    let (status, _headers, body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "DELETE POST TITLE",
            "content": "<p>Some HTMLContents</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
    };
    assert_eq!(status, 200, "get post response {:?}", body);
    assert_eq!(body["post"]["pk"], post_pk);

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200);
    assert_eq!(body["status"], 2);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone()
    };
    assert_eq!(status, 404, "get post response {:?}", body);
    assert_eq!(body["code"], 2001);
}

#[tokio::test]
async fn test_list_posts() {
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = TestContextV3::setup().await;

    for i in 0..11 {
        let (status, _headers, create_body) = post! {
            app: app,
            path: "/v3/posts",
            headers: test_user.1.clone(),
            response_type: CreatePostResponse
        };

        assert_eq!(status, 200);
        assert!(create_body.post_pk.to_string().len() > 0);

        let (status, _headers, body) = get! {
            app: app,
            path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
            headers: test_user.1.clone()
        };
        assert_eq!(status, 200, "get post response {:?}", body);
        assert_eq!(body["post"]["pk"], create_body.post_pk.to_string());

        let post_pk = body["post"]["pk"].as_str().unwrap_or_default().to_string();
        let title = format!("Updated Title {} {}", now, i);
        let content = format!("<p>Updated Content {} {}</p>", now, i);

        // Writing
        let (status, _headers, _body) = patch! {
            app: app,
            path: format!("/v3/posts/{}", post_pk.to_string()),
            headers: test_user.1.clone(),
            body: {
                "title": title,
                "content": content
            }
        };

        assert_eq!(status, 200);

        let (status, _headers, body) = patch! {
            app: app,
            path: format!("/v3/posts/{}", post_pk.to_string()),
            headers: test_user.1.clone(),
            body: {
                "visibility": "PUBLIC"
            }
        };

        assert_eq!(status, 200);
        assert_eq!(body["visibility"], "PUBLIC");

        let (status, _headers, body) = patch! {
            app: app,
            path: format!("/v3/posts/{}", post_pk),
            headers: test_user.1.clone(),
            body: {
                "title": title,
                "content": content,
                "visibility": "PUBLIC",
                "publish": true
            }
        };

        assert_eq!(status, 200);
        assert_eq!(body["status"], 2);

        sleep(std::time::Duration::from_millis(10)).await; // ensure the order by created_at
    }

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts"),
        response_type: serde_json::Value,
    };

    assert_eq!(status, 200);
    let items = body["items"].as_array().unwrap();
    let bookmark = body["bookmark"].as_str().unwrap_or_default().to_string();
    assert!(bookmark.len() > 0);
    assert!(items.len() >= 5);

    let first = items[0].as_object().unwrap();
    assert_eq!(first["title"], format!("Updated Title {} 10", now));
    assert_eq!(
        first["html_contents"],
        format!("<p>Updated Content {} 10</p>", now)
    );
    assert_eq!(items.len(), 10);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts?bookmark={}", percent_encoding::utf8_percent_encode(&bookmark, NON_ALPHANUMERIC).to_string()),
        response_type: serde_json::Value,
    };
    assert_eq!(status, 200);
    let items = body["items"].as_array().unwrap();
    assert!(bookmark.len() > 0);
    let first = items[0].as_object().unwrap();
    assert_eq!(first["title"], format!("Updated Title {} 0", now));
    assert_eq!(
        first["html_contents"],
        format!("<p>Updated Content {} 0</p>", now)
    );
}

#[tokio::test]
async fn test_delete_post_by_guest() {
    // TODO: failure test
}

#[tokio::test]
async fn test_delete_post_by_other_no_permitted() {
    // TODO:  failure test
}

#[tokio::test]
async fn test_delete_post_by_other_permitted() {
    // TODO:  success test
}

// Tests for get_post_handler
#[tokio::test]
async fn test_get_post_when_authenticated() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Create a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200);
    assert!(create_body.post_pk.to_string().len() > 0);

    // Update and publish the post
    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Post",
            "content": "<p>Test Content</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Get the post
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    let post = body.post.unwrap();
    assert_eq!(post.pk, create_body.post_pk);
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.html_contents, "<p>Test Content</p>");
    assert_eq!(body.is_liked, false);
}

#[tokio::test]
async fn test_get_published_post_when_not_authenticated() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Create and publish a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Public Post",
            "content": "<p>Public Content</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Get the post without authentication
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", create_body.post_pk.to_string()),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    let post = body.post.unwrap();
    assert_eq!(post.pk, create_body.post_pk);
    assert_eq!(post.title, "Public Post");
}

#[tokio::test]
async fn test_get_post_with_comments() {
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = TestContextV3::setup().await;

    // Create and publish a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let post_pk = create_body.post_pk.to_string();

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "Post with Comments",
            "content": "<p>Some HTMLContents</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Add comments
    let comment_content = format!("<p>Test comment {}</p>", now);
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments", post_pk),
        headers: test_user.1.clone(),
        body: {
            "content": &comment_content
        }
    };
    assert_eq!(status, 200);

    // Get the post with comments
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    assert!(body.comments.len() >= 1);
    assert_eq!(body.comments[0].content, comment_content);
}

#[tokio::test]
async fn test_get_post_with_like() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Create and publish a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let post_pk = create_body.post_pk.to_string();

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "Post with Like",
            "content": "<p>Some HTMLContents</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Like the post
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/posts/{}/likes", post_pk),
        headers: test_user.1.clone(),
        body: { "like": true }
    };
    assert_eq!(status, 200);

    // Get the post and verify like status
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    assert_eq!(body.is_liked, true);
    let post = body.post.unwrap();
    assert_eq!(post.likes, 1);
}

#[tokio::test]
async fn test_get_post_with_comment_likes() {
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = TestContextV3::setup().await;

    // Create and publish a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let post_pk = create_body.post_pk.to_string();

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "Post with Comment Likes",
            "content": "<p>Some HTMLContents</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Add a comment
    let comment_content = format!("<p>Test comment {}</p>", now);
    let (status, _headers, comment_body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments", post_pk),
        headers: test_user.1.clone(),
        body: {
            "content": &comment_content
        },
        response_type: serde_json::Value
    };
    assert_eq!(status, 200);
    let comment_sk = comment_body["sk"].as_str().unwrap();

    // Like the comment
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/posts/{}/comments/{}/likes", post_pk, comment_sk),
        headers: test_user.1.clone(),
        body: { "like": true }
    };
    assert_eq!(status, 200);

    // Get the post and verify comment like status
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.comments.len() >= 1);
    assert_eq!(body.comments[0].content, comment_content);
    assert_eq!(body.comments[0].likes, 1);
    assert_eq!(body.comments[0].liked, true);
}

#[tokio::test]
async fn test_get_nonexistent_post() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Try to get a non-existent post
    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/posts/FEED#nonexistent",
        headers: test_user.1.clone()
    };

    assert_eq!(status, 404);
    assert_eq!(body["code"], 2001); // PostNotFound error code
}

#[tokio::test]
async fn test_get_post_permissions() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Create a draft post (not published)
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let post_pk = create_body.post_pk.to_string();

    // Owner should be able to read their own draft
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    assert!(body.permissions as u64 > 0); // Should have permissions
}

#[tokio::test]
async fn test_get_post_guest_no_like_data() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Create and publish a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let post_pk = create_body.post_pk.to_string();

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "Public Post",
            "content": "<p>Some HTMLContents</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Like the post as authenticated user
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/posts/{}/likes", post_pk),
        headers: test_user.1.clone(),
        body: { "like": true }
    };
    assert_eq!(status, 200);

    // Get post as guest (not authenticated)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200);
    assert!(body.post.is_some());
    // Guest should see likes count but is_liked should be false
    assert_eq!(body.is_liked, false);
    let post = body.post.unwrap();
    assert_eq!(post.likes, 1);
}

#[tokio::test]
async fn test_get_post_with_200_comments() {
    let TestContextV3 {
        app,
        test_user,
        now,
        ..
    } = TestContextV3::setup().await;

    // Create and publish a post
    let (status, _headers, create_body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };
    assert_eq!(status, 200);

    let post_pk = create_body.post_pk.to_string();

    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "Post with 200 Comments",
            "content": "<p>This post will have 200 comments</p>",
            "visibility": "PUBLIC",
            "publish": true
        }
    };
    assert_eq!(status, 200);

    // Create 200 comments
    println!("Creating 200 comments...");
    for i in 0..200 {
        let comment_content = format!("<p>Test comment {} - {}</p>", i, now);
        let (status, _headers, _body) = post! {
            app: app,
            path: format!("/v3/posts/{}/comments", post_pk),
            headers: test_user.1.clone(),
            body: {
                "content": &comment_content
            }
        };
        assert_eq!(status, 200, "Failed to create comment {}", i);

        // Add small delay every 50 comments to avoid overwhelming the system
        if i % 50 == 0 && i > 0 {
            println!("Created {} comments...", i);
            sleep(std::time::Duration::from_millis(100)).await;
        }
    }
    println!("Finished creating 200 comments");

    // Get the post with all comments
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", post_pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };

    assert_eq!(status, 200, "Failed to get post with 200 comments");
    assert!(body.post.is_some(), "Post should exist");

    let comments_count = body.comments.len();
    println!("Retrieved {} comments", comments_count);

    // Verify we can retrieve comments (may be paginated)
    assert!(comments_count > 0, "Should retrieve at least some comments");

    // If pagination is implemented, this will be less than 200
    // If not paginated, this should be 200
    println!(
        "Post with 200 comments test completed. Retrieved {} comments",
        comments_count
    );
}
