// Cross-posting server-function controllers.
//
// PR B (1A connection management):
//   GET    /api/cross-posting/connections
//   POST   /api/cross-posting/connections/bluesky
//   PATCH  /api/cross-posting/connections/{platform}
//   DELETE /api/cross-posting/connections/{platform}
//
// PR B2 (1A post-detail syndication panel):
//   GET    /api/cross-posting/posts/{post_id}/syndication
//   POST   /api/cross-posting/posts/{post_id}/jobs/{platform}/retry
//
// To be added in subsequent PRs:
//   GET  /api/cross-posting/oauth/{platform}/start            (1B/1C)
//   GET  /api/cross-posting/oauth/{platform}/callback         (1B/1C)
//   POST /api/cross-posting/onboarding/dismiss                (1D)

pub mod connect_bluesky;
pub mod connect_linkedin_init;
pub mod disconnect;
pub mod get_syndication_panel;
pub mod list_connections;
pub mod retry_job;
pub mod toggle_auto_post;

pub use connect_bluesky::*;
pub use connect_linkedin_init::*;
pub use disconnect::*;
pub use get_syndication_panel::*;
pub use list_connections::*;
pub use retry_job::*;
pub use toggle_auto_post::*;
