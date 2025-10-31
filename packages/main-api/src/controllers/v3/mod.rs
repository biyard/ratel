use crate::{AppState, Error, models::user::User, types::*};
use axum::extract::State;
use bdk::prelude::*;

pub mod did;
pub mod networks;

pub mod promotions {
    pub mod get_top_promotion;
}
pub mod me {
    pub mod get_info;
    pub mod update_user;

    pub mod list_my_drafts;
    pub mod list_my_posts;
    #[cfg(test)]
    pub mod tests;
}
pub mod users {
    pub mod find_user;

    #[cfg(test)]
    pub mod tests;
}

pub mod assets {
    pub mod complete_multipart_upload;
    pub mod get_put_multi_object_uri;
    pub mod get_put_object_uri;
}

pub mod auth {
    pub mod login;
    pub mod logout;
    pub mod reset_password;
    pub mod signup;

    #[cfg(test)]
    pub mod tests;

    pub mod verification {
        pub mod send_code;
        pub mod verify_code;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod spaces;

pub mod teams {
    pub mod create_team;
    pub mod delete_team;
    pub mod find_team;
    pub mod get_team;
    pub mod list_members;
    pub mod list_team_posts;
    pub mod update_team;

    pub mod dto;
    #[cfg(test)]
    pub mod tests;

    pub mod groups {
        pub mod add_member;
        pub mod create_group;
        pub mod delete_group;
        pub mod remove_member;
        pub mod update_group;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod posts;

pub mod memberships;

use crate::*;
use crate::{
    assets::{
        complete_multipart_upload::complete_multipart_upload,
        get_put_multi_object_uri::get_put_multi_object_uri, get_put_object_uri::get_put_object_uri,
    },
    auth::{
        login::login_handler,
        logout::logout_handler,
        reset_password::reset_password_handler,
        signup::signup_handler,
        verification::{send_code::send_code_handler, verify_code::verify_code_handler},
    },
    me::{
        get_info::get_info_handler, list_my_drafts::list_my_drafts_handler,
        list_my_posts::list_my_posts_handler, update_user::update_user_handler,
    },
    posts::*,
    promotions::get_top_promotion::get_top_promotion_handler,
    spaces::{
        create_space::create_space_handler, delete_space::delete_space_handler, get_space_handler,
        list_spaces_handler, update_space::update_space_handler,
    },
    teams::{
        create_team::create_team_handler,
        delete_team::delete_team_handler,
        find_team::find_team_handler,
        get_team::get_team_handler,
        groups::{
            add_member::add_member_handler, create_group::create_group_handler,
            delete_group::delete_group_handler, remove_member::remove_member_handler,
            update_group::update_group_handler,
        },
        list_members::list_members_handler,
        list_team_posts::list_team_posts_handler,
        update_team::update_team_handler,
    },
    users::find_user::find_user_handler,
    utils::{
        aws::{DynamoClient, SesClient},
        telegram::ArcTelegramBot,
    },
};
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub struct RouteDeps {
    pub dynamo_client: DynamoClient,
    pub ses_client: SesClient,
    pub pool: bdk::prelude::sqlx::PgPool,
    pub bot: Option<ArcTelegramBot>,
}

pub fn route(
    RouteDeps {
        dynamo_client,
        ses_client,
        pool,
        bot,
    }: RouteDeps,
) -> Result<Router> {
    Ok(Router::new()
        .nest("/did", did::route()?)
        .nest(
            "/networks",
            Router::new().route(
                "/suggestions",
                get(crate::controllers::v3::networks::get_suggestions_handler),
            ),
        )
        .route("/promotions/top", get(get_top_promotion_handler))
        .nest(
            "/me",
            Router::new()
                .route("/", get(get_info_handler).patch(update_user_handler))
                .route("/posts", get(list_my_posts_handler))
                .route("/drafts", get(list_my_drafts_handler)),
        )
        .nest("/users", Router::new().route("/", get(find_user_handler)))
        .nest(
            "/posts",
            Router::new()
                .route("/", post(create_post_handler).get(list_posts_handler))
                .route("/:post_pk/likes", post(like_post_handler))
                .route("/:post_pk/comments", post(add_comment_handler))
                .route(
                    "/:post_pk/comments/:comment_sk",
                    post(reply_to_comment_handler).get(list_comments_handler),
                )
                .route(
                    "/:post_pk/comments/:comment_sk/likes",
                    post(like_comment_handler),
                )
                .route(
                    "/:post_pk",
                    get(get_post_handler)
                        .patch(update_post_handler)
                        .delete(delete_post_handler),
                ),
        )
        .nest(
            "/auth",
            Router::new()
                .route("/login", post(login_handler))
                .route("/logout", post(logout_handler))
                .route("/signup", post(signup_handler))
                .route("/reset", post(reset_password_handler))
                .nest(
                    "/verification",
                    Router::new()
                        .route("/send-verification-code", post(send_code_handler))
                        .route("/verify-code", post(verify_code_handler)),
                ),
        )
        .nest(
            "/spaces",
            Router::new()
                .route("/", post(create_space_handler).get(list_spaces_handler))
                .route(
                    "/:space_pk",
                    delete(delete_space_handler)
                        .patch(update_space_handler)
                        .get(get_space_handler),
                )
                .layer(Extension(bot.clone()))
                .nest(
                    "/:space_pk",
                    Router::new()
                        .nest(
                            "/verifications",
                            crate::controllers::v3::spaces::invitations::route(),
                        )
                        .nest("/files", crate::controllers::v3::spaces::files::route())
                        .nest("/panels", crate::controllers::v3::spaces::panels::route())
                        .nest(
                            "/recommendations",
                            crate::controllers::v3::spaces::recommendations::route(),
                        )
                        .nest(
                            "/discussions",
                            crate::controllers::v3::spaces::discussions::route(),
                        )
                        .nest("/polls", crate::controllers::v3::spaces::polls::route())
                        .nest(
                            "/sprint-leagues",
                            crate::controllers::v3::spaces::sprint_leagues::route(),
                        ),
                ),
        )
        .nest(
            "/teams",
            Router::new()
                .route("/", post(create_team_handler).get(find_team_handler))
                .nest(
                    "/:team_pk",
                    Router::new()
                        .route(
                            "/",
                            get(get_team_handler)
                                .patch(update_team_handler)
                                .delete(delete_team_handler),
                        )
                        .route("/members", get(list_members_handler))
                        .route("/posts", get(list_team_posts_handler))
                        .nest(
                            "/groups",
                            Router::new().route("/", post(create_group_handler)).nest(
                                "/:group_sk",
                                Router::new()
                                    .route(
                                        "/",
                                        post(update_group_handler).delete(delete_group_handler),
                                    )
                                    .route(
                                        "/member",
                                        post(add_member_handler).delete(remove_member_handler),
                                    ),
                            ),
                        ),
                ),
        )
        .nest(
            "/assets",
            Router::new()
                .route("/", get(get_put_object_uri))
                .route("/multiparts", get(get_put_multi_object_uri))
                .route("/multiparts", post(complete_multipart_upload)),
        )
        .with_state(AppState::new(dynamo_client, ses_client, pool)))
}
