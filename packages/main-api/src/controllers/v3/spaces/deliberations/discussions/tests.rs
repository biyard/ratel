use crate::{
    controllers::v3::spaces::deliberations::{
        create_deliberation::{CreateDeliberationRequest, create_deliberation_handler},
        discussions::{
            create_discussion::{
                CreateDiscussionRequest, DeliberationDiscussionPath, create_discussion_handler,
            },
            end_recording::end_recording_handler,
            exit_meeting::exit_meeting_handler,
            participant_meeting::participant_meeting_handler,
            start_meeting::{DeliberationDiscussionByIdPath, start_meeting_handler},
            start_recording::start_recording_handler,
        },
    },
    tests::{create_app_state, create_test_user, get_auth},
    types::Partition,
};
use dto::by_axum::axum::{
    Json,
    extract::{Extension, Path, State},
};

#[tokio::test]
async fn test_create_discussion_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_deliberation = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk.clone();
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

    let create_discussion = create_discussion_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionPath {
            deliberation_id: space_pk.to_string(),
        }),
        Json(CreateDiscussionRequest {
            name: "Test discussion title".to_string(),
            description: "Test disscussion description".to_string(),
            started_at: now,
            ended_at: now + 3600,
            members,
        }),
    )
    .await;

    assert!(
        create_discussion.is_ok(),
        "Failed to create discussion {:?}",
        create_discussion.err()
    );
}

#[tokio::test]
async fn test_start_meeting_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_deliberation = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk.clone();
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

    let create_discussion = create_discussion_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionPath {
            deliberation_id: space_pk.to_string(),
        }),
        Json(CreateDiscussionRequest {
            name: "Test discussion title".to_string(),
            description: "Test disscussion description".to_string(),
            started_at: now,
            ended_at: now + 3600,
            members,
        }),
    )
    .await;

    assert!(
        create_discussion.is_ok(),
        "Failed to create discussion {:?}",
        create_discussion.err()
    );

    let discussion = create_discussion.unwrap().0;
    let pk = discussion.pk;

    let start_meeting = start_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.to_string(),
            id: pk.to_string(),
        }),
    )
    .await;

    assert!(
        start_meeting.is_ok(),
        "Failed to start meeting {:?}",
        start_meeting.err()
    );

    let start_meeting = start_meeting.unwrap().0;

    assert!(
        start_meeting.members.len() == 2,
        "Meeting count is not matched"
    );
}

#[tokio::test]
async fn test_create_participants_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_deliberation = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk.clone();
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

    let create_discussion = create_discussion_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionPath {
            deliberation_id: space_pk.to_string(),
        }),
        Json(CreateDiscussionRequest {
            name: "Test discussion title".to_string(),
            description: "Test disscussion description".to_string(),
            started_at: now,
            ended_at: now + 3600,
            members,
        }),
    )
    .await;

    assert!(
        create_discussion.is_ok(),
        "Failed to create discussion {:?}",
        create_discussion.err()
    );

    let discussion = create_discussion.unwrap().0;
    let pk = discussion.pk;

    let start_meeting = start_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.to_string(),
            id: pk.to_string(),
        }),
    )
    .await;

    assert!(
        start_meeting.is_ok(),
        "Failed to start meeting {:?}",
        start_meeting.err()
    );

    let participant_meeting = participant_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.to_string(),
            id: pk.to_string(),
        }),
    )
    .await;

    assert!(
        participant_meeting.is_ok(),
        "Failed to participant meeting {:?}",
        participant_meeting.err()
    );

    let participant_meeting = participant_meeting.unwrap().0;

    eprintln!("participant meeting: {:?}", participant_meeting);

    assert!(
        participant_meeting.participants.len() == 1,
        "Failed to participant meeting",
    );
}

