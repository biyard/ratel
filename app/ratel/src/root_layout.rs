use crate::*;

#[component]
pub fn RootLayout() -> Element {
    debug!("Initializing RootLayout contexts...");
    crate::features::auth::Context::init()?;
    debug!("Auth context initialized.");
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
