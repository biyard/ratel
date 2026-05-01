// Cross-posting reusable UI components — embedded inside other pages.
//
// PR E2 (1A) — Compose-time right-rail sidebar (`post_edit`).
// PR E3 (1A) — Post-detail author syndication panel (`post_detail`).
// PR E1 (1A) — Bluesky connect modal (used by both ConnectionsPage and
//              OnboardingPage views).
//
// Page-level routed components live under `features::cross_posting::views`
// (per `feature-module-structure.md` "Page-level views → views/<page>/").

pub mod bluesky_connect_modal;
pub mod compose_sidebar;
pub mod syndication_panel;

// Explicit re-exports — glob would clash on internal helpers like
// `PlatformLogo` that compose_sidebar and syndication_panel both define
// as private sub-components (the Dioxus #[component] macro publishes
// `<Name>Props` types alongside even private fns).
pub use bluesky_connect_modal::BlueskyConnectModal;
pub use compose_sidebar::CrossPostSidebar;
pub use syndication_panel::SyndicationPanel;
