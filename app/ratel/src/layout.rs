use crate::*;

#[component]
pub fn AppLayout() -> Element {
    TeamContext::init();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = use_team_context();

    // Load teams when user is logged in
    let _teams_loader = use_resource(move || async move {
        let user = user_ctx().user.clone();
        if user.is_some() {
            match get_user_teams_handler().await {
                Ok(teams) => {
                    team_ctx.set_teams(teams);
                }
                Err(e) => {
                    debug!("Failed to load teams: {:?}", e);
                }
            }
        }
    });

    rsx! {
        SidebarProvider { default_open: false, class: "flex flex-row w-full min-h-screen",
            Sidebar {
                collapsible: SidebarCollapsible::Icon,
                variant: SidebarVariant::Sidebar,
                class: "border-r border-divider bg-bg",
                AppMenu {}
            }
            SidebarInset { class: "overflow-x-hidden relative flex-1 min-w-0",
                // Right gradient blur edge
                div { class: "fixed top-0 right-0 z-40 w-10 h-full bg-linear-to-l to-transparent pointer-events-none from-bg backdrop-blur-sm mask-[linear-gradient(to_left,black,transparent)]" }
                // Add bottom padding on mobile so content is not hidden behind the bottom nav.
                // Uses safe-area-inset-bottom for iPhone home indicator. Reset to 0 at md+.
                div { class: "pb-[calc(3.5rem+env(safe-area-inset-bottom))] md:pb-0",
                    Outlet::<Route> {}
                }
            }
            // Mobile bottom navigation bar (visible only < 768px)
            MobileBottomNav {}
        }
    }
}
