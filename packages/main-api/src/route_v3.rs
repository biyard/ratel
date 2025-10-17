use crate::controllers::v3::assets::complete_multipart_upload::complete_multipart_upload;
use crate::controllers::v3::assets::get_put_multi_object_uri::get_put_multi_object_uri;
use crate::controllers::v3::assets::get_put_object_uri::AssetPresignedUris;
use crate::controllers::v3::assets::get_put_object_uri::get_put_object_uri;
use crate::controllers::v3::auth::verification::verify_code::VerifyCodeResponse;
use crate::controllers::v3::me::list_my_drafts::list_my_drafts_handler;
use crate::controllers::v3::me::list_my_posts::list_my_posts_handler;
use crate::controllers::v3::posts::list_comments::list_comments_handler;
use crate::controllers::v3::posts::post_response::PostResponse;
use crate::controllers::v3::posts::reply_to_comment::reply_to_comment_handler;
use crate::controllers::v3::promotions::get_top_promotion::get_top_promotion_handler;
use crate::controllers::v3::spaces::deliberations::discussions::create_discussion::create_discussion_handler;
use crate::controllers::v3::spaces::deliberations::discussions::end_recording::end_recording_handler;
use crate::controllers::v3::spaces::deliberations::discussions::exit_meeting::exit_meeting_handler;
use crate::controllers::v3::spaces::deliberations::discussions::get_discussion::get_discussion_handler;
use crate::controllers::v3::spaces::deliberations::discussions::get_meeting::{
    MeetingData, get_meeting_handler,
};
use crate::controllers::v3::spaces::deliberations::discussions::participant_meeting::participant_meeting_handler;
use crate::controllers::v3::spaces::deliberations::discussions::start_meeting::start_meeting_handler;
use crate::controllers::v3::spaces::deliberations::discussions::start_recording::start_recording_handler;
use crate::controllers::v3::spaces::deliberations::get_deliberation_common::{
    GetDeliberationCommonResponse, get_deliberation_common_handler,
};
use crate::controllers::v3::spaces::deliberations::get_deliberation_deliberation::{
    GetDeliberationDeliberationResponse, get_deliberation_deliberation_handler,
};
use crate::controllers::v3::spaces::deliberations::get_deliberation_poll::get_deliberation_poll_handler;
use crate::controllers::v3::spaces::deliberations::get_deliberation_recommendation::get_deliberation_recommendation_handler;
use crate::controllers::v3::spaces::deliberations::get_deliberation_summary::get_deliberation_summary_handler;
use crate::controllers::v3::spaces::deliberations::posting_deliberation::{
    PostingDeliberationResponse, posting_deliberation_handler,
};
use crate::controllers::v3::spaces::deliberations::responses::create_response_answer::{
    DeliberationResponse, create_response_answer_handler,
};
use crate::controllers::v3::spaces::deliberations::responses::get_response_answer::get_response_answer_handler;
use crate::controllers::v3::spaces::get_files::get_files_handler;
use crate::controllers::v3::spaces::get_files::GetSpaceFileResponse;
use crate::controllers::v3::spaces::get_space_handler;
use crate::controllers::v3::spaces::polls::dto::*;
use crate::controllers::v3::spaces::deliberations::update_deliberation_deliberation::{
    UpdateDeliberationDeliberationResponse, update_deliberation_deliberation_handler,
};
use crate::controllers::v3::spaces::deliberations::update_deliberation_poll::{
    UpdateDeliberationPollResponse, update_deliberation_poll_handler,
};
use crate::controllers::v3::spaces::deliberations::update_deliberation_recommendation::{
    UpdateDeliberationRecommendationResponse, update_deliberation_recommendation_handler,
};
use crate::controllers::v3::spaces::deliberations::update_deliberation_summary::{
    UpdateDeliberationSummaryResponse, update_deliberation_summary_handler,
};
use crate::controllers::v3::spaces::polls::respond_poll_space::{
    RespondPollSpaceResponse, respond_poll_space_handler,
};
use crate::controllers::v3::spaces::polls::update_poll_space::{
    UpdatePollSpaceResponse, update_poll_space_handler,
};
use crate::controllers::v3::spaces::update_files::update_files_handler;
use crate::controllers::v3::spaces::update_files::UpdateSpaceFileResponse;
use crate::controllers::v3::spaces::{dto::*, list_spaces_handler};
use crate::models::space::{DeliberationDiscussionResponse, DeliberationSpaceResponse};
use crate::models::{
    DeliberationContentResponse, DeliberationSurveyResponse, SpaceCommon, feed::*,
};
use crate::types::list_items_response::ListItemsResponse;
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
        posts::*,
        spaces::deliberations::{
            delete_deliberation::delete_deliberation_handler,
            get_deliberation::get_deliberation_handler,
            update_deliberation::update_deliberation_handler,
        },
        spaces::polls::{
            get_poll_space::{GetPollSpaceResponse, get_poll_space_handler},
            get_survey_summary::get_poll_space_survey_summary,
        },
        spaces::{
            create_space::{CreateSpaceResponse, create_space_handler},
            delete_space::delete_space_handler,
            update_space::update_space_handler,
        },
        teams::{
            create_team::{CreateTeamResponse, create_team_handler},
            delete_team::{DeleteTeamResponse, delete_team_handler},
            find_team::{FindTeamResponse, find_team_handler},
            get_permissions::{GetPermissionsResponse, get_permissions_handler},
            get_team::{GetTeamResponse, get_team_handler},
            groups::{
                add_member::add_member_handler,
                create_group::{CreateGroupResponse, create_group_handler},
                delete_group::{DeleteGroupResponse, delete_group_handler},
                remove_member::remove_member_handler,
                update_group::update_group_handler,
            },
            list_members::{TeamMember, list_members_handler},
            list_team_posts::list_team_posts_handler,
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
        .nest(
            "/networks",
            Router::new().route(
                "/suggestions",
                get_with(
                    crate::controllers::v3::networks::get_suggestions_handler,
                    api_docs!(
                        crate::controllers::v3::networks::GetSuggestionsResponse,
                        "Get Suggestions",
                        "Get user and team suggestions for the logged-in user"
                    ),
                ),
            ),
        )
        .route("/promotions/top", get(get_top_promotion_handler))
        .nest(
            "/me",
            Router::new()
                .route(
                    "/",
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
                .route(
                    "/posts",
                    get_with(
                        list_my_posts_handler,
                        api_docs!(
                            Json<ListItemsResponse<PostResponse>>,
                            "List My Posts",
                            "List all posts created by the logged-in user"
                        ),
                    ),
                )
                .route(
                    "/drafts",
                    get_with(
                        list_my_drafts_handler,
                        api_docs!(
                            Json<ListItemsResponse<PostResponse>>,
                            "List My Posts",
                            "List all posts created by the logged-in user"
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
                            Json<ListItemsResponse<PostResponse>>,
                            "List Posts",
                            "List all posts"
                        ),
                    ),
                )
                .route(
                    "/:post_pk/likes",
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
                            Json<PostComment>,
                            "Add Comment",
                            "Add a comment to a post by ID"
                        ),
                    ),
                )
                .route(
                    "/:post_pk/comments/:comment_sk",
                    post_with(
                        reply_to_comment_handler,
                        api_docs!(
                            Json<PostComment>,
                            "Reply to Comment",
                            "Add a comment to a comment"
                        ),
                    )
                    .get_with(
                        list_comments_handler,
                        api_docs!(
                            Json<ListItemsResponse<PostComment>>,
                            "List Comments on a comment",
                            "List all comments on a comment"
                        ),
                    ),
                )
                .route(
                    "/:post_pk/comments/:comment_sk/likes",
                    post_with(
                        like_comment_handler,
                        api_docs!(
                            Json<LikeCommentResponse>,
                            "Like Comment",
                            "Like a comment"
                        ),
                    )
                )
                .route(
                    "/:post_pk",
                    get_with(
                        get_post_handler,
                        api_docs!(Json<PostDetailResponse>, "Get Post", "Get a post by ID"),
                    )
                    .patch_with(
                        update_post_handler,
                        api_docs!(Json<Post>, "Update Post", "Update a post by ID"),
                    )
                    .delete_with(
                        delete_post_handler,
                        api_docs!(Json<Post>, "Delete Post", "Delete a post by ID"),
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
            Router::new()
                .route(
                    "/",
                    post_with(
                        create_space_handler,
                        api_docs!(
                            Json<CreateSpaceResponse>,
                            "Create Space",
                            "Create a new space"
                        ),
                    ).get_with(
                        list_spaces_handler,
                        api_docs!(
                            Json<ListItemsResponse<SpaceCommon>>,
                            "List Spaces",
                            "List all spaces"
                        ),
                    ),
                )
                .route(
                    "/:space_pk",
                    delete_with(
                        delete_space_handler,
                        api_docs!((), "Delete Space", "Delete a space by ID"),
                    )
                    .patch_with(
                        update_space_handler,
                        api_docs!(
                            Json<SpaceCommonResponse>,
                            "Update Space",
                            "Update space details"
                        ),
                    ).get(get_space_handler),
                )
                .nest("/:space_pk", Router::new()
                    // FILE feature
                    .nest(
                        "/files", 
                        Router::new()
                            .route(
                                "/",
                                patch_with(
                                    update_files_handler,
                                    api_docs!(
                                            Json<UpdateSpaceFileResponse>,
                                            "Update Files",
                                            "Update Files by space pk"
                                        ),
                                )
                            )
                            .route(
                                "/",
                                get_with(
                                    get_files_handler,
                                    api_docs!(
                                            Json<GetSpaceFileResponse>,
                                            "Get Files",
                                            "Get Files by space pk"
                                        ),
                                )
                            )
                    )
                )
                .nest(
                    "/:space_pk/deliberation",
                    Router::new()
                        .nest(
                            "/responses",
                            Router::new()
                                .route(
                                    "/",
                                    post_with(
                                        create_response_answer_handler,
                                        api_docs!(
                                            Json<DeliberationResponse>,
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
                            "/discussions",
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
                                    "/:discussion_pk",
                                    get_with(
                                        get_discussion_handler,
                                        api_docs!(
                                            Json<DeliberationDiscussionResponse>,
                                            "Get Discussion",
                                            "Get Discussion with id"
                                        ),
                                    ),
                                )
                                .route(
                                    "/:discussion_pk/meeting",
                                    get_with(
                                        get_meeting_handler,
                                        api_docs!(
                                            Json<MeetingData>,
                                            "Get Discussion Meeting",
                                            "Get Discussion Meeting with discussion id"
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
                            "/common",
                            get_with(
                                get_deliberation_common_handler,
                                api_docs!(
                                    Json<GetDeliberationCommonResponse>,
                                    "Deliberation Common",
                                    "Get Deliberation Common Response"
                                ),
                            ),
                        )
                        .route(
                            "/summary",
                            get_with(
                                get_deliberation_summary_handler,
                                api_docs!(
                                    Json<DeliberationContentResponse>,
                                    "Deliberation Summary",
                                    "Get Deliberation Summary Response"
                                ),
                            ),
                        )
                        .route(
                            "/deliberation",
                            get_with(
                                get_deliberation_deliberation_handler,
                                api_docs!(
                                    Json<GetDeliberationDeliberationResponse>,
                                    "Deliberation Deliberation",
                                    "Get Deliberation Deliberation Response"
                                ),
                            ),
                        )
                        .route(
                            "/poll",
                            get_with(
                                get_deliberation_poll_handler,
                                api_docs!(
                                    Json<DeliberationSurveyResponse>,
                                    "Deliberation Poll",
                                    "Get Deliberation Poll Response"
                                ),
                            ),
                        )
                        .route(
                            "/recommendation",
                            get_with(
                                get_deliberation_recommendation_handler,
                                api_docs!(
                                    Json<DeliberationContentResponse>,
                                    "Deliberation Recommendation",
                                    "Get Deliberation Recommendation Response"
                                ),
                            ),
                        )
                        .route(
                            "/summary",
                            patch_with(
                                update_deliberation_summary_handler,
                                api_docs!(
                                    Json<UpdateDeliberationSummaryResponse>,
                                    "Deliberation Summary",
                                    "Update Deliberation Summary with space id"
                                ),
                            ),
                        )
                        .route(
                            "/deliberation",
                            patch_with(
                                update_deliberation_deliberation_handler,
                                api_docs!(
                                    Json<UpdateDeliberationDeliberationResponse>,
                                    "Deliberation Deliberation",
                                    "Update Deliberation Deliberation with space id"
                                ),
                            ),
                        )
                        .route(
                            "/poll",
                            patch_with(
                                update_deliberation_poll_handler,
                                api_docs!(
                                    Json<UpdateDeliberationPollResponse>,
                                    "Deliberation Poll",
                                    "Update Deliberation Poll with space id"
                                ),
                            ),
                        )
                        .route(
                            "/recommendation",
                            patch_with(
                                update_deliberation_recommendation_handler,
                                api_docs!(
                                    Json<UpdateDeliberationRecommendationResponse>,
                                    "Deliberation Recommendation",
                                    "Update Deliberation Recommendation with space id"
                                ),
                            ),
                        )
                        
                        .route(
                            "/",
                            // FIXME: this method will be deprecated
                            post_with(
                                update_deliberation_handler,
                                api_docs!(
                                    Json<DeliberationDetailResponse>,
                                    "Update deliberation",
                                    "Update a deliberation"
                                ),
                            )
                            // FIXME: this method will be deprecated
                            .get_with(
                                get_deliberation_handler,
                                api_docs!(
                                    Json<DeliberationDetailResponse>,
                                    "Get deliberation",
                                    "Get deliberation with ID"
                                ),
                            )
                            .delete_with(
                                delete_deliberation_handler,
                                api_docs!(
                                    Json<String>,
                                    "Delete deliberation",
                                    "Delete deliberation with id"
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
                        ),
                )
                .nest(
                    "/:space_pk/polls",
                    Router::new()
                        .route(
                            "/",
                            get_with(
                                get_poll_space_handler,
                                api_docs!(
                                    Json<GetPollSpaceResponse>,
                                    "Get poll",
                                    "Get poll with ID"
                                ),
                            )
                            .put_with(
                                update_poll_space_handler,
                                api_docs!(
                                    Json<UpdatePollSpaceResponse>,
                                    "Update poll",
                                    "Update poll with ID"
                                ),
                            ),
                        )
                        .route(
                            "/responses",
                            post_with(
                                respond_poll_space_handler,
                                api_docs!(
                                    Json<RespondPollSpaceResponse>,
                                    "Respond to poll",
                                    "Submit a response to the poll with Pk"
                                ),
                            ),
                        )
                        .route(
                            "/summary",
                            get_with(
                                get_poll_space_survey_summary,
                                api_docs!(
                                    Json<PollSpaceSurveySummary>,
                                    "Get poll survey summary",
                                    "Get survey summary for the poll with Pk"
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
                .route(
                    "/permissions",
                    get_with(
                        get_permissions_handler,
                        api_docs!(
                            Json<GetPermissionsResponse>,
                            "Get permissions",
                            "Check if user has specific permission for a team"
                        ),
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
                            )
                            .delete_with(
                                delete_team_handler,
                                api_docs!(
                                    Json<DeleteTeamResponse>,
                                    "Delete team",
                                    "Delete a team and all related data (owner only)"
                                ),
                            ),
                        )
                        .route(
                            "/members",
                            get_with(
                                list_members_handler,
                                api_docs!(
                                    Json<ListItemsResponse<TeamMember>>,
                                    "List team members",
                                    "List all members of a team with their groups. Use query param: ?team_pk=TEAM%23uuid or ?team_pk=username"
                                ),
                            ),
                        )
                        .route(
                            "/posts",
                            get_with(
                                list_team_posts_handler,
                                api_docs!(
                                    Json<ListItemsResponse<PostResponse>>,
                                    "List team posts",
                                    "List all posts for a team. Supports query params: ?status=1 (draft) or ?status=2 (published), ?bookmark=..."
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
                                            )
                                            .delete_with(
                                                delete_group_handler,
                                                api_docs!(
                                                    Json<DeleteGroupResponse>,
                                                    "Delete group",
                                                    "Delete a group and all related data (owner only)"
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
        .nest("/assets", Router::new()
            .route(
                "/",
                get_with(
                    get_put_object_uri,
                    api_docs!(
                        Json<AssetPresignedUris>,
                        "Get Presigned Url",
                        "Get Presigned Url"
                    ),
                ),
            )
            .route(
                "/multiparts",
                get_with(
                    get_put_multi_object_uri,
                    api_docs!(
                        Json<AssetPresignedUris>,
                        "Get Multi Object Presigned Url",
                        "Get Multi Object Presigned Url"
                    ),
                ),
            )
            .route(
                "/multiparts/complete",
                post_with(
                    complete_multipart_upload,
                    api_docs!(
                        Json<String>,
                        "Checking Multipart upload complete",
                        "Checking Multipart upload complete"
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
