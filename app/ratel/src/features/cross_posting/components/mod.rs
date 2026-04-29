// Cross-posting UI components.
//
// PR E1 (1A) — Settings → Connections page + Bluesky connect modal.
// PR E2 (1A) — Compose-time right-rail sidebar.
// PR E3 (1A) — Post-detail author syndication panel.
//
// To be added in subsequent PRs:
//   - onboarding_interstitial/ Single-screen post-signup (1D)
//   - threads_no_ig_modal/     (1C)
//   - public_backlink_view/    Public landing page (1D polish)

pub mod bluesky_connect_modal;
pub mod compose_sidebar;
pub mod connections_page;
pub mod syndication_panel;

// Explicit re-exports — glob would clash on internal helpers like
// `PlatformLogo` that compose_sidebar and syndication_panel both define
// as private sub-components (the Dioxus #[component] macro publishes
// `<Name>Props` types alongside even private fns).
pub use bluesky_connect_modal::BlueskyConnectModal;
pub use compose_sidebar::CrossPostSidebar;
pub use connections_page::ConnectionsPage;
pub use syndication_panel::SyndicationPanel;
