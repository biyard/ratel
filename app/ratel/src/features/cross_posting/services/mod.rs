// Server-only services for the cross-posting pipeline.
// To be populated:
//   - adapters/        per-platform CrossPostAdapter impls (1A: bluesky, 1B: linkedin, 1C: threads)
//   - format.rs        format_for_platform / truncate_override (FR-5.5)
//   - dispatcher.rs    Stage 2 lock-pattern dispatcher
//   - shard.rs         shard_for(post_id) — single shared deterministic hash utility
//   - retry_sweeper.rs Stage 3 (1D)
//   - engagement.rs    Stage 4 adaptive sweeper (1D)
