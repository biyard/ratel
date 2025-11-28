mod portone;

use crate::*;

pub fn route() -> Result<Router> {
    let app_state = AppState::default();
    Ok(Router::new()
        .route("/portone", post(portone::portone_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            middleware,
        ))
        .with_state(app_state))
}

pub async fn middleware(
    State(_state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response<Body>> {
    warn!("Project authorization middleware");
    let (mut _parts, body) = req.into_parts();

    // Reconstruct request and continue to the handler
    let req = Request::from_parts(_parts, body);
    Ok(next.run(req).await)
}
