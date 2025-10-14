use bdk::prelude::*;
use by_axum::axum::extract::{Query, State};

use crate::{
    models::user::UserPhoneNumber,
    tests::{create_app_state, create_test_user},
};

use super::find_user::{FindUserQueryParams, FindUserQueryType, find_user_handler};

#[tokio::test]
async fn test_find_user() {
    let app_state = create_app_state();
    let cli = app_state.dynamo.client.clone();
    let user = create_test_user(&cli).await;
    let phone_number = uuid::Uuid::new_v4().to_string();
    let res = UserPhoneNumber::new(user.pk.clone(), phone_number.clone())
        .create(&cli)
        .await;
    assert!(
        res.is_ok(),
        "Failed to create phone number: {:?}",
        res.err()
    );
    let username = user.username.clone();

    let res = find_user_handler(
        State(app_state.clone()),
        Query(FindUserQueryParams {
            r#type: FindUserQueryType::Username,
            value: username,
        }),
    )
    .await;
    assert!(res.is_ok(), "Failed to find user: {:?}", res.err());
    let res = res.unwrap().0;
    assert_eq!(res.user.pk, user.pk.to_string(), "User PK should match");

    let email = user.email.clone();

    let res = find_user_handler(
        State(app_state.clone()),
        Query(FindUserQueryParams {
            r#type: FindUserQueryType::Email,
            value: email,
        }),
    )
    .await;
    assert!(res.is_ok(), "Failed to find user: {:?}", res.err());
    let res = res.unwrap().0;
    assert_eq!(res.user.pk, user.pk.to_string(), "User PK should match");

    let res = find_user_handler(
        State(app_state.clone()),
        Query(FindUserQueryParams {
            r#type: FindUserQueryType::PhoneNumber,
            value: phone_number,
        }),
    )
    .await;
    assert!(res.is_ok(), "Failed to find user: {:?}", res.err());
    let res = res.unwrap().0;
    assert_eq!(res.user.pk, user.pk.to_string(), "User PK should match");
}
