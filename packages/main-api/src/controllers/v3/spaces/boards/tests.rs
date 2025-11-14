use crate::controllers::v3::posts::CreatePostResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::controllers::v3::spaces::boards::{CreateSpacePostResponse, DeleteSpacePostResponse};
use crate::features::spaces::boards::dto::list_space_posts_response::ListSpacePostsResponse;
use crate::features::spaces::boards::dto::space_post_response::SpacePostResponse;
use crate::tests::v3_setup::TestContextV3;
use crate::types::{Answer, ChoiceQuestion, EntityType, Partition, Question};
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
            "title": "Deliberation Post",
            "content": "<p>This is a deliberation post</p>",
            "publish": true
        }
    };

    // Create a deliberation space
    let (status, _headers, create_space_res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": SpaceType::Deliberation,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    assert_eq!(status, 200);

    let space_pk = create_space_res.space_pk;
    let now = chrono::Utc::now().timestamp();

    // Create a space board post
    let (status, _headers, create_space_post_res) = post! {
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
    assert_eq!(status, 200);

    let (status, _headers, _res) = patch! {
        app: app,
        path: format!("/v3/spaces/{}", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "publish": true,
            "visibility": "PUBLIC",
        }
    };
    assert_eq!(status, 200);

    (ctx, space_pk, create_space_post_res.space_post_pk)
}

#[tokio::test]
async fn test_get_space_post() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };

    tracing::debug!("get space body: {:?}", body);

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_delete_space_post() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: DeleteSpacePostResponse
    };

    tracing::debug!("delete space body: {:?}", body);

    assert_eq!(status, 200);

    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: SpacePostResponse
    };

    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_list_categories() {
    let (ctx, space_pk, _space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards/categories", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: Vec<String>
    };

    tracing::debug!("list categories body: {:?}", body);

    assert_eq!(status, 200);
    assert_eq!(body.len(), 1);
    assert_eq!(body[0], "space_category".to_string());
}

#[tokio::test]
async fn test_list_space_posts_by_category() {
    let (ctx, space_pk, _space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards?category=space_category", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: ListSpacePostsResponse
    };

    tracing::debug!("list space posts body: {:?}", body);

    assert_eq!(status, 200);
    assert_eq!(body.posts.len(), 1);
}

#[tokio::test]
async fn test_list_space_posts() {
    let (ctx, space_pk, _space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/boards", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: ListSpacePostsResponse
    };

    tracing::debug!("list space posts body: {:?}", body);

    assert_eq!(status, 200);
    assert_eq!(body.posts.len(), 1);
}

#[tokio::test]
async fn test_update_space_posts() {
    let (ctx, space_pk, space_post_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;
    let now = chrono::Utc::now().timestamp();

    let (status, _headers, update_space_post_res) = patch! {
        app: app,
         path: format!("/v3/spaces/{}/boards/{}", space_pk.to_string(), space_post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "update space boards title".to_string(),
            "html_contents": "<div>update space boards desc</div>".to_string(),
            "category_name": "space_category".to_string(),
            "urls": [],
            "files": [],
            "started_at": now,
            "ended_at": now
        },
        response_type: SpacePostResponse
    };
    assert_eq!(status, 200);
    assert_eq!(
        update_space_post_res.title,
        "update space boards title".to_string()
    );
    assert_eq!(
        update_space_post_res.html_contents,
        "<div>update space boards desc</div>".to_string()
    );
}
