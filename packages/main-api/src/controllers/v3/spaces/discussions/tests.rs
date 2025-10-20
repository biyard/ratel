use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::features::spaces::discussions::dto::{
    CreateDiscussionResponse, DeleteDiscussionResponse, GetDiscussionResponse,
    ListDiscussionResponse,
};
use crate::types::{Partition, SpaceType};
use crate::*;
use crate::{
    controllers::v3::posts::CreatePostResponse,
    tests::v3_setup::{TestContextV3, setup_v3},
};
use axum::AxumRouter;

struct CreatedDeliberationSpace {
    space_pk: Partition,
}

#[tokio::test]
async fn test_create_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let users = vec![user.pk];

    let now = chrono::Utc::now().timestamp();

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "discussion title".to_string(), "description": "discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.discussion.is_member, true);
}

#[tokio::test]
async fn test_update_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let users = vec![user.pk.clone()];

    let now = chrono::Utc::now().timestamp();

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "discussion title".to_string(), "description": "discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.discussion.is_member, true);

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/{}/discussions/{}",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "updated discussion title".to_string(), "description": "update discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.discussion.name, "updated discussion title".to_string());
    assert_eq!(
        body.discussion.description,
        "update discussion description".to_string()
    );
}

#[tokio::test]
async fn test_delete_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let users = vec![user.pk];

    let now = chrono::Utc::now().timestamp();

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "discussion title".to_string(), "description": "discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/{}/discussions/{}",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, _body) = delete! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "updated discussion title".to_string(), "description": "update discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: DeleteDiscussionResponse
    };

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_list_discussions_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let users = vec![user.pk];

    let now = chrono::Utc::now().timestamp();

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "discussion title".to_string(), "description": "discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "discussion title".to_string(), "description": "discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: ListDiscussionResponse
    };

    assert_eq!(status, 200);

    tracing::debug!("list discussions body: {:?}", body);
}

#[tokio::test]
async fn test_get_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let users = vec![user.pk];

    let now = chrono::Utc::now().timestamp();

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "started_at": now, "ended_at": now + 10_000, "name": "discussion title".to_string(), "description": "discussion description".to_string(), "user_ids": users.clone()
        },
        response_type: CreateDiscussionResponse
    };

    assert_eq!(status, 200);
    tracing::debug!("discussion body: {:?}", body);

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/discussions/{}",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: GetDiscussionResponse
    };

    assert_eq!(status, 200);

    tracing::debug!("get discussion body: {:?}", body);
}

async fn bootstrap_deliberation_space(
    app: &AxumRouter,
    headers: axum::http::HeaderMap,
) -> CreatedDeliberationSpace {
    let (_status, _headers, post) = post! {
        app: app,
        path: "/v3/posts",
        headers: headers.clone(),
        response_type: CreatePostResponse
    };

    let feed_pk = post.post_pk.clone();

    let (_status, _headers, space) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": SpaceType::Deliberation,
            "post_pk": feed_pk
        },
        response_type: CreateSpaceResponse
    };

    CreatedDeliberationSpace {
        space_pk: space.space_pk,
    }
}
