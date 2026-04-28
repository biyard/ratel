// Cross-posting server-function controllers.
// Populated per the API surface in
// docs/superpowers/specs/2026-04-28-cross-posting-design.md:
//   GET    /api/cross-posting/connections
//   POST   /api/cross-posting/connections/bluesky
//   GET    /api/cross-posting/oauth/{platform}/start            (1B/1C)
//   GET    /api/cross-posting/oauth/{platform}/callback         (1B/1C)
//   PATCH  /api/cross-posting/connections/{platform}
//   DELETE /api/cross-posting/connections/{platform}
//   GET    /api/cross-posting/posts/{post_id}/syndication
//   POST   /api/cross-posting/posts/{post_id}/jobs/{platform}/retry
//   POST   /api/cross-posting/onboarding/dismiss
