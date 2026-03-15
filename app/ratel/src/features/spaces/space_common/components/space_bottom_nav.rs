use crate::features::spaces::space_common::*;

use super::SpaceNavItem;

/// Mobile/tablet bottom navigation bar for Space pages.
/// Renders a fixed bottom bar with icon+label nav items, visible only below the tablet breakpoint.
/// This component mirrors the sidebar SpaceNav items in a horizontal bottom bar layout,
/// similar to YouTube's mobile bottom navigation.
#[component]
pub fn SpaceBottomNav(menus: Vec<SpaceNavItem>) -> Element {
    rsx! {
        nav { class: "flex tablet:hidden flex-row items-stretch justify-around border-t border-divider bg-space-bg h-14 shrink-0",
            for item in menus.iter() {
                BottomNavItem { item: item.clone() }
            }
        }
    }
}

#[component]
fn BottomNavItem(item: SpaceNavItem) -> Element {
    let current_path = use_context::<dioxus::router::RouterContext>().full_route_string();
    let is_active = match &item.link {
        NavigationTarget::Internal(route) => current_path.starts_with(&route.to_string()),
        _ => false,
    };

    rsx! {
        Link {
            class: "flex flex-col gap-0.5 items-center justify-center flex-1 py-1.5 text-[10px] font-medium transition-colors aria-selected:text-primary text-foreground-muted",
            "aria-selected": is_active,
            to: item.link,
            div { class: "flex items-center justify-center w-6 h-6",
                {item.icon}
            }
            span { class: "truncate max-w-full px-0.5", {item.label} }
        }
    }
}
