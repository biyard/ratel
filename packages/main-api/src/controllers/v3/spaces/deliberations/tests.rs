use crate::{
    controllers::v3::spaces::deliberations::create_deliberation::{
        CreateDeliberationRequest, create_deliberation_handler,
    },
    tests::{create_app_state, create_auth, get_test_user},
};
use dto::by_axum::axum::{
    Json,
    extract::{Extension, State},
};

#[tokio::test]
async fn test_create_space_handler() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = get_test_user(&cli).await;
    let auth = create_auth(user.clone()).await;
    let uid = uuid::Uuid::new_v4().to_string();
    let create_res = create_deliberation_handler(
        State(app_state.clone()),
        Extension(Some(auth.clone())),
        Json(CreateDeliberationRequest { feed_id: uid }),
    )
    .await;

    assert!(
        create_res.is_ok(),
        "Failed to create deliberation {:?}",
        create_res.err()
    );
}
