//! SSE endpoint for arcade channels (PR4e).
//!
//! `GET /api/arcade/events?channel=<kind:inner>` — opens a
//! long-lived `text/event-stream` response, validates the caller's
//! session, looks up the registered `RoomChannel` handler by kind,
//! runs its `authorize` hook for the initial snapshot, and then
//! pumps `ServerEvent`s from the broadcast hub into the SSE stream
//! as they arrive.
//!
//! Designed to run on Lambda Function URL with
//! `InvokeMode::RESPONSE_STREAM` (PR4e infra commit). On `cargo run`
//! / tests it behaves like a normal long-poll response so unit tests
//! can drive it without Lambda-specific plumbing.
//!
//! Multi-invocation fan-out for chat is layered on top by the DDB
//! Stream listener (PR4f) — each SSE invocation receives the same
//! Stream record and calls `hub.publish(...)` locally for its own
//! subscribers.

use crate::common::axum::{
    Router,
    extract::Query,
    response::sse::{Event as SseEvent, KeepAlive, Sse},
    routing::get,
};
use crate::common::models::auth::User;
use crate::features::arcade::ArcadeError;
use crate::features::arcade::realtime::channel::{ChannelContext, ChannelId, ServerEvent};
use crate::features::arcade::realtime::hub::global_hub;
use futures::stream::{Stream, StreamExt};
use serde::Deserialize;
use std::convert::Infallible;
use tokio::sync::broadcast;

// ── Router ──────────────────────────────────────────────────────────

pub fn router() -> Router {
    Router::new().route("/api/arcade/events", get(events_handler))
}

// ── Query params ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EventsQuery {
    /// `<kind>:<inner>` channel id, e.g. `fof.chat:abc123`.
    pub channel: String,
}

// ── Handler ─────────────────────────────────────────────────────────

/// SSE handler. Authenticates via the standard `User` extractor (so
/// session cookies / bypass work the same as the rest of the app),
/// then defers per-channel authorization to the registered
/// `RoomChannel` impl.
pub async fn events_handler(
    user: User,
    Query(query): Query<EventsQuery>,
) -> Result<Sse<impl Stream<Item = Result<SseEvent, Infallible>>>, ArcadeError> {
    let hub = global_hub();
    let channel = ChannelId(query.channel);

    let handler = hub
        .handler_for(&channel)
        .await
        .map_err(|_| ArcadeError::ChannelUnknown)?;

    let user_id = user
        .pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user.pk.to_string())
        .to_string();
    let ctx = ChannelContext {
        user_id: user_id.clone(),
    };

    let initial_state = handler
        .authorize(&ctx, &channel, serde_json::Value::Null)
        .await
        .map_err(|_| ArcadeError::ChannelForbidden)?;

    let rx = hub.subscribe_stream(&channel).await;
    let stream = make_event_stream(channel.clone(), initial_state, rx);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

// ── Stream wiring ───────────────────────────────────────────────────

/// Build the SSE event stream: one synthetic `subscribed` event with
/// the handler's initial snapshot, followed by every `ServerEvent`
/// the broadcast receiver yields.
fn make_event_stream(
    channel: ChannelId,
    initial_state: serde_json::Value,
    rx: broadcast::Receiver<ServerEvent>,
) -> impl Stream<Item = Result<SseEvent, Infallible>> {
    let initial = SseEvent::default()
        .event("subscribed")
        .json_data(serde_json::json!({
            "channel": channel.0,
            "initial_state": initial_state,
        }))
        .expect("subscribed event serialization");

    let initial_stream = futures::stream::iter(std::iter::once(Ok(initial)));

    let live_stream = futures::stream::unfold(rx, |mut rx| async move {
        loop {
            match rx.recv().await {
                Ok(evt) => {
                    let sse = SseEvent::default()
                        .event(&evt.name)
                        .id(evt.id.to_string())
                        .json_data(serde_json::json!({
                            "channel": evt.channel.0,
                            "payload": evt.payload,
                        }))
                        .ok();
                    if let Some(sse) = sse {
                        return Some((Ok(sse), rx));
                    }
                    // Serialization failure (should not happen for our
                    // controlled payloads). Drop the event and keep
                    // pumping.
                    continue;
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    // Slow subscriber missed events. SSE clients
                    // reconnect with Last-Event-ID; for now drop and
                    // continue.
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    });

    initial_stream.chain(live_stream)
}
