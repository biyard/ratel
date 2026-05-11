//! Project-wide entry point for `use_loader`.
//!
//! ## Background — why this re-export exists at all
//!
//! Dioxus 0.7 fullstack's `spawn_platform` originally built tasks on a
//! `tokio_util::task::LocalPoolHandle` with `available_parallelism()` workers.
//! Each worker had its own current-thread Tokio runtime — and therefore its
//! own I/O reactor.
//!
//! On Android (and any non-wasm target where mobile is enabled), this
//! fragmented the runtime in a way that broke `reqwest`:
//!
//! - `GLOBAL_REQUEST_CLIENT` is lazily built when the first server function is
//!   called. Its hyper connection-driver task gets pinned to that worker's
//!   reactor (call it A).
//! - A later server function future polled on a different worker B could still
//!   write the request out (the connection was reused), and the server would
//!   respond with `200 OK`. But the response read events fired on A's reactor —
//!   which never woke B's task. `.send().await` hung forever on the client
//!   even though server logs showed the request completed cleanly.
//!
//! ## Fix
//!
//! The root cause is fixed in our Dioxus fork at
//! `packages/fullstack/src/spawn.rs` by pinning `LocalPoolHandle::new(1)` so
//! every `!Send` Dioxus task shares a single Tokio runtime with a single I/O
//! reactor. Once that patch is in place, `reqwest` calls are driven by the
//! same runtime that registered their sockets, and responses are delivered
//! normally.
//!
//! Because the fix is at the framework layer, this `use_loader` is a pure
//! pass-through to `dioxus::prelude::use_loader` on every target — no extra
//! `Send` bounds, no thread bridging, no API breakage at call sites. We keep
//! it routed through this module so:
//!
//! 1. Every `use crate::common::*` callsite goes through one canonical entry
//!    point. If we ever need to add an additional shim (e.g. for a future
//!    Dioxus regression or for desktop-specific behavior), it lives here.
//! 2. The `common::*` namespace's `use_loader` always wins over
//!    `dioxus::prelude::*`'s glob — preventing accidental drift if anyone
//!    introduces a competing version.

pub use dioxus::prelude::use_loader;
