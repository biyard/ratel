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
use crate::controllers::v3::spaces::create_discussion::create_discussion_handler;
use crate::controllers::v3::spaces::delete_discussion::delete_discussion_handler;
use crate::controllers::v3::spaces::get_discussion::get_discussion_handler;
use crate::controllers::v3::spaces::get_files::get_files_handler;
use crate::controllers::v3::spaces::get_space_handler;
use crate::controllers::v3::spaces::list_discussions::list_discussions_handler;
use crate::controllers::v3::spaces::update_discussion::update_discussion_handler;
use crate::controllers::v3::spaces::update_files::update_files_handler;
use crate::controllers::v3::spaces::{dto::*, list_spaces_handler};
use crate::models::Post;
use crate::models::PostComment;
use crate::models::SpaceCommon;
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
            list_members::{ListMembersResponse, list_members_handler},
            update_team::{UpdateTeamResponse, update_team_handler},
        },
        users::find_user::{FindUserResponse, find_user_handler},
    },
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
                    // FIXME: relocate in files directory in this logic
                    .nest(
                        "/files", 
                        Router::new()
                            .route(
                                "/",
                                    patch(
                                    update_files_handler,
                                )
                                .get(
                                    get_files_handler,
                                )
                            )
                    )
                    // DISCUSSION feature
                    // FIXME: relocate in discussions directory in this logic
                    .nest(
                        "/discussions", 
                        Router::new()
                            .route(
                                "/",
                                post(
                                    create_discussion_handler,
                                )
                                .get(
                                    list_discussions_handler
                                )
                            )
                            .route(
                                "/:discussion_pk",
                                patch(
                                    update_discussion_handler
                                )
                                .get(
                                    get_discussion_handler
                                )
                                .delete(
                                    delete_discussion_handler
                                )
                            )
                    )
                                    .nest("/polls", crate::controllers::v3::spaces::polls::route())
                )
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
                                    Json<ListMembersResponse>,
                                    "List team members",
                                    "List all members of a team with their groups"
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
