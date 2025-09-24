use crate::{AppState, Error2, models::email::EmailVerification, utils::time::get_now_timestamp};
use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::axum::extract::{Json, State},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct VerifyCodeRequest {
    #[schemars(description = "Email address used for verification.")]
    pub email: String,
    #[schemars(description = "Verification code sent to user's email.")]
    pub code: String,
}

const MAX_ATTEMPTS: i32 = 3;
pub async fn verify_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<VerifyCodeRequest>,
) -> Result<(), Error2> {
    let now = get_now_timestamp();
    let (verification_list, _) =
        EmailVerification::find_by_email(&dynamo.client, &req.email, Default::default()).await?;

    if verification_list.is_empty() {
        return Err(Error2::NotFound(format!(
            "No verification found for email: {}",
            req.email
        )));
    }

    let email_verification = verification_list[0].clone();

    if email_verification.attempt_count >= MAX_ATTEMPTS {
        EmailVerification::delete(
            &dynamo.client,
            email_verification.pk,
            Some(email_verification.sk),
        )
        .await?;
        return Err(Error2::BadRequest(
            "Maximum verification attempts exceeded".to_string(),
        ));
    }

    if email_verification.expired_at < now {
        EmailVerification::delete(
            &dynamo.client,
            email_verification.pk,
            Some(email_verification.sk),
        )
        .await?;
        return Err(Error2::BadRequest(
            "Verification code has expired".to_string(),
        ));
    }

    if email_verification.value != req.code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(&dynamo.client)
            .await?;
        return Err(Error2::BadRequest("Code mismatch".to_string()));
    }

    Ok(())
}

#[cfg(test)]
pub mod verify_code_tests {
    use dto::by_axum::axum::{Json, extract::State};

    use crate::{
        controllers::v3::auth::verification::{
            send_code::{SendCodeRequest, send_code_handler},
            verify_code::{VerifyCodeRequest, verify_code_handler},
        },
        models::email::EmailVerification,
        tests::create_app_state,
    };

    #[tokio::test]
    async fn test_verify_code_handler() {
        let app_state = create_app_state();
        let email = format!("{}@not.valid", uuid::Uuid::new_v4());
        let res = send_code_handler(
            State(app_state.clone()),
            Json(SendCodeRequest {
                email: email.clone(),
            }),
        )
        .await;
        assert!(res.is_ok(), "Failed to send code: {:?}", res);

        let res = verify_code_handler(
            State(app_state.clone()),
            Json(VerifyCodeRequest {
                email: "wrong@email.com".to_string(),
                code: "SOME_CODE".to_string(),
            }),
        )
        .await;
        assert!(
            res.is_err(),
            "Expected error for wrong email, got {:?}",
            res
        );

        let res = verify_code_handler(
            State(app_state.clone()),
            Json(VerifyCodeRequest {
                email: email.clone(),
                code: "SOME_CODE".to_string(),
            }),
        )
        .await;
        assert!(res.is_err(), "Expected error for wrong code, got {:?}", res);

        let (verification_list, _) = EmailVerification::find_by_email(
            &app_state.dynamo.client,
            email.clone(),
            Default::default(),
        )
        .await
        .expect("Failed to find verification");

        assert!(
            verification_list.len() >= 1,
            "Expected more than 1 verification record, got {}",
            verification_list.len()
        );

        let email_verification = verification_list[0].clone();

        let res = verify_code_handler(
            State(app_state.clone()),
            Json(VerifyCodeRequest {
                email: email.clone(),
                code: email_verification.value.clone(),
            }),
        )
        .await;
        assert!(
            res.is_ok(),
            "Expected success for correct code, got {:?}",
            res
        );
    }
}
