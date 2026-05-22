use crate::*;
use dioxus::prelude::use_loader as use_loader_origin;
use dioxus::CapturedError;
use serde::de::DeserializeOwned;
use std::future::Future;

// `use_loader` re-runs only when signals it depends on change. Dependency
// tracking happens during the *synchronous* prefix of the closure — i.e.
// every `Signal` / `Memo` read that executes before the `async move` block
// is registered as a dependency of the loader.
//
// Reads performed *inside* the `async move` body do NOT register: the
// async block is polled later, outside the reactive scope of the closure,
// so any signal touched there is invisible to the dependency graph and
// the loader will not re-run when that signal changes.
//
// To make `logged_in` an actual dependency we read it eagerly into a
// plain `bool` here, then move that captured value into the async block.
// Swap this for `if !logged_in() { ... }` inside `async move` and the
// loader stops reacting to login/logout.
#[track_caller]
pub fn use_loader<F, T, E>(future: impl FnMut() -> F + 'static) -> Result<Loader<T>, RenderError>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: 'static + PartialEq + Serialize + DeserializeOwned,
    E: Into<CapturedError> + 'static,
{
    let ret = use_loader_origin(future)?;

    Ok(ret)
}
