//! Global `InProcessChannel` singleton.
//!
//! In Lambda each invocation owns its own process, so this is a
//! *per-invocation* singleton — not a cluster-wide one. The
//! cross-invocation fan-out for chat is handled by the DDB Stream
//! listener (PR4f): when a chat row is written, every active SSE
//! invocation receives the same Stream record and runs its own
//! `hub.publish(...)` locally for the subscribers it happens to
//! hold.
//!
//! For local dev (`cargo run`) and tests there is exactly one process,
//! so this singleton behaves like the in-memory hub it is and chat
//! fan-out works directly through `hub.publish`.

use crate::features::arcade::realtime::channel::InProcessChannel;
use std::sync::LazyLock;

/// One hub per process. Cloning is cheap (Arc-wrapped state inside
/// `InProcessChannel`), so callers should clone freely rather than
/// holding a long-lived borrow on the static.
static HUB: LazyLock<InProcessChannel> = LazyLock::new(InProcessChannel::new);

/// Clone of the process-wide hub. Use this everywhere — from HTTP
/// handlers that publish, from SSE handlers that subscribe, from the
/// stream listener that bridges DDB events.
pub fn global_hub() -> InProcessChannel {
    HUB.clone()
}
