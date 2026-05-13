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
//! be `Fn + Copy + 'static`. It is wrapped in an `Arc<Mutex<…>>` inside the
//! hook so the same `Fn` closure can be shared with the bridge thread without
//! requiring the closure type itself to be `Send`. This is what allows the
//! wrapper to accept closures that capture Dioxus signals (which use
//! `UnsyncStorage` and are therefore `!Send`).

use std::future::Future;
use std::sync::{Arc, Mutex};

use dioxus::fullstack::{Loader, Loading};
use dioxus::CapturedError;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Force a `!Send` value to be `Send`.
///
/// Used to ferry a `Copy` closure across the bridge-thread boundary. The
/// closure itself is `Copy` (so we hand out an owned copy on the bridge
/// thread, never a reference), and Dioxus mobile is effectively
/// single-OS-process — so the cross-thread move is a structural one, not a
/// concurrency one.
struct AssertSend<T>(T);
// SAFETY: We only access the inner closure as an owned `Copy` value on the
// bridge thread. Any `!Send` state the closure captures (e.g. Dioxus signals)
// is reached through reactive reads inside the captured future, which is
// constructed and consumed on the bridge thread before any other thread sees
// it. There is no shared mutable cross-thread access.
unsafe impl<T> Send for AssertSend<T> {}

/// Drop-in replacement for `dioxus::prelude::use_loader` that survives the
/// Dioxus-mobile runtime fragmentation bug.
///
/// Each invocation of `future_fn` runs inside a dedicated OS thread plus a
/// freshly-built `current_thread` Tokio runtime. The result is shipped back
/// to the calling Dioxus task via a `std::sync::mpsc` channel.
#[allow(clippy::result_large_err)]
#[track_caller]
pub fn use_loader<FutFn, F, T, E>(future_fn: FutFn) -> std::result::Result<Loader<T>, Loading>
where
    FutFn: Fn() -> F + Copy + 'static,
    F: Future<Output = std::result::Result<T, E>> + 'static,
    T: 'static + PartialEq + Serialize + DeserializeOwned + Send,
    E: Into<CapturedError> + Send + 'static,
{
    // Wrap the closure in `Arc<Mutex<AssertSend<…>>>` so we can:
    //   1. Hand a clone of the `Arc` to each bridge thread spawn (cheap; no
    //      cloning of the closure body itself).
    //   2. Treat the inner value as `Send` even when the closure isn't
    //      auto-`Send` (e.g. when it captures Dioxus signals).
    //   3. Copy out an owned `FutFn` on the bridge thread via the `Copy`
    //      bound, without keeping the mutex locked across the `.await`.
    let shared = Arc::new(Mutex::new(AssertSend(future_fn)));

    dioxus::prelude::use_loader(move || {
        let shared = shared.clone();
        async move {
            use std::sync::mpsc;

            let (tx, rx) = mpsc::channel::<std::result::Result<T, E>>();

            std::thread::spawn(move || {
                let result = (move || -> std::result::Result<T, E> {
                    let rt = match tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                    {
                        Ok(rt) => rt,
                        Err(e) => {
                            unimplemented!(
                                "Failed to build Tokio runtime in use_loader bridge thread: {e:?}"
                            )
                        }
                    };
                    // Lock only long enough to copy the closure out, then
                    // drop the guard so it never crosses the `.await` below.
                    let f: FutFn = {
                        let guard = shared.lock().expect("future_fn mutex poisoned");
                        guard.0
                    };
                    rt.block_on(async move { f().await })
                })();
                let _ = tx.send(result);
            });

            match rx.recv() {
                Ok(v) => {
                    crate::debug!("Received value from use_loader bridge thread:");
                    v
                }
                Err(e) => {
                    unimplemented!("Failed to receive result from use_loader bridge thread: {e:?}")
                }
            }
        }
    })
}
