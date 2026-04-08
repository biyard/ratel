use dioxus::prelude::*;

use crate::components::layout::shell::Shell;
use crate::pages::{
    overview::Overview,
    foundations::{
        colors::Colors,
        typography::Typography,
        spacing::Spacing,
        radius::Radius,
        stroke::Stroke,
        shadows::Shadows,
    },
    components::{
        overview::ComponentsOverview,
        button::ButtonDocs,
        input::InputDocs,
    },
    playground::Playground,
};

/// Global theme signal. Components read this to toggle data-theme attribute.
pub static THEME: GlobalSignal<ThemeMode> = Signal::global(|| ThemeMode::Dark);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn attr(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark  => "dark",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark  => ThemeMode::Light,
        }
    }
}

/// Site route enum — one variant per page.
/// All routes live inside the Shell layout.
#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Shell)]
        // ── Overview ──────────────────────────────────────────────────────
        #[route("/")]
        Overview,

        // ── Foundations ───────────────────────────────────────────────────
        #[route("/foundations/colors")]
        Colors,

        #[route("/foundations/typography")]
        Typography,

        #[route("/foundations/spacing")]
        Spacing,

        #[route("/foundations/radius")]
        Radius,

        #[route("/foundations/stroke")]
        Stroke,

        #[route("/foundations/shadows")]
        Shadows,

        // ── Components ────────────────────────────────────────────────────
        #[route("/components")]
        ComponentsOverview,

        #[route("/components/button")]
        ButtonDocs,

        #[route("/components/input")]
        InputDocs,

        // ── Playground ────────────────────────────────────────────────────
        #[route("/playground")]
        Playground,

    #[end_layout]

    // 404 — rendered without Shell
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

/// Embed CSS directly to bypass LightningCSS variable stripping in manganis.
static CSS: &str = include_str!("../assets/tailwind.out.css");

/// Root application component — injects the stylesheet and sets up theme + router.
#[component]
pub fn App() -> Element {
    let theme = THEME.read();
    rsx! {
        document::Style { "{CSS}" }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/@phosphor-icons/web@2.1.1/src/regular/style.css",
        }
        div {
            id: "app",
            "data-theme": theme.attr(),
            Router::<Route> {}
        }
    }
}

/// Simple 404 page
#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen gap-4",
            h1 { class: "text-h1 font-bold text-ratel-text", "404" }
            p  { class: "text-body-2 text-ratel-text-body",
                "Page \"/{segments.join(\"/\")}\" not found."
            }
            Link {
                to: Route::Overview,
                class: "text-ratel-primary text-label-2 hover:underline",
                "← Back to Overview"
            }
        }
    }
}
