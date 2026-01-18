pub mod create_team;
pub mod delete_team;
pub mod find_team;
pub mod get_team;
pub mod list_members;
pub mod list_team_posts;
pub mod memberships;
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

use crate::{models::Team, *};

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
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
                )
                .nest("/membership", memberships::route()?),
        )
        .layer(middleware::from_fn_with_state(
            AppState::default(),
            inject_team,
        ))
        .route("/", post(create_team_handler).get(find_team_handler)))
}

pub async fn inject_team(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, Error> {
    tracing::debug!("Team authorization middleware");

    // Extract request parts to access headers and URI
    let (mut parts, body) = req.into_parts();

    // Extract team identifier from the URI path
    // Note: Middleware sees the path relative to the nesting point
    // Full path: /v3/teams/{team_identifier}/...
    // Path seen by middleware: /{team_identifier}/... (after nesting at "/:team_pk")
    // team_identifier can be either a Partition (TEAM#uuid) or a username (testteam123)
    let path = parts.uri.path();
    let path_segments: Vec<&str> = path.split('/').collect();

    // path_segments[0] = "" (from leading slash), [1] = team_identifier
    if path_segments.len() < 2 {
        return Err(Error::BadRequest("Invalid team path".into()));
    }

    let team_identifier = path_segments[1].to_string();

    // Try to parse as Partition first (e.g., TEAM#uuid)
    let team = if let Ok(team_pk) = team_identifier.parse::<Partition>() {
        // Look up by PK
        Team::get(&state.dynamo.client, team_pk, Some(EntityType::Team))
            .await?
            .ok_or(Error::TeamNotFound)?
    } else {
        // Look up by username
        let team_results =
            Team::find_by_username_prefix(&state.dynamo.client, team_identifier.clone(), Default::default())
                .await?;

        team_results
            .0
            .into_iter()
            .find(|t| t.username == team_identifier)
            .ok_or(Error::TeamNotFound)?
    };

    parts.extensions.insert(team);

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
