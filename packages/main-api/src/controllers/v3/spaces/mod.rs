pub mod create_space;
pub mod delete_space;
pub mod list_spaces;
pub mod update_space;

pub mod analyzes;
pub mod boards;
pub mod dao;
pub mod discussions;
pub mod files;
pub mod members;
pub mod panels;
pub mod polls;
pub mod recommendations;
pub mod reports;

pub mod dto;

pub mod get_space;
#[cfg(test)]
pub mod tests;

pub mod artworks;
pub mod participate_space;
pub use create_space::*;
pub use delete_space::*;
pub use dto::*;
use ethers::signers::Signer;
pub use get_space::*;
pub use list_spaces::*;
use participate_space::participate_space_handler;
pub use update_space::*;

pub mod rewards;
pub mod sprint_leagues;

use crate::{
    features::spaces::SpaceParticipant,
    models::{SpaceCommon, Team},
    *,
};

pub fn route() -> Result<Router<AppState>> {
    let app_state = AppState::default();

    Ok(Router::new()
        .nest(
            "/:space_pk",
            Router::new()
                .route(
                    "/",
                    delete(delete_space_handler).patch(update_space_handler),
                )
                .nest("/panels", panels::route())
                .nest("/dao", dao::route())
                // NOTE: Above are TeamAdmin-only routes
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    authorize_team_admin,
                ))
                .nest("/analyzes", analyzes::route())
                .route("/", get(get_space_handler))
                .route("/participate", post(participate_space_handler))
                .nest("/members", members::route())
                .nest("/files", files::route())
                .nest("/recommendations", recommendations::route())
                .nest("/discussions", discussions::route())
                .nest("/artworks", artworks::route())
                .nest("/boards", boards::route())
                .nest("/polls", polls::route())
                .nest("/rewards", rewards::route())
                .nest("/sprint-leagues", sprint_leagues::route())
                .nest("/reports", reports::route()),
        )
        // NOTE: Above all, apply user participant instead of real user.
        // Real user will be passed only when space admin access is needed.
        .layer(middleware::from_fn_with_state(app_state, inject_space))
        .route("/", post(create_space_handler).get(list_spaces_handler)))
}

pub async fn authorize_team_admin(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response<Body>> {
    debug!("Project authorization middleware");
    let (mut parts, body) = req.into_parts();

    let permissions = Permissions::from_request_parts(&mut parts, &state).await?;
    permissions.permitted(TeamGroupPermission::TeamAdmin)?;

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}

pub async fn inject_space(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, Error> {
    debug!("Project authorization middleware");

    // Extract request parts to access headers and URI
    let (mut parts, body) = req.into_parts();

    // Extract project_id from the URI path
    let path = parts.uri.path();
    let path_segments: Vec<&str> = path.split('/').collect();
    let space_pk_encoded = path_segments[1].to_string();

    // URL-decode the space_pk (it may be percent-encoded in the URI)
    let space_pk = urlencoding::decode(&space_pk_encoded)
        .map_err(|_| crate::Error::BadRequest("Invalid URL encoding".to_string()))?
        .to_string();

    debug!("Verifying project access for space_id: {}", space_pk,);

    let space_pk: SpacePartition = space_pk.parse()?;
    let space_pk: Partition = space_pk.into();

    let space: SpaceCommon = SpaceCommon::get(
        &state.dynamo.client,
        space_pk,
        Some(EntityType::SpaceCommon),
    )
    .await
    .map_err(|e| {
        error!("failed to get space common from db: {:?}", e);
        crate::Error::SpaceNotFound
    })?
    .ok_or(crate::Error::SpaceNotFound)?;

    if matches!(space.user_pk, Partition::Team(_)) {
        let team = Team::get(
            &state.dynamo.client,
            space.user_pk.clone(),
            Some(EntityType::Team),
        )
        .await
        .map_err(|e| {
            error!("failed to get team from db: {:?}", e);
            crate::Error::TeamNotFound
        })?
        .ok_or(crate::Error::TeamNotFound)?;

        parts.extensions.insert(team);
    }

    if let Ok(user) = User::from_request_parts(&mut parts, &state).await {
        if space.is_published()
            && space.should_explicit_participation()
            && !space.is_space_admin(&state.dynamo.client, &user).await
        {
            // Check if the user is a participant
            if let Ok(Some(participant)) = SpaceParticipant::get(
                &state.dynamo.client,
                CompositePartition(space.pk.clone(), user.pk.clone()),
                Some(EntityType::SpaceParticipant),
            )
            .await
            {
                parts.extensions.insert(participant.clone());

                let SpaceParticipant {
                    display_name,
                    profile_url,
                    username,
                    user_type,
                    ..
                } = participant;
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
