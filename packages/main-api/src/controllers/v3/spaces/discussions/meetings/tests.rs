use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::features::spaces::discussions::dto::{
    CreateDiscussionResponse, SpaceDiscussionResponse,
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
async fn test_start_meeting_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

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

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/start-meeting",
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

#[tokio::test]
async fn test_participant_meeting_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

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

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/start-meeting",
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

    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/participant-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: SpaceDiscussionResponse
    };

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_exit_meeting_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

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

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/start-meeting",
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

    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/participant-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: SpaceDiscussionResponse
    };

    assert_eq!(status, 200);

    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/exit-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: SpaceDiscussionResponse
    };

    tracing::debug!("exit meeting: {:?}", body.clone());

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_recording() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    let CreatedDeliberationSpace { space_pk, .. } =
        bootstrap_deliberation_space(&app, headers.clone()).await;

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/{}/discussions", space_pk_encoded);

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

    let discussion_pk = body.discussion.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");
    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/start-meeting",
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

    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/start-recording",
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
    assert!(body.media_pipeline_arn.is_some());

    let path = format!(
        "/v3/spaces/{}/discussions/{}/meetings/end-recording",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, _body) = patch! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: SpaceDiscussionResponse
    };

    assert_eq!(status, 200);
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
