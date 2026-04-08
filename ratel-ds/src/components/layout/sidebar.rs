use dioxus::prelude::*;
use crate::router::Route;

// ─── Nav data types ────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct NavSection {
    label: &'static str,
    items: &'static [NavItem],
}

#[derive(Clone, PartialEq)]
struct NavItem {
    label:  &'static str,
    icon:   &'static str,   // SVG path data (24×24 viewBox)
    route:  NavRoute,
}

/// Mirrors Route variants but as a Copy type so we can use it in const statics.
#[derive(Clone, Copy, PartialEq)]
enum NavRoute {
    Overview,
    Colors,
    Typography,
    Spacing,
    Radius,
    Stroke,
    Shadows,
    ComponentsOverview,
    ButtonDocs,
    InputDocs,
    Playground,
}

impl NavRoute {
    fn to_route(self) -> Route {
        match self {
            NavRoute::Overview           => Route::Overview,
            NavRoute::Colors             => Route::Colors,
            NavRoute::Typography         => Route::Typography,
            NavRoute::Spacing            => Route::Spacing,
            NavRoute::Radius             => Route::Radius,
            NavRoute::Stroke             => Route::Stroke,
            NavRoute::Shadows            => Route::Shadows,
            NavRoute::ComponentsOverview => Route::ComponentsOverview,
            NavRoute::ButtonDocs         => Route::ButtonDocs,
            NavRoute::InputDocs          => Route::InputDocs,
            NavRoute::Playground         => Route::Playground,
        }
    }
}

// ─── Static navigation tree ────────────────────────────────────────────────────

static NAV: &[NavSection] = &[
    NavSection {
        label: "",
        items: &[
            NavItem { label: "Overview", icon: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6", route: NavRoute::Overview },
        ],
    },
    NavSection {
        label: "Foundations",
        items: &[
            NavItem { label: "Colors",     icon: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01", route: NavRoute::Colors },
            NavItem { label: "Typography", icon: "M4 6h16M4 12h16m-7 6h7",    route: NavRoute::Typography },
            NavItem { label: "Spacing",    icon: "M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z", route: NavRoute::Spacing },
            NavItem { label: "Radius",     icon: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15", route: NavRoute::Radius },
            NavItem { label: "Stroke",     icon: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z", route: NavRoute::Stroke },
            NavItem { label: "Shadows",    icon: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4", route: NavRoute::Shadows },
        ],
    },
    NavSection {
        label: "Components",
        items: &[
            NavItem { label: "All Components", icon: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z", route: NavRoute::ComponentsOverview },
            NavItem { label: "Button", icon: "M20 7H4a2 2 0 00-2 2v6a2 2 0 002 2h16a2 2 0 002-2V9a2 2 0 00-2-2z",                                        route: NavRoute::ButtonDocs },
            NavItem { label: "Input",  icon: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z", route: NavRoute::InputDocs },
        ],
    },
    NavSection {
        label: "Internal",
        items: &[
            NavItem { label: "Playground", icon: "M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z M21 12a9 9 0 11-18 0 9 9 0 0118 0z", route: NavRoute::Playground },
        ],
    },
];

// ─── Sidebar component ─────────────────────────────────────────────────────────

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside { class: "ds-sidebar",
            // ── Brand header ──────────────────────────────────────────────
            div {
                class: "flex items-center gap-3 px-4 py-5 shrink-0",
                style: "border-bottom: 1px solid #1F1F1F;",

                // Logo mark
                div {
                    class: "flex items-center justify-center w-8 h-8 rounded-ratel-md shrink-0",
                    style: "background-color: var(--ratel-color-generic-primary);",
                    span {
                        class: "text-label-3 font-black",
                        style: "color: #0A0A0A;",
                        "R"
                    }
                }

                div {
                    span {
                        class: "text-label-1 font-bold block",
                        style: "color: #F5F5F5;",
                        "Ratel DS"
                    }
                    span {
                        class: "text-label-4",
                        style: "color: #525252;",
                        "Design System"
                    }
                }
            }

            // ── Navigation ────────────────────────────────────────────────
            nav {
                class: "flex-1 py-2 overflow-y-auto",
                for section in NAV {
                    SidebarSection { section }
                }
            }

            // ── Footer ────────────────────────────────────────────────────
            div {
                class: "px-4 py-4 shrink-0 text-label-4",
                style: "border-top: 1px solid #1F1F1F; color: #404040;",
                "v0.1.0 · Phase 2"
            }
        }
    }
}

// ─── Section block (header + items) ───────────────────────────────────────────

#[component]
fn SidebarSection(section: &'static NavSection) -> Element {
    rsx! {
        div {
            if !section.label.is_empty() {
                div { class: "ds-nav-section-header", { section.label } }
            }
            for item in section.items {
                NavLink { item }
            }
        }
    }
}

// ─── Individual nav link ───────────────────────────────────────────────────────

#[component]
fn NavLink(item: &'static NavItem) -> Element {
    rsx! {
        Link {
            to: item.route.to_route(),
            active_class: "active",
            class: "ds-nav-link",

            // Icon
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "w-4 h-4 shrink-0",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "1.5",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: item.icon,
                }
            }

            span { { item.label } }
        }
    }
}