#[tokio::test]
async fn test_exit_meeting_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();
    let create_deliberation = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .unwrap();

    let space_pk = create_deliberation.0.metadata.deliberation.pk.clone();
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

    let create_discussion = create_discussion_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionPath {
            deliberation_id: space_pk.to_string(),
        }),
        Json(CreateDiscussionRequest {
            name: "Test discussion title".to_string(),
            description: "Test disscussion description".to_string(),
            started_at: now,
            ended_at: now + 3600,
            members,
        }),
    )
    .await;

    assert!(
        create_discussion.is_ok(),
        "Failed to create discussion {:?}",
        create_discussion.err()
    );

    let discussion = create_discussion.unwrap().0;
    let pk = discussion.pk;

    let start_meeting = start_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.to_string(),
            id: pk.to_string(),
        }),
    )
    .await;

    assert!(
        start_meeting.is_ok(),
        "Failed to start meeting {:?}",
        start_meeting.err()
    );

    let joined = participant_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.clone(),
            id: pk.clone(),
        }),
    )
    .await;
    assert!(
        joined.is_ok(),
        "participant_meeting failed: {:?}",
        joined.err()
    );
    let me_after_join = joined.unwrap().0;

    let me_user_pk = me_after_join
        .participants
        .iter()
        .find(|p| !p.user_pk.is_empty())
        .map(|p| p.user_pk.clone())
        .unwrap_or_default();

    assert!(
        me_after_join
            .participants
            .iter()
            .any(|p| p.user_pk == me_user_pk),
        "self should be in participants after join"
    );

    let exited = exit_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.clone(),
            id: pk.clone(),
        }),
    )
    .await;
    assert!(exited.is_ok(), "exit_meeting failed: {:?}", exited.err());

    let exited = exited.unwrap().0;

    assert!(
        exited.participants.len() == 0,
        "not matched participants len",
    );
}

#[tokio::test]
async fn test_start_recording_handler() {
    let (app_state, deliberation_id, discussion_id, auth) = setup_disc_with_meeting().await;

    let _joined = participant_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: deliberation_id.clone(),
            id: discussion_id.clone(),
        }),
    )
    .await
    .expect("participant_meeting failed");

    let started_rec = start_recording_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: deliberation_id.clone(),
            id: discussion_id.clone(),
        }),
    )
    .await;

    assert!(
        started_rec.is_ok(),
        "start_recording failed: {:?}",
        started_rec.err()
    );

    let resp = started_rec.unwrap().0;
    assert!(
        !resp.members.is_empty(),
        "members should be present in response"
    );
}

#[tokio::test]
async fn test_end_recording_handler() {
    let (app_state, deliberation_id, discussion_id, auth) = setup_disc_with_meeting().await;

    let _ = start_recording_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: deliberation_id.clone(),
            id: discussion_id.clone(),
        }),
    )
    .await
    .expect("start_recording failed");

    let ended_rec = end_recording_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: deliberation_id.clone(),
            id: discussion_id.clone(),
        }),
    )
    .await;

    assert!(
        ended_rec.is_ok(),
        "end_recording failed: {:?}",
        ended_rec.err()
    );

    let resp = ended_rec.unwrap().0;
    assert!(
        !resp.members.is_empty(),
        "members should be present after ending recording"
    );
}

fn upk(pk: Partition) -> String {
    match pk {
        Partition::User(v) | Partition::Team(v) => v,
        _ => String::new(),
    }
}

async fn setup_disc_with_meeting() -> (
    crate::AppState,
    String,
    String,
    dto::by_axum::auth::Authorization,
) {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();

    let user = create_test_user(&cli).await;
    let auth = get_auth(&user.clone());
    let uid = uuid::Uuid::new_v4().to_string();

    let created_space = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await
    .expect("create_deliberation failed");

    let space_pk = created_space.0.metadata.deliberation.pk.clone();

    let now = chrono::Utc::now().timestamp();
    let members = vec![
        upk(create_test_user(&cli).await.pk),
        upk(create_test_user(&cli).await.pk),
    ];

    let created_disc = create_discussion_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionPath {
            deliberation_id: space_pk.to_string(),
        }),
        Json(CreateDiscussionRequest {
            name: "recording test".into(),
            description: "recording test desc".into(),
            started_at: now,
            ended_at: now + 3600,
            members,
        }),
    )
    .await
    .expect("create_discussion failed");

    let disc_id = created_disc.0.pk;

    let started = start_meeting_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Path(DeliberationDiscussionByIdPath {
            deliberation_id: space_pk.to_string(),
            id: disc_id.to_string(),
        }),
    )
    .await;

    assert!(started.is_ok(), "start_meeting failed: {:?}", started.err());

    (app_state, space_pk.to_string(), disc_id.to_string(), auth)
}
