//! arcade에 등록된 미니게임 모듈들. 각 게임은 자기 trait
//! 구현(`StageScheduler`, `RoomChannel` handler 등) + controllers /
//! services / models / pages 를 가진다. v1은 Fact or Fold 단 1개.

pub mod fact_or_fold;

pub use fact_or_fold::*;
