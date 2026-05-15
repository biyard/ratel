//! 라텔 오락실 — 미니게임 플랫폼.
//!
//! arcade-level 추상(wallet / realtime / services)과 그 위에 얹히는 게임
//! 모듈들 (`games::<name>`) 의 owner. v1은 Fact or Fold 단 1개. wallet /
//! realtime / services / pages / components / hooks 같은 arcade-level
//! 모듈은 PR4b 이후에 추가된다.
//!
//! Design: [docs/superpower/2026-05-15-arcade-platform.md](../../../../docs/superpower/2026-05-15-arcade-platform.md)

pub mod games;

pub use games::*;
