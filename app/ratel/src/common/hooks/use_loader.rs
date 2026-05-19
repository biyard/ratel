use crate::*;
use dioxus::prelude::use_loader as use_loader_origin;
use dioxus::CapturedError;
use serde::de::DeserializeOwned;
use std::future::Future;

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
