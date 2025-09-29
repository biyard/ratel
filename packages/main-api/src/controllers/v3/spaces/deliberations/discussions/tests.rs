use crate::{
    controllers::v3::spaces::deliberations::{
        create_deliberation::{CreateDeliberationRequest, create_deliberation_handler},
        discussions::create_discussion::{
            CreateDiscussionRequest, DeliberationDiscussionPath, create_discussion_handler,
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
