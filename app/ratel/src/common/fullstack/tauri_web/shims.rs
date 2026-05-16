//! Minimal compile-time shims for `dioxus-fullstack` types referenced in
//! handler signatures and a few hooks. Under `tauri-web` the fullstack
//! crate is not compiled — the original `#[get]`/`#[post]`/… handler
//! bodies are dropped by our `by_macros::server_fn` proc macro and only
//! the stub remains, so these types only need to exist as names; they do
//! not need real behaviour.

use std::future::Future;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use serde::de::DeserializeOwned;
use serde::Serialize;

/// Stand-in for `dioxus_fullstack::ServerFnError`. The dropped handler
/// bodies that mention this name on the server side never run on the
/// tauri-web client, so the only thing that matters is that the type
/// resolves and has the same surface (`::new(impl Display)`).
#[derive(Debug, Clone)]
pub struct ServerFnError(pub String);

impl ServerFnError {
    pub fn new(msg: impl std::fmt::Display) -> Self {
        Self(msg.to_string())
    }
}

impl std::fmt::Display for ServerFnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ServerFnError {}

/// Stand-in for `dioxus_fullstack::Form<T>`. Server-side it would wrap a
/// form-extracted body; on the tauri-web client we strip it from stubs
/// via the `unwrap_form_type` helper in `by_macros::server_fn`. The
/// `Deref` impls keep the dropped server-side handler bodies parseable.
pub struct Form<T>(pub T);

impl<T> Deref for Form<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> DerefMut for Form<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// `use_server_cached` shim — identical surface to `use_server_future`
/// in the bits the codebase exercises (cache semantics only matter
/// server-side; on the client we just re-run on dependency change).
pub fn use_server_cached<O, M>(future: impl Fn() -> O) -> O
where
    O: 'static + Clone,
    M: 'static,
{
    dioxus::prelude::use_hook(|| future())
}

/// `Lazy<T>` is a server-only helper for global config init; on
/// tauri-web it should never be touched, so the impl is a stub that
/// panics if anyone actually calls into it.
pub struct Lazy<T> {
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Lazy<T> {
    pub const fn new<F>(_init: F) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
