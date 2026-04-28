use crate::*;

#[component]
pub fn AppLayout() -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = use_team_context();
    let mut notifications_open = use_signal(|| false);

    // Load teams when user is logged in
    let _teams_loader = use_resource(move || async move {
        let user = user_ctx().user.clone();
        if user.is_some() {
            match crate::features::social::controllers::get_user_teams_handler(None).await {
                Ok(resp) => {
                    team_ctx.set_teams(resp.items);
                }
                Err(e) => {
                    debug!("Failed to load teams: {:?}", e);
                }
            }
        }
    });

    let logged_in = user_ctx().is_logged_in();

    rsx! {
        SidebarProvider { default_open: false, class: "flex flex-row w-full min-h-screen",
            Sidebar {
                collapsible: SidebarCollapsible::Icon,
                variant: SidebarVariant::Sidebar,
                class: "border-r border-divider bg-bg",
                AppMenu {}
            }
            SidebarInset {
                class: "overflow-x-hidden relative flex-1 min-w-0",
                "data-testid": "app-layout",
                // Right gradient blur edge
                div { class: "fixed top-0 right-0 z-40 w-10 h-full to-transparent pointer-events-none bg-linear-to-l from-bg backdrop-blur-sm mask-[linear-gradient(to_left,black,transparent)]" }
                if logged_in {
                    div { class: "fixed top-3 right-3 z-50",
                        crate::features::notifications::components::NotificationBell { onclick: move |_| notifications_open.toggle() }
                    }
                }
                // Add bottom padding on mobile so content is not hidden behind the bottom nav.
                // Uses safe-area-inset-bottom for iPhone home indicator.
                div { class: "max-tablet:pb-[calc(var(--mobile-bottom-nav-height)+env(safe-area-inset-bottom))]",
                    Outlet::<Route> {}
                }
            }
            // Mobile bottom navigation bar (visible only < tablet breakpoint)
            MobileBottomNav {}
        }
        if logged_in {
            SuspenseBoundary {
                crate::features::notifications::components::NotificationPanel {
                    open: notifications_open(),
                    on_close: move |_| notifications_open.set(false),
                }
            }
        }
    }
}
