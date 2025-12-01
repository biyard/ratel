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
mod get_space_report;
pub mod participate_space;
pub use create_space::*;
pub use delete_space::*;
pub use dto::*;
use ethers::signers::Signer;
pub use get_space::*;
pub use list_spaces::*;
use participate_space::participate_space_handler;
pub use update_space::*;
use x402_axum::{IntoPriceTag, X402Middleware};
use x402_rs::network::{Network, USDCDeployment};

pub mod rewards;
pub mod sprint_leagues;

use crate::{
    features::spaces::SpaceParticipant,
    models::{SpaceCommon, Team},
    *,
};

pub fn route() -> Result<Router<AppState>> {
    let app_state = AppState::default();
    let x402_config = config::get().x402;
    let x402 = X402Middleware::try_from(x402_config.facilitator_url)
        .unwrap()
        .with_base_url(url::Url::parse("http://localhost:3000/").unwrap());
    let usdc = USDCDeployment::by_network(Network::BaseSepolia);

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
                .nest("/panels", panels::route())
                // NOTE: Above are TeamAdmin-only routes
                .layer(middleware::from_fn_with_state(
                    app_state.clone(),
                    authorize_team_admin,
                ))
                .nest("/members", members::route())
                .nest("/files", files::route())
                .nest("/recommendations", recommendations::route())
                .nest("/discussions", discussions::route())
                .nest("/artworks", artworks::route())
                .nest("/boards", boards::route())
                .nest("/polls", polls::route())
                .nest("/rewards", rewards::route())
                .nest("/sprint-leagues", sprint_leagues::route()),
        )
        // NOTE: Above all, apply user participant instead of real user.
        // Real user will be passed only when space admin access is needed.
        .route("/:space_pk/participate", post(participate_space_handler))
        .layer(middleware::from_fn_with_state(app_state, inject_space))
        .route(
            "/:space_pk/reports",
            get(get_space_report::get_space_report_handler).layer(
                x402.with_price_tag(usdc.amount("0.025").pay_to(x402_config.address()).unwrap())
                    .with_dynamic_price(get_space_report::get_usdt_price_callback()),
            ),
        )
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
    let space_pk = path_segments[1].to_string();

    debug!("Verifying project access for space_id: {}", space_pk,);

    let space_pk: Partition = space_pk.parse()?;

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
