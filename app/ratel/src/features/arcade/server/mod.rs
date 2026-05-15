//! Raw axum router for arcade endpoints that the Dioxus
//! `#[get]`/`#[post]` macros can't represent — currently just the
//! SSE realtime stream.

use crate::common::axum::Router;

pub fn router() -> Router {
    crate::features::arcade::realtime::sse::router()
}
