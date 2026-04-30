use crate::common::components::{Button, ButtonShape, ButtonStyle};
use crate::*;

#[component]
pub fn RootLayout() -> Element {
    crate::features::auth::Context::init()?;
    crate::common::contexts::TeamContext::init()?;

    rsx! {
        ErrorBoundary {
            handle_error: move |error: ErrorContext| {
                error!("Error in component tree: {:?}", error);
                rsx! {
                    ErrorPage { ctx: error }
                }
            },
            // `NotificationsBootstrap` is the stable ancestor that owns
            // the notification signals. It renders `Outlet::<Route>`
            // directly (no `children: Element` prop) and is mounted
            // unconditionally — regardless of login state — so the
            // component tree shape doesn't churn between the two branches
            // of an `if logged_in { ... } else { ... }`. That churn, when
            // combined with route navigation, triggered Dioxus 0.7's
            // "cannot reclaim ElementId" arena errors on space enter.
            // The hooks inside Bootstrap read login state reactively and
            // no-op on the network side when the viewer is logged out,
            // so this is safe for anonymous traffic too.
            SuspenseBoundary { NotificationsBootstrap {} }
        }
        PopupZone {}
        ToastProvider {}
    }
}

/// Mounts the route outlet under a stable ancestor scope that owns the
/// notification-hook signals. Because every route (and therefore every
/// `NotificationBell` / `NotificationPanel` mount) renders as a
/// descendant of this component, consumer scopes always sit on the
/// ancestor chain of the owning scope — satisfying the Dioxus
/// signal-ownership invariant.
#[component]
fn NotificationsBootstrap() -> Element {
    // `use_unread_count` is safe to call here — internally wrapped in
    // `use_hook` (stable 1 hook slot per render). It also reads login
    // state so the polling loop is a no-op when the viewer is signed out.
    let _ = crate::features::notifications::hooks::use_unread_count();
    // `use_provide_inbox` is the installer variant that always runs the
    // full hook sequence (signals + loader + actions). Calling the
    // consumer-only `use_inbox()` here would only do a context read and
    // never create the underlying signals. The loader closure checks
    // login state so logged-out users don't hit /api/inbox. See the
    // installer/consumer split in `use_inbox.rs`.
    let _ = crate::features::notifications::hooks::use_provide_inbox()?;
    rsx! {
        Outlet::<Route> {}
    }
}

#[component]
fn ErrorPage(ctx: ErrorContext) -> Element {
    let tr: ErrorPageTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();
    let mut user_ctx = crate::features::auth::hooks::use_user_context();

    let (is_auth_error, message) = match ctx.error() {
        Some(err) => match err.downcast_ref::<Error>() {
            Some(typed) => {
                let is_auth = matches!(
                    typed,
                    Error::NoSessionFound
                        | Error::UnauthorizedAccess
                        | Error::UserNotFoundInContext
                );
                (is_auth, typed.translate(&lang()).to_string())
            }
            None => (false, format!("{err:?}")),
        },
        None => (false, tr.description.to_string()),
    };

    rsx! {
        div { class: "flex flex-col gap-6 justify-center items-center py-16 px-6 w-full min-h-screen bg-background",
            div { class: "flex flex-col gap-3 items-center max-w-md text-center",
                h1 { class: "text-2xl font-bold text-text-primary", "{tr.title}" }
                p { class: "text-sm text-foreground-muted", "{message}" }
            }
            div { class: "flex flex-row gap-3",
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Rounded,
                    onclick: {
                        let ctx = ctx;
                        move |_| {
                            // If the error came from a missing/expired
                            // session, the cached user_ctx is now lying
                            // (server-side it's gone). Reset it and wipe
                            // the localStorage cache before navigating
                            // home, otherwise the Index page's auth-gated
                            // loaders refire the same 401 immediately and
                            // we bounce right back to this error page.
                            if is_auth_error {
                                user_ctx.set(crate::features::auth::context::UserContext::default());
                                crate::common::services::persistent_state::clear_cached_session();
                            }
                            ctx.clear_errors();
                            nav.push(Route::Index {});
                        }
                    },
                    "{tr.go_home}"
                }
            }
        }
    }
}

translate! {
    ErrorPageTranslate;

    title: {
        en: "Something went wrong",
        ko: "문제가 발생했습니다",
    },

    description: {
        en: "We couldn't load this page.",
        ko: "페이지를 불러올 수 없습니다.",
    },

    go_home: {
        en: "Go home",
        ko: "홈으로 이동",
    },
}
