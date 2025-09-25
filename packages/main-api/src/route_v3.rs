use crate::{
    Error2,
    controllers::{
        m3::membership::{
            get_membership::{UserInfo, get_user_membership},
            promote_to_admin::promote_user_to_admin,
            set_membership::{SetMembershipResponse, set_user_membership},
        },
        v3::{
            auth::{
                login::{LoginResponse, login_handler},
                signup::signup_handler,
                verification::{
                    send_code::{SendCodeResponse, send_code_handler},
                    verify_code::verify_code_handler,
                },
            },
            me::{
                get_info::{GetInfoResponse, get_info_handler},
                update_user::{UpdateUserResponse, update_user_handler},
            },
            teams::{
                create_team::{CreateTeamResponse, create_team_handler},
                find_team::{FindTeamResponse, find_team_handler},
                get_team::{GetTeamResponse, get_team_handler},
                groups::{
                    add_member::add_member_handler,
                    create_group::{CreateGroupResponse, create_group_handler},
                    remove_member::remove_member_handler,
                    update_group::update_group_handler,
                },
                update_team::{UpdateTeamResponse, update_team_handler},
            },
            users::find_user::{FindUserResponse, find_user_handler},
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
            "/me",
            Router::new().route(
                "/me",
                get_with(
                    get_info_handler,
                    api_docs!(
                        Json<GetInfoResponse>,
                        "Get Logged-in User Info",
                        "Get the user data of the logged-in user"
                    ),
                )
                .patch_with(
                    update_user_handler,
                    api_docs!(
                        Json<UpdateUserResponse>,
                        "Update Logged-in User Info",
                        "Update the user data of the logged-in user"
                    ),
                ),
            ),
        )
        .nest(
            "/users",
            Router::new().route(
                "/",
                get_with(find_user_handler, api_docs!(Json<FindUserResponse>, "", "")),
            ),
        )
        .nest(
            "/auth",
            Router::new()
                .route(
                    "/login",
                    post_with(
                        login_handler,
                        api_docs!(
                            LoginResponse,
                            "User login",
                            "Authenticate user and create a session"
                        ),
                    ),
                )
                .route(
                    "/signup",
                    post_with(
                        signup_handler,
                        api_docs!((), "User signup", "Register a new user account"),
                    ),
                )
                .nest(
                    "/verification",
                    Router::new()
                        .route(
                            "/send-verification-code",
                            post_with(
                                send_code_handler,
                                api_docs!(
                                    Json<SendCodeResponse>,
                                    "Send verification code",
                                    "Send a verification code to the user's email"
                                ),
                            ),
                        )
                        .route(
                            "/verify-code",
                            post_with(
                                verify_code_handler,
                                api_docs!(
                                    (),
                                    "Verify code",
                                    "Verify the provided email verification code"
                                ),
                            ),
                        ),
                ),
        )
        .nest(
            "/teams",
            Router::new()
                .route(
                    "/",
                    post_with(
                        create_team_handler,
                        api_docs!(Json<CreateTeamResponse>, "Create team", "Create a new team"),
                    )
                    .get_with(
                        find_team_handler,
                        api_docs!(Json<FindTeamResponse>, "Find team", "Find a team by ID"),
                    ),
                )
                .nest(
                    "/:team_pk",
                    Router::new()
                        .route(
                            "/",
                            get_with(
                                get_team_handler,
                                api_docs!(
                                    Json<GetTeamResponse>,
                                    "Get team",
                                    "Get team information"
                                ),
                            )
                            .patch_with(
                                update_team_handler,
                                api_docs!(
                                    Json<UpdateTeamResponse>,
                                    "Update team",
                                    "Update team information"
                                ),
                            ),
                        )
                        .nest(
                            "/groups",
                            Router::new()
                                .route(
                                    "/",
                                    post_with(
                                        create_group_handler,
                                        api_docs!(
                                            Json<CreateGroupResponse>,
                                            "Create group",
                                            "Create a new group"
                                        ),
                                    ),
                                )
                                .nest(
                                    "/:group_sk",
                                    Router::new()
                                        .route(
                                            "/",
                                            post_with(
                                                update_group_handler,
                                                api_docs!(
                                                    (),
                                                    "Update group",
                                                    "Update group information"
                                                ),
                                            ),
                                        )
                                        .route(
                                            "/member",
                                            post_with(
                                                add_member_handler,
                                                api_docs!(
                                                    (),
                                                    "Add member",
                                                    "Add a new member to the group"
                                                ),
                                            )
                                            .delete_with(
                                                remove_member_handler,
                                                api_docs!(
                                                    (),
                                                    "Remove member",
                                                    "Remove a member from the group"
                                                ),
                                            ),
                                        ),
                                ),
                        ),
                ),
        )
        .nest(
            "/m3",
            Router::new()
                .nest(
                    "/admin/users/:user_id",
                    Router::new()
                        .route(
                            "/membership",
                            get_with(
                                get_user_membership,
                                api_docs!(
                                    Json<UserInfo>,
                                    "Get User Membership",
                                    "Get membership details for a specific user"
                                ),
                            )
                            .post_with(
                                set_user_membership,
                                api_docs!(
                                    Json<SetMembershipResponse>,
                                    "Set User Membership",
                                    "Set membership type for a specific user"
                                ),
                            ),
                        )
                        .route(
                            "/promote",
                            post_with(
                                promote_user_to_admin,
                                api_docs!(
                                    Json<SetMembershipResponse>,
                                    "Promote User to Admin",
                                    "Promote a user to admin membership"
                                ),
                            ),
                        )
                ),
        )
        .with_state(AppState {
            dynamo: dynamo_client.clone(),
            ses: ses_client.clone(),
        }))
}
