pub mod create_space;
pub mod delete_space;
pub mod list_spaces;
pub mod update_space;

pub mod boards;
pub mod discussions;
pub mod files;
pub mod members;
pub mod panels;
pub mod polls;
pub mod recommendations;

pub mod dto;

pub mod get_space;
#[cfg(test)]
pub mod tests;

pub mod artworks;
pub use create_space::*;
pub use delete_space::*;
pub use dto::*;
pub use get_space::*;
pub use list_spaces::*;
pub use update_space::*;

pub mod sprint_leagues;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
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
                    "/invitations",
                    crate::controllers::v3::spaces::members::route(),
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
                .nest(
                    "/artworks",
                    crate::controllers::v3::spaces::artworks::route(),
                )
                .nest("/boards", crate::controllers::v3::spaces::boards::route())
                .nest("/polls", crate::controllers::v3::spaces::polls::route())
                .nest(
                    "/sprint-leagues",
                    crate::controllers::v3::spaces::sprint_leagues::route(),
                ),
        ))
}
