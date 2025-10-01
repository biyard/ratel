use bdk::prelude::axum::{Extension, Json, extract::State};

use crate::{
    controllers::v3::{
        posts::create_post::{CreatePostRequest, create_post_handler},
        spaces::deliberations::create_deliberation::CreateDeliberationResponse,
    },
    models::space::DeliberationDiscussionResponse,
    post,
    tests::{
        create_app_state, create_test_user, get_auth,
        v3_setup::{TestContextV3, setup_v3},
    },
    types::Partition,
};

#[tokio::test]
async fn test_create_discussion_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);

    let now = chrono::Utc::now().timestamp();

    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };

    let members = vec![team_1, team_2];
    let space_pk = body.metadata.deliberation.pk;
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/discussions", space_pk_encoded);

    eprintln!("space_pk_encoded: {:?}", space_pk_encoded);
    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Test discussion title",
            "description": "Test discussion description",
            "started_at": now,
            "ended_at": now + 3600,
            "members": members
        },
        response_type: DeliberationDiscussionResponse
    };

    eprintln!("create discussion: {:?}", body);

    assert_eq!(status, 200);
}

#[tokio::test]
async fn test_start_meeting_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);

    let now = chrono::Utc::now().timestamp();

    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };

    let members = vec![team_1, team_2];
    let space_pk = body.metadata.deliberation.pk;
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/discussions", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Test discussion title",
            "description": "Test discussion description",
            "started_at": now,
            "ended_at": now + 3600,
            "members": members
        },
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    let discussion_pk = body.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/start-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    assert!(body.members.len() == 2, "Meeting count is not matched");
}

#[tokio::test]
async fn test_create_participants_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);

    let now = chrono::Utc::now().timestamp();

    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };

    let members = vec![team_1, team_2];
    let space_pk = body.metadata.deliberation.pk;
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/discussions", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Test discussion title",
            "description": "Test discussion description",
            "started_at": now,
            "ended_at": now + 3600,
            "members": members
        },
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    let discussion_pk = body.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/participant-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    eprintln!("participant meeting: {:?}", body);

    assert!(
        body.participants.len() == 1,
        "Failed to participant meeting",
    );
}

#[tokio::test]
async fn test_exit_meeting_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: {
            "feed_pk": feed_pk
        },
        response_type: CreateDeliberationResponse
    };

    assert_eq!(status, 200);

    let now = chrono::Utc::now().timestamp();

    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => "".to_string(),
    };

    let members = vec![team_1, team_2];
    let space_pk = body.metadata.deliberation.pk;
    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let path = format!("/v3/spaces/deliberation/{}/discussions", space_pk_encoded);

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {
            "name": "Test discussion title",
            "description": "Test discussion description",
            "started_at": now,
            "ended_at": now + 3600,
            "members": members
        },
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    let discussion_pk = body.pk;
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/start-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, _body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    let path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/participant-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    let me_user_pk: Partition = body
        .participants
        .iter()
        .find_map(|p| match &p.user_pk {
            Partition::User(v) | Partition::Team(v) if !v.is_empty() => Some(p.user_pk.clone()),
            _ => None,
        })
        .unwrap_or_default();

    assert!(
        body.participants.iter().any(|p| p.user_pk == me_user_pk),
        "self should be in participants after join"
    );

    let path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/exit-meeting",
        space_pk_encoded, discussion_pk_encoded
    );

    let (status, _headers, body) = post! {
        app: app,
        path: path.clone(),
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };

    assert_eq!(status, 200);

    eprintln!("exited meeting: {:?}", body);

    assert!(body.participants.len() == 0, "not matched participants len",);
}

#[tokio::test]
async fn test_start_recording_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: { "feed_pk": feed_pk },
        response_type: CreateDeliberationResponse
    };
    assert_eq!(status, 200);

    let space_pk = body.metadata.deliberation.pk.clone();
    let now = chrono::Utc::now().timestamp();

    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => String::new(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => String::new(),
    };
    let members = vec![team_1, team_2];

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let create_disc_path = format!("/v3/spaces/deliberation/{}/discussions", space_pk_encoded);

    let (status, _headers, disc_body) = post! {
        app: app,
        path: create_disc_path,
        headers: headers.clone(),
        body: {
            "name": "recording test",
            "description": "recording test desc",
            "started_at": now,
            "ended_at": now + 3600,
            "members": members
        },
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);

    let discussion_pk = disc_body.pk.clone();
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let start_meeting_path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/start-meeting",
        space_pk_encoded, discussion_pk_encoded
    );
    let (status, _headers, _started_meeting) = post! {
        app: app,
        path: start_meeting_path,
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);

    let start_recording_path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/start-recording",
        space_pk_encoded, discussion_pk_encoded
    );
    let (status, _headers, resp) = post! {
        app: app,
        path: start_recording_path,
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);
    assert!(
        !resp.members.is_empty(),
        "members should be present in response"
    );
}

#[tokio::test]
async fn test_end_recording_handler() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ..
    } = setup_v3().await;

    //FIXME: fix by session and one test code
    let app_state = create_app_state();
    let cli = &app_state.dynamo.client;
    let auth = get_auth(&user);

    let post = create_post_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreatePostRequest { team_pk: None }),
    )
    .await;
    assert!(post.is_ok(), "Failed to create post: {:?}", post);

    let feed_pk = post.unwrap().post_pk.clone();

    // SPACE
    let (status, _headers, created_space) = post! {
        app: app,
        path: "/v3/spaces/deliberation",
        headers: headers.clone(),
        body: { "feed_pk": feed_pk },
        response_type: CreateDeliberationResponse
    };
    assert_eq!(status, 200);

    let space_pk = created_space.metadata.deliberation.pk.clone();
    let now = chrono::Utc::now().timestamp();

    let team_1 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => String::new(),
    };
    let team_2 = match create_test_user(&cli).await.pk {
        Partition::User(v) => v,
        _ => String::new(),
    };
    let members = vec![team_1, team_2];

    let space_pk_encoded = space_pk.to_string().replace('#', "%23");
    let create_disc_path = format!("/v3/spaces/deliberation/{}/discussions", space_pk_encoded);

    let (status, _headers, disc_body) = post! {
        app: app,
        path: create_disc_path,
        headers: headers.clone(),
        body: {
            "name": "recording test",
            "description": "recording test desc",
            "started_at": now,
            "ended_at": now + 3600,
            "members": members
        },
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);

    let discussion_pk = disc_body.pk.clone();
    let discussion_pk_encoded = discussion_pk.to_string().replace('#', "%23");

    let start_meeting_path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/start-meeting",
        space_pk_encoded, discussion_pk_encoded
    );
    let (status, _, _) = post! {
        app: app,
        path: start_meeting_path,
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);

    let start_recording_path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/start-recording",
        space_pk_encoded, discussion_pk_encoded
    );
    let (status, _, _) = post! {
        app: app,
        path: start_recording_path,
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);

    let end_recording_path = format!(
        "/v3/spaces/deliberation/{}/discussions/{}/end-recording",
        space_pk_encoded, discussion_pk_encoded
    );
    let (status, _headers, resp) = post! {
        app: app,
        path: end_recording_path,
        headers: headers.clone(),
        body: {},
        response_type: DeliberationDiscussionResponse
    };
    assert_eq!(status, 200);
    assert!(
        !resp.members.is_empty(),
        "members should be present after ending recording"
    );
}
