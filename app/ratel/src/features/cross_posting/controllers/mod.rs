// Cross-posting server-function controllers.
//
// PR B (1A scope) — connection management:
//   GET    /api/cross-posting/connections
//   POST   /api/cross-posting/connections/bluesky
//   PATCH  /api/cross-posting/connections/{platform}
//   DELETE /api/cross-posting/connections/{platform}
//
// To be added in subsequent PRs:
//   GET  /api/cross-posting/oauth/{platform}/start            (1B/1C)
//   GET  /api/cross-posting/oauth/{platform}/callback         (1B/1C)
//   GET  /api/cross-posting/posts/{post_id}/syndication       (PR B2)
//   POST /api/cross-posting/posts/{post_id}/jobs/{platform}/retry  (PR B2)
//   POST /api/cross-posting/onboarding/dismiss                (1D)

pub mod connect_bluesky;
pub mod disconnect;
pub mod list_connections;
pub mod toggle_auto_post;

pub use connect_bluesky::*;
pub use disconnect::*;
pub use list_connections::*;
pub use toggle_auto_post::*;
