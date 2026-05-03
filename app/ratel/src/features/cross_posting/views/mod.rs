// Page-level routed views for cross-posting (per `feature-module-structure.md`).
//
// - `connections_page` — Settings → Connections (`/{username}/settings/connections`)
// - `onboarding_page`  — Post-signup interstitial (`/onboarding/connections`)

pub mod connections_page;
pub mod onboarding_page;

pub use connections_page::ConnectionsPage;
pub use onboarding_page::OnboardingPage;
