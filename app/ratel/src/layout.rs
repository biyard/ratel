use crate::*;

#[component]
pub fn AppLayout() -> Element {
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
                // Uses safe-area-inset-bottom for iPhone home indicator.
                div { class: "max-tablet:pb-[calc(var(--mobile-bottom-nav-height)+env(safe-area-inset-bottom))]",
                    Outlet::<Route> {}
                }
            }
            // Mobile bottom navigation bar (visible only < tablet breakpoint)
            MobileBottomNav {}
        }
    }
}
