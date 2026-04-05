use crate::*;
use dioxus::fullstack::{Loading, Transportable};
use serde::de::DeserializeOwned;

use crate::common::{
    traits::{Bookmarker, ItemIter},
    *,
};

pub struct InfiniteQuery<Bookmark, I, T>
where
    Bookmark: 'static,
    I: 'static,
    T: 'static + Clone,
{
    bookmark: Signal<Option<Bookmark>>,
    next_bookmark: Signal<Option<Bookmark>>,
    accumulated: Signal<Vec<I>>,
    has_more: Memo<bool>,
    rsc: Loader<T>,
    effect: Effect,
    loading: Signal<bool>,
    key: u64,
}

// Manual Clone/Copy impls: all fields (Signal, Memo, Resource, Effect, u64) are
// Clone+Copy regardless of type parameters. Derive would add overly restrictive bounds.
impl<Bookmark, I, T> Clone for InfiniteQuery<Bookmark, I, T>
where
    Bookmark: 'static,
    I: 'static,
    T: 'static + Clone,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Bookmark, I, T> Copy for InfiniteQuery<Bookmark, I, T>
where
    Bookmark: 'static,
    I: 'static,
    T: 'static + Clone,
{
}

impl<Bookmark, I, T> InfiniteQuery<Bookmark, I, T>
where
    Bookmark: 'static + Clone + PartialEq + std::fmt::Debug,
    I: 'static + Clone,
    T: Clone,
{
    pub fn next(&mut self) {
        let nb = self.next_bookmark.read().clone();
        debug!("Next called on InfiniteQuery with bookmark: {:?}", nb);

        if self.is_loading() || nb.is_none() || self.bookmark.read().clone() == nb {
            return;
        }

        self.loading.set(true);
        self.bookmark.set(nb);
    }

    pub fn items(&self) -> Vec<I> {
        self.accumulated.read().clone()
    }

    pub fn insert(&mut self, items: I) {
        let mut new_items = vec![items];
        new_items.extend(self.accumulated.read().clone());
        self.accumulated.set(new_items);
    }

    pub fn restart(&mut self) {
        self.bookmark.set(None);
        self.next_bookmark.set(None);
        self.accumulated.set(Vec::new());
        self.rsc.restart();
    }

    pub fn has_more(&self) -> bool {
        *self.has_more.read()
    }

    pub fn is_loading(&self) -> bool {
        *self.loading.read()
    }

    pub fn more_element(&mut self) -> Element {
        if self.has_more() {
            if self.is_loading() {
                // FIXME: refactoring loading indicator
                return rsx! {
                    div { class: "", "Loading more..." }
                };
            } else {
                let ctrl = self.clone();
                let sentinel_id = format!("infinite-scroll-sentinel-{}", ctrl.key);

                rsx! {
                    div {
                        id: "{sentinel_id}",
                        class: "h-px",
                        onmounted: move |_| {
                            #[cfg(feature = "web")]
                            {
                                use std::cell::RefCell;
                                use std::rc::Rc;
                                use wasm_bindgen::prelude::*;

                                let mut ctrl = ctrl.clone();
                                let window = web_sys::window().unwrap();
                                let document = window.document().unwrap();

                                if let Some(el) = document.get_element_by_id(&sentinel_id) {

                                    let observer_rc: Rc<RefCell<Option<web_sys::IntersectionObserver>>> = Rc::new(
                                        RefCell::new(None),
                                    );
                                    let observer_ref = observer_rc.clone();
                                    let callback = Closure::<
                                        dyn FnMut(js_sys::Array),
                                    >::new(move |entries: js_sys::Array| {
                                        let entry: web_sys::IntersectionObserverEntry = entries
                                            .get(0)
                                            .unchecked_into();
                                        if entry.is_intersecting() {
                                            if let Some(obs) = observer_ref.borrow().as_ref() {
                                                obs.disconnect();
                                            }
                                            ctrl.next();
                                        }
                                    });
                                    let options = web_sys::IntersectionObserverInit::new();
                                    options.set_threshold(&wasm_bindgen::JsValue::from_f64(0.1));
                                    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                                        callback.as_ref().unchecked_ref(),
                                        &options,
                                    ) {
                                        observer.observe(&el);
                                        *observer_rc.borrow_mut() = Some(observer);
                                    }
                                    callback.forget();
                                }
                            }
                        },
                    }
                }
            }
        } else {
            rsx! {}
        }
    }
}

/// Usage:
/// ``rust,no_run
/// #[component]
/// fn Follows() -> Element {
///     let mut followers_query = use_infinite_query(move |bookmark| list_followers(bookmark))?;
///     let followers_loading = followers_query.is_loading();
///     let followers = followers_query.items();
///     let followers_more = followers_query.more_element();
///
///     rsx! {
///         FollowList {
///             users: followers,
///             selected: FollowTab::Followers,
///             loading: followers_loading,
///             on_follow,
///             on_unfollow,
///             more_element: followers_more,
///         }
///     }
/// }
/// ```
pub fn use_infinite_query<Bookmark, I, T, F>(
    mut future: impl FnMut(Option<Bookmark>) -> F + 'static + Clone + Copy,
) -> dioxus::prelude::Result<InfiniteQuery<Bookmark, I, T>, RenderError>
where
    Bookmark: 'static + Clone + PartialEq + std::fmt::Debug,
    I: 'static + Clone + PartialEq,
    F: std::future::Future<Output = Result<T>> + 'static,
    T: 'static
        + Clone
        + PartialEq
        + Serialize
        + DeserializeOwned
        + Bookmarker<Bookmark>
        + ItemIter<I>
        + Default,
{
    let bookmark: Signal<Option<Bookmark>> = use_signal(move || None);

    let rsc = use_loader(move || async move { future(None).await })?;
    let mut next_bookmark: Signal<Option<Bookmark>> = use_signal(move || rsc().bookmark());
    let mut accumulated: Signal<Vec<I>> = use_signal(move || rsc().items().clone());
    let has_more = use_memo(move || next_bookmark().is_some());
    let mut loading = use_signal(|| false);
    let key = use_server_cached(|| {
        use rand::RngExt;
        rand::rng().random::<u64>()
    });

    let effect = use_effect(move || {
        let nb = bookmark();

        if nb.is_none() {
            return;
        }

        spawn(async move {
            loading.set(true);
            let res = match future(nb.clone()).await {
                Ok(ret) => {
                    let next = ret.bookmark();
                    let items = ret.items().clone();
                    let mut appended = false;

                    accumulated.with_mut(|current| {
                        for item in items {
                            if current.iter().any(|existing| existing == &item) {
                                continue;
                            }

                            current.push(item);
                            appended = true;
                        }
                    });

                    if !appended {
                        next_bookmark.set(None);
                    } else {
                        next_bookmark.set(next);
                    }
                }
                Err(e) => {
                    debug!(
                        "Effect fetch failed for bookmark: {:?} with error: {:?}",
                        nb, e
                    );
                }
            };
            loading.set(false);
            res
        });
    });

    Ok(InfiniteQuery {
        bookmark,
        next_bookmark,
        accumulated,
        has_more,
        rsc,
        effect,
        loading,
        key,
    })
}
