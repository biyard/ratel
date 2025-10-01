// use crate::controllers::v2::posts::list_posts::ListPostsQueryParams;
use crate::controllers::v3::auth::verification::verify_code::VerifyCodeResponse;
use crate::controllers::v3::spaces::deliberations::discussions::create_discussion::create_discussion_handler;
use crate::controllers::v3::spaces::deliberations::discussions::end_recording::end_recording_handler;
use crate::controllers::v3::spaces::deliberations::discussions::exit_meeting::exit_meeting_handler;
use crate::controllers::v3::spaces::deliberations::discussions::participant_meeting::participant_meeting_handler;
// use crate::controllers::v3::spaces::deliberations::discussions::participant_meeting::participant_meeting_handler;
use crate::controllers::v3::spaces::deliberations::discussions::start_meeting::start_meeting_handler;
use crate::controllers::v3::spaces::deliberations::discussions::start_recording::start_recording_handler;
use crate::controllers::v3::spaces::deliberations::posting_deliberation::{
    PostingDeliberationResponse, posting_deliberation_handler,
};
use crate::controllers::v3::spaces::deliberations::responses::create_response_answer::create_response_answer_handler;
use crate::controllers::v3::spaces::deliberations::responses::get_response_answer::get_response_answer_handler;
use crate::models::feed::Post;
// use crate::models::feed::Post;
use crate::models::space::{DeliberationDiscussionResponse, DeliberationSpaceResponse};
use crate::types::list_items_response::ListItemsResponse;
// use crate::types::list_items_response::ListItemsResponse;
use crate::{
    Error2,
    controllers::v3::{
        auth::{
            login::login_handler,
            logout::logout_handler,
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
        posts::{
            comments::add_comment::{AddCommentResponse, add_comment_handler},
            create_post::{CreatePostResponse, create_post_handler},
            delete_post::delete_post_handler,
            get_post::{GetPostResponse, get_post_handler},
            like_post::{LikePostResponse, like_post_handler},
            list_posts::list_posts_handler,
            update_post::{UpdatePostResponse, update_post_handler},
        },
        spaces::deliberations::{
            create_deliberation::{CreateDeliberationResponse, create_deliberation_handler},
            delete_deliberation::delete_deliberation_handler,
            get_deliberation::get_deliberation_handler,
            update_deliberation::update_deliberation_handler,
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
    models::space::DeliberationDetailResponse,
    utils::aws::{DynamoClient, SesClient},
};

use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

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
    pub pool: bdk::prelude::sqlx::PgPool,
}

pub struct RouteDeps {
    pub dynamo_client: DynamoClient,
    pub ses_client: SesClient,
    pub pool: bdk::prelude::sqlx::PgPool,
}

pub fn route(
    RouteDeps {
        dynamo_client,
        ses_client,
        pool,
    }: RouteDeps,
) -> Result<Router, Error2> {
    Ok(Router::new()
        .route(
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
        )
        .nest(
            "/users",
            Router::new().route(
                "/",
                get_with(find_user_handler, api_docs!(Json<FindUserResponse>, "", "")),
            ),
        )
        .nest(
            "/posts",
            Router::new()
                .route(
                    "/",
                    post_with(
                        create_post_handler,
                        api_docs!(Json<CreatePostResponse>, "Create Post", "Create a new post"),
                    )
                    .get_with(
                        list_posts_handler,
                        api_docs!(
                            Json<ListItemsResponse<Post>>,
                            "List Posts",
                            "List all posts"
                        ),
                    ),
                )
                .route(
                    "/:post_pk/like",
                    post_with(
                        like_post_handler,
                        api_docs!(
                            Json<LikePostResponse>,
                            "Like/Unlike Post",
                            "Like or unlike a post by ID"
                        ),
                    ),
                )
                .route(
                    "/:post_pk/comments",
                    post_with(
                        add_comment_handler,
                        api_docs!(
                            Json<AddCommentResponse>,
                            "Add Comment",
                            "Add a comment to a post by ID"
                        ),
                    ),
                )
                .route(
                    "/:post_pk",
                    get_with(
                        get_post_handler,
                        api_docs!(Json<GetPostResponse>, "Get Post", "Get a post by ID"),
                    )
                    .put_with(
                        update_post_handler,
                        api_docs!(
                            Json<UpdatePostResponse>,
                            "Update Post",
                            "Update a post by ID"
                        ),
                    )
                    .delete_with(
                        delete_post_handler,
                        api_docs!((), "Delete Post", "Delete a post by ID"),
                    ),
                ),
        )
        .nest(
            "/auth",
            Router::new()
                .route("/login", post(login_handler))
                .route("/logout", post(logout_handler))
                .route("/signup", post(signup_handler))
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
                                    Json<VerifyCodeResponse>,
                                    "Verify code",
                                    "Verify the provided email verification code"
                                ),
                            ),
                        ),
                ),
        )
        .nest(
            "/spaces",
            Router::new().nest(
                "/deliberation",
                Router::new()
                    .nest(
                        "/:space_pk/responses",
                        Router::new()
                            .route(
                                "/",
                                post_with(
                                    create_response_answer_handler,
                                    api_docs!(
                                        Json<CreateDeliberationResponse>,
                                        "Create response answer",
                                        "Create response answer with survey id"
                                    ),
                                ),
                            )
                            .route(
                                "/:response_pk",
                                get_with(
                                    get_response_answer_handler,
                                    api_docs!(
                                        Json<DeliberationSpaceResponse>,
                                        "Get response answer",
                                        "Get response answer with response id"
                                    ),
                                ),
                            ),
                    )
                    .nest(
                        "/:space_pk/discussions",
                        Router::new()
                            .route(
                                "/",
                                post_with(
                                    create_discussion_handler,
                                    api_docs!(
                                        Json<DeliberationDiscussionResponse>,
                                        "Create discussion",
                                        "Create discussion under deliberation with id"
                                    ),
                                ),
                            )
                            .route(
                                "/:discussion_pk/start-meeting",
                                post_with(
                                    start_meeting_handler,
                                    api_docs!(
                                        Json<DeliberationDiscussionResponse>,
                                        "Start meeting",
                                        "Start meeting for discussion with id"
                                    ),
                                ),
                            )
                            .route(
                                "/:discussion_pk/participant-meeting",
                                post_with(
                                    participant_meeting_handler,
                                    api_docs!(
                                        Json<DeliberationDiscussionResponse>,
                                        "Participant meeting",
                                        "Participant meeting for discussion with id"
                                    ),
                                ),
                            )
                            .route(
                                "/:discussion_pk/start-recording",
                                post_with(
                                    start_recording_handler,
                                    api_docs!(
                                        Json<DeliberationDiscussionResponse>,
                                        "Start recording",
                                        "Start recording for discussion with id"
                                    ),
                                ),
                            )
                            .route(
                                "/:discussion_pk/end-recording",
                                post_with(
                                    end_recording_handler,
                                    api_docs!(
                                        Json<DeliberationDiscussionResponse>,
                                        "End recording",
                                        "End recording for discussion with id"
                                    ),
                                ),
                            )
                            .route(
                                "/:discussion_pk/exit-meeting",
                                post_with(
                                    exit_meeting_handler,
                                    api_docs!(
                                        Json<DeliberationDiscussionResponse>,
                                        "Exit meeting",
                                        "Exit meeting for discussion with id"
                                    ),
                                ),
                            ),
                    )
                    .route(
                        "/",
                        post_with(
                            create_deliberation_handler,
                            api_docs!(
                                Json<CreateDeliberationResponse>,
                                "Create deliberation",
                                "Create a new deliberation"
                            ),
                        ),
                    )
                    .route(
                        "/:space_pk",
                        post_with(
                            update_deliberation_handler,
                            api_docs!(
                                Json<DeliberationDetailResponse>,
                                "Update deliberation",
                                "Update a deliberation"
                            ),
                        ),
                    )
                    .route(
                        "/:space_pk",
                        get_with(
                            get_deliberation_handler,
                            api_docs!(
                                Json<DeliberationDetailResponse>,
                                "Get deliberation",
                                "Get deliberation with ID"
                            ),
                        ),
                    )
                    .route(
                        "/:space_pk/posting",
                        post_with(
                            posting_deliberation_handler,
                            api_docs!(
                                Json<PostingDeliberationResponse>,
                                "Posting deliberation",
                                "Posting deliberation with id"
                            ),
                        ),
                    )
                    .route(
                        "/:space_pk/delete",
                        post_with(
                            delete_deliberation_handler,
                            api_docs!(
                                Json<String>,
                                "Delete deliberation",
                                "Delete deliberation with id"
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
        .with_state(AppState {
            dynamo: dynamo_client,
            ses: ses_client,
            pool,
        }))
}
