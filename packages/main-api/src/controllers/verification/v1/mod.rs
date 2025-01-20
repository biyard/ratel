use by_axum::{
    axum::{
        extract::{Path, State},
        routing::{get, post},
        Extension, Json,
    },
    log::root,
};
use dto::*;
use rest_api::Signature;
use slog::o;
use crate::utils::email::{send_email, build_content};

#[derive(Clone, Debug)]
pub struct VerificationControllerV1 {
    log: slog::Logger,
}

impl VerificationControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router> {
        let log = root().new(o!("api-controller" => "VerificationControllerV1"));
        let ctrl = VerificationControllerV1 { log };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_verification))
            .route("/:id", get(Self::get_verification))
            .with_state(ctrl.clone()))
    }

    pub async fn act_verification(
        State(ctrl): State<VerificationControllerV1>,
        Extension(sig): Extension<Option<Signature>>,
        Json(body): Json<VerificationActionRequest>,
    ) -> Result<Json<VerificationActionResponse>> {
        let log = ctrl.log.new(o!("api" => "act_verification"));
        slog::debug!(log, "act_verification: sig={:?} {:?}", sig, body);
        let cli = easy_dynamodb::get_client(&log);

        match body {
            VerificationActionRequest::CryptoStance(req) => {
                let id = uuid::Uuid::new_v4().to_string(); // FIXME: use time-based uuid
                let expire_time = 60 * 60 * 24; // 24 hours
                let doc: VerificationCryptoStance = VerificationCryptoStance::new(id.clone(), req.code, expire_time);

                match cli.create(&doc).await {
                    Ok(_) => {
                        // FIXME: title / content of the email
                        match send_email(
                                req.email,
                            build_content("Verification".to_string()),
                             build_content(format!("click here to verify: {}/verification/v1/{}", 
                                    option_env!("BASE_DOMAIN").unwrap_or("http://localhost:3000"), // FIXME: use front-end url
                                    id
                                ))
                            ).await {
                            Ok(_) => {
                                slog::info!(log, "Email sent");
                                Ok(Json(VerificationActionResponse::default()))
                            }
                            Err(e) => {
                                slog::error!(log, "Failed to send email: {:?}", e);
                                Err(ServiceError::from(e))
                            }
                        }
                    }
                    Err(e) => {
                        slog::error!(log, "Failed to create document: {:?}", e);
                        Err(ServiceError::from(e))
                    }
                }
            }
        }
    }

    // FIXME: use front-end verification page
    pub async fn get_verification(
        State(ctrl): State<VerificationControllerV1>,
        Path(id): Path<String>,
        Extension(sig): Extension<Option<Signature>>,
    ) -> Result<Json<VerificationActionResponse>> {
        let log = ctrl.log.new(o!("api" => "get_verification"));
        slog::debug!(log, "get_verification: sig={:?} id={}", sig, id);
        let cli = easy_dynamodb::get_client(&log);

        match cli.get::<VerificationCryptoStance>(&id).await {
            Ok(Some(_)) => {
                let now: u64 = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
                cli
                    .update(
                        &id,
                        vec![
                            ("done_at", now),
                        ],
                    )
                    .await
                    .map_err(|e| ServiceError::from(e))?;
                Ok(Json(VerificationActionResponse::default()))
            },
            Ok(None) => Err(ServiceError::NotFound),
            Err(e) => {
                slog::error!(log, "Failed to get document: {:?}", e);
                Err(ServiceError::from(e))
            }
        }
    }

    pub async fn check_verification(
        State(ctrl): State<VerificationControllerV1>,
        Extension(sig): Extension<Option<Signature>>,
    ) -> Result<Json<VerificationActionResponse>> {
        let log = ctrl.log.new(o!("api" => "check_verification"));
        slog::debug!(log, "check_verification: sig={:?}", sig);
        // TODO: check the request is verified

        Ok(Json(VerificationActionResponse::default()))
    } 
}
