//! Mobile-safe wrapper around `dioxus::use_loader`.
//!
//! ## Why this exists
//!
//! Dioxus 0.7 fullstack's `spawn_platform` builds tasks on a
//! `tokio_util::task::LocalPoolHandle` with `available_parallelism()` workers.
//! Each worker has its own current-thread Tokio runtime — and therefore its
//! own I/O reactor.
//!
//! On Android (and any non-wasm target where mobile is enabled), this fragments
//! the runtime in a way that breaks `reqwest`:
//!
//! - `GLOBAL_REQUEST_CLIENT` is lazily built when the first server function is
//!   called. Its hyper connection-driver task gets pinned to that worker's
//!   reactor (call it A).
//! - A later server function future polled on a different worker B can still
//!   write the request out (the connection is reused), and the server responds
//!   with `200 OK`. But the response read events fire on A's reactor — which
//!   never wakes B's task. `.send().await` hangs forever on the client even
//!   though server logs show the request completed cleanly.
//!
//! Curl from inside the same emulator works (proving the network is fine).
//! Running the same `reqwest` call inside a fresh OS thread with a fresh
//! `current_thread` Tokio runtime works (because the reactor that registers
//! the socket is the same runtime that polls the response future).
//!
//! This wrapper makes that "fresh thread + fresh runtime" the default for any
//! `use_loader` call when `feature = "mobile"` is enabled. On all other
//! targets it is a direct pass-through.
//!
//! ## API
//!
//! Same shape as `dioxus::use_loader`, but the future-producing closure must
//! be `Fn + Clone + Send + 'static` (not `FnMut`) so it can be cloned and
//! moved to the bridge thread on each invocation. The output value and error
//! must be `Send` for the same reason.

use std::future::Future;

use dioxus::fullstack::{Loader, Loading};
use dioxus::CapturedError;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(not(feature = "mobile"))]
pub use dioxus::prelude::use_loader;

/// Drop-in replacement for `dioxus::prelude::use_loader` that survives the
/// Dioxus-mobile runtime fragmentation bug.
///
/// On `feature = "mobile"`: each invocation of `future_fn` runs inside a
/// dedicated OS thread + freshly-built `current_thread` Tokio runtime. The
/// result is shipped back to the calling Dioxus task via a Tokio oneshot
/// channel, so this remains a normal `async`/`await`-driven loader from the
/// outside.
///
/// On every other target: pass-through to `dioxus::prelude::use_loader` with
/// zero overhead.
#[cfg(feature = "mobile")]
#[allow(clippy::result_large_err)]
#[track_caller]
pub fn use_loader<FutFn, F, T, E>(future_fn: FutFn) -> std::result::Result<Loader<T>, Loading>
where
    FutFn: Fn() -> F + Clone + Send + 'static,
    F: Future<Output = std::result::Result<T, E>> + 'static,
    T: 'static + PartialEq + Serialize + DeserializeOwned + Send,
    E: Into<CapturedError> + Send + 'static,
{
    dioxus::prelude::use_loader(move || {
        let f = future_fn.clone();
        async move {
            let (tx, rx) = tokio::sync::oneshot::channel();
            std::thread::spawn(move || {
                let rt = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(rt) => rt,
                    Err(_) => {
                        // Drop tx without sending; caller's rx.await will
                        // return Err and we map that to a panic so the
                        // loader surfaces a clear failure rather than
                        // hanging.
                        return;
                    }
                };
                let res = rt.block_on(async move { f().await });
                let _ = tx.send(res);
            });
            rx.await
                .expect("use_loader: dedicated bridge thread aborted before sending result")
        }
    })
}
