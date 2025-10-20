use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::features::dto::{
    CreateDiscussionResponse, DeleteDiscussionResponse, GetDiscussionResponse,
    ListDiscussionResponse, SpaceDiscussionResponse,
};
use crate::tests::create_test_user;
use crate::types::{Partition, SpaceType};
use crate::*;
use crate::{
    controllers::v3::posts::CreatePostResponse,
    tests::{
        create_app_state,
        v3_setup::{TestContextV3, setup_v3},
    },
};
use axum::AxumRouter;

struct CreatedDeliberationSpace {
    space_pk: Partition,
}

#[tokio::test]
async fn test_create_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let team_1 = create_test_user(&cli).await.pk;
    let team_2 = create_test_user(&cli).await.pk;

    let users = vec![team_1.clone(), team_2.clone()];

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
    assert_eq!(body.discussion.members.len(), 2);
}

#[tokio::test]
async fn test_update_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let team_1 = create_test_user(&cli).await.pk;
    let team_2 = create_test_user(&cli).await.pk;

    let users = vec![team_1.clone(), team_2.clone()];

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
    assert_eq!(body.discussion.members.len(), 2);

    let users = vec![team_1.clone()];
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
    assert_eq!(body.discussion.members.len(), 1);
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
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let team_1 = create_test_user(&cli).await.pk;
    let team_2 = create_test_user(&cli).await.pk;

    let users = vec![team_1.clone(), team_2.clone()];

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
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let team_1 = create_test_user(&cli).await.pk;
    let team_2 = create_test_user(&cli).await.pk;

    let users = vec![team_1.clone(), team_2.clone()];

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
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    // create user
    let team_1 = create_test_user(&cli).await.pk;
    let team_2 = create_test_user(&cli).await.pk;

    let users = vec![team_1.clone(), team_2.clone()];

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

    let (status, _headers, body) = get! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        response_type: GetDiscussionResponse
    };

    assert_eq!(status, 200);

    tracing::debug!("get discussion body: {:?}", body);
}

#[tokio::test]
async fn test_start_meeting_handler() {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = setup_v3().await;

    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

    let team_1 = create_test_user(&cli).await.pk;
    let team_2 = create_test_user(&cli).await.pk;

    let users = vec![team_1.clone(), team_2.clone()];

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
    assert_eq!(body.discussion.members.len(), 2);

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/discussions/{}/start-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: SpaceDiscussionResponse
    };

    assert_eq!(status, 200);
    assert!(body.meeting_id.is_some());
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
