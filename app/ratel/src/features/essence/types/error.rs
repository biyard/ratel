use crate::*;
use dioxus_translate::Translate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Feature-specific error enum for the Essence domain. Registered on
/// `common::Error` via `#[translate(from)]` so the translated message
/// flows through unchanged.
#[derive(Debug, Error, Clone, Serialize, Deserialize, Translate)]
pub enum EssenceError {
    #[error("essence not found")]
    #[translate(en = "Essence not found", ko = "항목을 찾을 수 없습니다")]
    NotFound,

    #[error("essence belongs to another user")]
    #[translate(
        en = "You don't have permission to modify this essence",
        ko = "이 항목을 수정할 권한이 없습니다"
    )]
    Forbidden,

    #[error("failed to read essence")]
    #[translate(en = "Failed to load essences", ko = "항목을 불러오지 못했습니다")]
    ReadFailed,

    #[error("failed to upsert essence")]
    #[translate(en = "Failed to save essence", ko = "항목을 저장하지 못했습니다")]
    UpsertFailed,

    #[error("failed to delete essence")]
    #[translate(en = "Failed to delete essence", ko = "항목을 삭제하지 못했습니다")]
    DeleteFailed,

    #[error("failed to migrate essence")]
    #[translate(en = "Migration failed", ko = "마이그레이션에 실패했습니다")]
    MigrationFailed,
}
