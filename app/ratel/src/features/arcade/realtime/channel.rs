//! Realtime channel — arcade-level pub/sub boundary (이음매 2).
//!
//! Transport-agnostic. v1 implementation publishes to an in-process
//! `tokio::sync::broadcast` hub keyed by `ChannelId`; PR4e wires the
//! SSE endpoint (`/api/arcade/events`) to drain subscriber receivers
//! into a `text/event-stream` response, and a separate polling
//! endpoint (`/api/arcade/poll`) to fall back when SSE drops.
//!
//! Why this lives at arcade level, not Ratel-wide: see design doc
//! 2026-05-15 § A2/A2'. Trait is named `RoomChannel` (domain-neutral)
//! and `arcade/realtime/` does not import any game code, so when a
//! second use case (notifications, live comments) needs SSE the
//! module relocates to `common/realtime/` with one import-path
//! sweep.

use crate::common::*;
use crate::features::arcade::ArcadeError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

// ── Identifiers ─────────────────────────────────────────────────────

/// Channel id in the form `<kind>:<inner>` (e.g. `"fof.round:abc"`,
/// `"user:alice"`). `kind` selects the registered `RoomChannel`
/// implementation; `inner` is opaque to the framework.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelId(pub String);

impl ChannelId {
    pub fn kind(&self) -> &str {
        self.0.splitn(2, ':').next().unwrap_or("")
    }
    pub fn inner(&self) -> &str {
        self.0.splitn(2, ':').nth(1).unwrap_or("")
    }
    pub fn from_parts(kind: &str, inner: &str) -> Self {
        ChannelId(format!("{kind}:{inner}"))
    }
}

/// Caller identity passed to every `RoomChannel` callback. Decoupled
/// from `common::models::auth::User` so the trait stays trivially
/// testable.
#[derive(Debug, Clone)]
pub struct ChannelContext {
    pub user_id: String,
}

// ── Server → client event ──────────────────────────────────────────

/// One server-to-client event. Carried inside an SSE `data:` field
/// (PR4e) or a polling response (`GET /api/arcade/poll`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerEvent {
    /// Monotonic per-channel sequence. SSE clients send the last seen
    /// `id` via `Last-Event-ID` on reconnect; polling clients pass
    /// `?since={id}`.
    pub id: u64,
    /// Event name within the channel (e.g. "stage_changed").
    pub name: String,
    /// Channel id this event belongs to. Echoed so a multi-channel
    /// subscriber can demux client-side.
    pub channel: ChannelId,
    /// JSON payload. Game-specific event types deserialize from this.
    pub payload: serde_json::Value,
}

// ── Trait ───────────────────────────────────────────────────────────

/// One implementation per channel kind. The framework looks up the
/// registered handler by `kind()` and routes subscribe / publish
/// calls through it.
#[async_trait]
pub trait RoomChannel: Send + Sync + 'static {
    /// Channel kind. Must match the prefix in `ChannelId` (e.g.
    /// `"fof.round"` for `fof.round:abc`).
    fn kind(&self) -> &'static str;

    /// Subscribe request. Caller has already authenticated; this is
    /// the per-channel authorization + initial-state hook.
    /// Returns the snapshot the client should render before any
    /// streamed event arrives.
    async fn authorize(
        &self,
        ctx: &ChannelContext,
        channel: &ChannelId,
        params: serde_json::Value,
    ) -> crate::common::Result<serde_json::Value>;

    /// Optional per-subscriber redaction hook. Called once per
    /// (subscriber, event) before delivery. Return `None` to drop
    /// the event for that subscriber (e.g. insider info on FOF).
    /// Default: deliver as-is.
    async fn before_publish(
        &self,
        _channel: &ChannelId,
        _subscriber: &ChannelContext,
        event: ServerEvent,
    ) -> Option<ServerEvent> {
        Some(event)
    }
}

// ── In-process hub ──────────────────────────────────────────────────

/// Per-channel state: monotonic counter + `tokio::sync::broadcast`
/// sender. PR4e attaches receivers to SSE response streams.
struct ChannelState {
    next_id: u64,
    tx: broadcast::Sender<ServerEvent>,
}

impl ChannelState {
    fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { next_id: 1, tx }
    }
}

/// In-memory pub/sub hub. One process, one hub. Multi-instance
/// fan-out (when we run more than one Lambda invocation per channel)
/// lands later — design doc § Open question b.
#[derive(Clone)]
pub struct InProcessChannel {
    handlers: Arc<Mutex<HashMap<&'static str, Arc<dyn RoomChannel>>>>,
    channels: Arc<Mutex<HashMap<ChannelId, ChannelState>>>,
    /// Broadcast channel capacity per (ChannelId). Slow subscribers
    /// that lag past this lose events; the SSE wrapper will drop the
    /// connection so the client reconnects + replays via
    /// `Last-Event-ID`.
    capacity: usize,
}

