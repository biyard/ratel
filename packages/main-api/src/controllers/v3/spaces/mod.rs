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
pub mod participate_space;
pub use create_space::*;
pub use delete_space::*;
pub use dto::*;
pub use get_space::*;
pub use list_spaces::*;
use participate_space::participate_space_handler;
pub use update_space::*;

pub mod sprint_leagues;

use crate::{features::spaces::SpaceParticipant, models::SpaceCommon, *};

pub fn route() -> Result<Router<AppState>> {
    let app_state = AppState::default();

    Ok(Router::new()
        .route(
            "/:space_pk",
            delete(delete_space_handler)
                .patch(update_space_handler)
                .get(get_space_handler),
        )
        .nest(
            "/:space_pk",
            Router::new()
                .nest("/members", members::route())
                .nest("/files", files::route())
                .nest("/panels", panels::route())
                .nest("/recommendations", recommendations::route())
                .nest("/discussions", discussions::route())
                .nest("/artworks", artworks::route())
                .nest("/boards", boards::route())
                .nest("/polls", polls::route())
                .nest("/sprint-leagues", sprint_leagues::route()),
        )
        // Above all, apply user participant instead of real user.
        // Real user will be passed only when space admin access is needed.
        .layer(middleware::from_fn_with_state(app_state, inject_space))
        .route("/:space_pk/participate", post(participate_space_handler))
        .route("/", post(create_space_handler).get(list_spaces_handler)))
}

pub async fn inject_space(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, Error> {
    tracing::debug!("Project authorization middleware");

    // Extract request parts to access headers and URI
    let (mut parts, body) = req.into_parts();

    // Extract project_id from the URI path
    let path = parts.uri.path();
    let path_segments: Vec<&str> = path.split('/').collect();
    let space_pk = path_segments[1].to_string();

    tracing::debug!("Verifying project access for space_id: {}", space_pk,);

    let space_pk: Partition = space_pk.parse()?;

    let space = SpaceCommon::get(
        &state.dynamo.client,
        space_pk,
        Some(EntityType::SpaceCommon),
    )
    .await
    .map_err(|e| {
        tracing::error!("failed to get space common from db: {:?}", e);
        crate::Error::SpaceNotFound
    })?
    .ok_or(crate::Error::SpaceNotFound)?;

    if let Ok(user) = User::from_request_parts(&mut parts, &state).await {
        if space.is_published()
            && space.should_explicit_participation()
            && !space.is_space_admin(&state.dynamo.client, &user).await
        {
            // Check if the user is a participant
            if let Ok(Some(SpaceParticipant {
                display_name,
                profile_url,
                username,
                user_type,
                ..
            })) = SpaceParticipant::get(
                &state.dynamo.client,
                CompositePartition(space.pk.clone(), user.pk.clone()),
                Some(EntityType::SpaceParticipant),
            )
            .await
            {
                // Participant mode
                let user: &mut User = parts.extensions.get_mut().unwrap();
                user.display_name = display_name;
                user.username = username;
                user.profile_url = profile_url;
                user.user_type = user_type;
            } else {
                // Viewer mode
                parts.extensions.remove::<User>();
            }
        }
    }

    parts.extensions.insert(space);

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
