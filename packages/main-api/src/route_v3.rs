use crate::Error2;
use crate::{
    controllers::v3::{
        auth::{
            login::{LoginResponse, login_handler},
            signup::signup_handler,
            verification::{
                send_code::{SendCodeResponse, send_code_handler},
                verify_code::verify_code_handler,
            },
        },
        users::{
            find_user::{FindUserResponse, find_user_handler},
            get_user_info::{GetUserInfoResponse, get_user_info_handler},
        },
    },
    utils::aws::{DynamoClient, SesClient},
};
use dto::by_axum::axum::Json;
use dto::{
    aide::axum::routing::{get_with, post_with},
    by_axum::axum::Router,
};

macro_rules! api_docs {
    ($success_ty:ty, $summary:expr, $description:expr) => {
        |op| {
            op.description($description)
                .summary(concat!("(V3)", $summary))
                .response::<200, $success_ty>()
                .response::<400, Error2>()
        }
    };
}

#[derive(Clone)]
pub struct AppState {
    pub dynamo: DynamoClient,
    pub ses: SesClient,
}

pub struct RouteDeps {
    pub dynamo_client: DynamoClient,
    pub ses_client: SesClient,
}

pub fn route(
    RouteDeps {
        dynamo_client,
        ses_client,
    }: RouteDeps,
) -> Result<Router, Error2> {
    Ok(Router::new()
        .nest(
            "/v3",
            Router::new().nest(
                "/auth",
                Router::new()
                    .route(
                        "/login",
                        post_with(
                            login_handler,
                            api_docs!(LoginResponse, "Login", "Login and receive JWT token"),
                        ),
                    )
                    .route(
                        "/signup",
                        post_with(signup_handler, api_docs!((), "Signup", "Signup user")),
                    )
                    .nest(
                        "/verification",
                        Router::new()
                            .route(
                                "/request_code",
                                post_with(
                                    send_code_handler,
                                    api_docs!(
                                        Json<SendCodeResponse>,
                                        "Request Email Verification Code",
                                        "Request a verification code to be sent to the user's email"
                                    ),
                                ),
                            )
                            .route(
                                "/verify_code",
                                post_with(
                                    verify_code_handler,
                                    api_docs!(
                                        (),
                                        "Verify Email Verification Code",
                                        "Verify the code sent to the user's email"
                                    ),
                                ),
                            ),
                    ),
            ),
        )
        .nest(
            "/users",
            Router::new()
                .route(
                    "/",
                    get_with(
                        get_user_info_handler,
                        api_docs!(
                            Json<GetUserInfoResponse>,
                            "List Users",
                            "Retrieve a list of users"
                        ),
                    ),
                )
                .route(
                    "/find",
                    get_with(
                        find_user_handler,
                        api_docs!(
                            Json<FindUserResponse>,
                            "Find User",
                            "Find a user by username, email, or phone number. Use Query Parameters"
                        ),
                    ),
                ),
        )
        .with_state(AppState {
            dynamo: dynamo_client.clone(),
            ses: ses_client.clone(),
        }))
}
