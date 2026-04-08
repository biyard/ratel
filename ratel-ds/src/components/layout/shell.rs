use dioxus::prelude::*;
use crate::router::{Route, THEME, ThemeMode};
use crate::components::layout::sidebar::Sidebar;

/// Shell — the outer persistent layout wrapping sidebar + content area.
/// Dioxus Router renders the matched page into `Outlet`.
#[component]
pub fn Shell() -> Element {
    rsx! {
        div {
            class: "flex min-h-screen",
            style: "background-color: var(--ratel-color-border-background-neutral-950);",

            // ── Left sidebar (always visible) ─────────────────────────────
            Sidebar {}

            // ── Main area (offset by sidebar width) ───────────────────────
            div {
                class: "ds-content-area flex flex-col flex-1",

                // ── Top bar ───────────────────────────────────────────────
                header { class: "ds-page-header",
                    // Breadcrumb / page context (placeholder — updated per-page via signals if needed)
                    div {
                        class: "flex items-center gap-2 text-label-3",
                        style: "color: var(--ratel-color-font-neutral-absolute);",
                        span { "Ratel Design System" }
                    }

                    // Controls: search (stub) + theme toggle
                    div { class: "flex items-center gap-3",

                        // Search stub
                        div {
                            class: "hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-ratel-md text-label-3",
                            style: "background: var(--ratel-color-border-incard-background-default); color: var(--ratel-color-font-neutral-absolute); border: 1px solid var(--ratel-color-border-stroke-neutral-800); cursor: not-allowed; opacity: 0.6;",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "w-3.5 h-3.5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                                }
                            }
                            span { "Search tokens…" }
                            kbd {
                                class: "ml-4 text-label-4 px-1.5 py-0.5 rounded",
                                style: "background: var(--ratel-color-border-background-neutral-800); color: var(--ratel-color-font-neutral-absolute);",
                                "⌘K"
                            }
                        }

                        // Theme toggle
                        ThemeToggle {}
                    }
                }

                // ── Page content (routed) ─────────────────────────────────
                main {
                    class: "flex-1",
                    style: "background-color: var(--ratel-color-border-background-neutral-950);",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

// ─── Theme toggle button ───────────────────────────────────────────────────────

#[component]
fn ThemeToggle() -> Element {
    let is_dark = *THEME.read() == ThemeMode::Dark;

    rsx! {
        button {
            class: "flex items-center justify-center w-8 h-8 rounded-ratel-md transition-colors duration-150",
            style: "background: var(--ratel-color-border-incard-background-default); border: 1px solid var(--ratel-color-border-stroke-neutral-800); color: var(--ratel-color-font-neutral-absolute);",
            title: if is_dark { "Switch to light mode" } else { "Switch to dark mode" },
            onclick: move |_| {
                let current = *THEME.read();
                *THEME.write() = current.toggle();
            },

            if is_dark {
                // Sun icon
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "w-4 h-4",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z",
                    }
                }
            } else {
                // Moon icon
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "w-4 h-4",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z",
                    }
                }
            }
        }
    }
}