impl InProcessChannel {
    pub fn new() -> Self {
        Self::with_capacity(64)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            channels: Arc::new(Mutex::new(HashMap::new())),
            capacity,
        }
    }

    /// Register a handler. Last registration wins so tests can swap
    /// in mocks.
    pub async fn register<H: RoomChannel>(&self, handler: H) {
        let mut h = self.handlers.lock().await;
        h.insert(handler.kind(), Arc::new(handler));
    }

    /// Resolve a handler by channel kind, or return
    /// `ArcadeError::ChannelUnknown`.
    pub async fn handler_for(
        &self,
        channel: &ChannelId,
    ) -> crate::common::Result<Arc<dyn RoomChannel>> {
        let h = self.handlers.lock().await;
        h.get(channel.kind())
            .cloned()
            .ok_or_else(|| ArcadeError::ChannelUnknown.into())
    }

    /// Authorize a subscribe and return the initial snapshot.
    /// Caller is responsible for subsequently attaching to
    /// [`Self::subscribe_stream`].
    pub async fn authorize_subscribe(
        &self,
        ctx: &ChannelContext,
        channel: &ChannelId,
        params: serde_json::Value,
    ) -> crate::common::Result<serde_json::Value> {
        let handler = self.handler_for(channel).await?;
        handler.authorize(ctx, channel, params).await
    }

    /// Attach a fresh receiver to the channel's broadcast bus. The
    /// SSE response layer (PR4e) will drain this into the response.
    pub async fn subscribe_stream(
        &self,
        channel: &ChannelId,
    ) -> broadcast::Receiver<ServerEvent> {
        let mut chans = self.channels.lock().await;
        let state = chans
            .entry(channel.clone())
            .or_insert_with(|| ChannelState::new(self.capacity));
        state.tx.subscribe()
    }

    /// Publish an event to every subscriber of `channel`. Assigns a
    /// monotonic id, runs the registered handler's `before_publish`
    /// hook per subscriber (PR4e splits this per-receiver so the
    /// redaction runs once per delivery — for now we send the same
    /// event to all).
    ///
    /// `name` is the event name, `payload` is the JSON payload.
    pub async fn publish(
        &self,
        channel: &ChannelId,
        name: &str,
        payload: serde_json::Value,
    ) -> crate::common::Result<u64> {
        let mut chans = self.channels.lock().await;
        let state = chans
            .entry(channel.clone())
            .or_insert_with(|| ChannelState::new(self.capacity));
        let id = state.next_id;
        state.next_id += 1;
        let evt = ServerEvent {
            id,
            name: name.to_string(),
            channel: channel.clone(),
            payload,
        };
        // `send` errs only when there are zero receivers — that's
        // expected for channels that no one is watching, so ignore.
        let _ = state.tx.send(evt);
        Ok(id)
    }
}

impl Default for InProcessChannel {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoChannel;

    #[async_trait]
    impl RoomChannel for EchoChannel {
        fn kind(&self) -> &'static str {
            "echo"
        }
        async fn authorize(
            &self,
            _ctx: &ChannelContext,
            channel: &ChannelId,
            _params: serde_json::Value,
        ) -> crate::common::Result<serde_json::Value> {
            Ok(serde_json::json!({ "subscribed": channel.inner() }))
        }
    }

    #[test]
    fn channel_id_split() {
        let c = ChannelId("fof.round:abc".into());
        assert_eq!(c.kind(), "fof.round");
        assert_eq!(c.inner(), "abc");

        let c2 = ChannelId::from_parts("user", "alice");
        assert_eq!(c2.0, "user:alice");
    }

    #[tokio::test]
    async fn handler_resolution_by_kind() {
        let hub = InProcessChannel::new();
        hub.register(EchoChannel).await;

        let ok = hub
            .handler_for(&ChannelId("echo:foo".into()))
            .await
            .is_ok();
        assert!(ok);

        let err = hub
            .handler_for(&ChannelId("unknown:foo".into()))
            .await
            .is_err();
        assert!(err);
    }

    #[tokio::test]
    async fn publish_assigns_monotonic_ids_and_fans_out() {
        let hub = InProcessChannel::new();
        let ch = ChannelId("echo:room1".into());
        let mut rx = hub.subscribe_stream(&ch).await;

        let id_a = hub.publish(&ch, "tick", serde_json::json!({"n": 1})).await.unwrap();
        let id_b = hub.publish(&ch, "tick", serde_json::json!({"n": 2})).await.unwrap();
        assert_eq!(id_a, 1);
        assert_eq!(id_b, 2);

        let e1 = rx.recv().await.unwrap();
        let e2 = rx.recv().await.unwrap();
        assert_eq!(e1.id, 1);
        assert_eq!(e2.id, 2);
        assert_eq!(e1.name, "tick");
    }
}
