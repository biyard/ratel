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

use create_team::create_team_handler;
use delete_team::delete_team_handler;
use find_team::find_team_handler;
use get_team::get_team_handler;
use groups::{
    add_member::add_member_handler, create_group::create_group_handler,
    delete_group::delete_group_handler, remove_member::remove_member_handler,
    update_group::update_group_handler,
};
use list_members::list_members_handler;
use list_team_posts::list_team_posts_handler;
use update_team::update_team_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
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
                            .route("/", post(update_group_handler).delete(delete_group_handler))
                            .route(
                                "/member",
                                post(add_member_handler).delete(remove_member_handler),
                            ),
                    ),
                ),
        ))
}
