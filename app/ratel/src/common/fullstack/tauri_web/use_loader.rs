use std::future::Future;
use std::ops::Deref;

use crate::*;

use dioxus::{
    core::{Subscribers, SuspendedFuture},
    signals::ReadableExt,
    CapturedError,
};
use serde::de::DeserializeOwned;

pub type Loading = RenderError;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LoaderState {
    Pending,
    Ready,
    Failed,
}

pub struct Loader<T: 'static> {
    real_value: Signal<Option<T>>,
    resource: Resource<()>,
}

impl<T> Clone for Loader<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Loader<T> {}

impl<T> Loader<T> {
    /// Restart the loading task. Re-runs the future; won't suspend the
    /// component (background reload).
    pub fn restart(&mut self) {
        self.resource.restart();
    }

    /// `true` while the underlying resource is still in-flight. Matches the
    /// `Loader::loading()` API on dioxus's own loader so call sites that
    /// drive UI off it (`if loader.loading() { ... }`) keep working.
    pub fn loading(&self) -> bool {
        !self.resource.finished()
    }

    /// Cancel the current loading task. After cancellation, future reads
    /// will panic; callers should restart if they want a new attempt.
    pub fn cancel(&mut self) {
        self.resource.cancel();
    }
}

pub fn use_loader<F, T, E>(mut future: impl FnMut() -> F + 'static) -> Result<Loader<T>, Loading>
where
    F: Future<Output = std::result::Result<T, E>> + 'static,
    T: 'static + PartialEq + Serialize + DeserializeOwned,
    E: Into<CapturedError> + 'static,
{
    let mut error = use_signal(|| None as Option<CapturedError>);
    let mut value = use_signal(|| None as Option<T>);
    let mut loader_state = use_signal(|| LoaderState::Pending);

    debug!("use_loader - initializing loader with pending state");
    let resource = use_resource(move || {
        debug!("before calling future in use_resource");
        let user_fut = future();

        #[allow(clippy::let_and_return)]
        async move {
            let out = user_fut.await;
            debug!("after awaiting future in use_resource, got output:",);

            let out = out.map_err(|e| {
                let anyhow_err: CapturedError = e.into();
                anyhow_err
            });

            match out {
                Ok(v) => {
                    value.set(Some(v));
                    loader_state.set(LoaderState::Ready);
                }
                Err(e) => {
                    error.set(Some(e));
                    loader_state.set(LoaderState::Failed);
                }
            };
        }
    });

    debug!(
        "use_loader - after initializing resource and polling task, loader state is: {:?}",
        *loader_state
    );

    resource.suspend()?;

    Ok(Loader {
        real_value: value,
        resource,
    })
}

impl<T> Readable for Loader<T> {
    type Target = T;
    type Storage = UnsyncStorage;

    #[track_caller]
    fn try_read_unchecked(
        &self,
    ) -> std::result::Result<ReadableRef<'static, Self>, generational_box::BorrowError>
    where
        T: 'static,
    {
        let opt_ref = self.real_value.try_read_unchecked()?;
        Ok(opt_ref.map(|inner| {
            std::cell::Ref::map(inner, |opt: &Option<T>| {
                opt.as_ref()
                    .expect("Loader read called before future resolved")
            })
        }))
    }

    #[track_caller]
    fn try_peek_unchecked(&self) -> generational_box::BorrowResult<ReadableRef<'static, Self>>
    where
        T: 'static,
    {
        let opt_ref = self.real_value.try_peek_unchecked()?;
        Ok(opt_ref.map(|inner| {
            std::cell::Ref::map(inner, |opt: &Option<T>| {
                opt.as_ref()
                    .expect("Loader peek called before future resolved")
            })
        }))
    }

    fn subscribers(&self) -> Subscribers
    where
        T: 'static,
    {
        self.real_value.subscribers()
    }
}

impl<T: 'static> Writable for Loader<T> {
    type WriteMetadata = <Signal<Option<T>> as Writable>::WriteMetadata;

    fn try_write_unchecked(
        &self,
    ) -> std::result::Result<
        dioxus_signals::WritableRef<'static, Self>,
        generational_box::BorrowMutError,
    >
    where
        Self::Target: 'static,
    {
        let writer = self.real_value.try_write_unchecked()?;
        Ok(WriteLock::map(writer, |f: &mut Option<T>| {
            f.as_mut()
                .expect("Loader value should be set if the `Loader<T>` exists")
        }))
    }
}

// Make Loader callable as `loader()` — auto-deref turns `loader()` into
// `(*loader)()`, dispatching to the `dyn Fn() -> T` returned by deref_impl.
// Same pattern as dioxus_fullstack_core::Loader (loader.rs:277).
impl<T> Deref for Loader<T>
where
    T: Clone + PartialEq + 'static,
{
    type Target = dyn Fn() -> T;

    fn deref(&self) -> &Self::Target {
        unsafe { ReadableExt::deref_impl(self) }
    }
}
