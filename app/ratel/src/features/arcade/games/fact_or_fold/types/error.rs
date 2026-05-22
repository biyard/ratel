use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum FactOrFoldError {
    // ── Subject CRUD ──────────────────────────────────────────────
    #[error("subject not found")]
    #[translate(
        en = "Subject not found",
        ko = "대상을 찾을 수 없습니다.",
    )]
    SubjectNotFound,

    #[error("subject already exists")]
    #[translate(
        en = "A subject with the same id already exists",
        ko = "동일한 ID의 대상이 이미 존재합니다.",
    )]
    SubjectAlreadyExists,

    #[error("subject is locked once a round is in progress")]
    #[translate(
        en = "This subject has a live or settled round; only verification sources may be appended",
        ko = "이미 라운드가 진행되었거나 완료된 대상입니다. 검증 출처만 추가할 수 있습니다.",
    )]
    SubjectLocked,

    #[error("subject field validation failed")]
    #[translate(
        en = "Subject field validation failed (length, range, or required field)",
        ko = "대상 입력값이 올바르지 않습니다 (길이/범위/필수 항목).",
    )]
    SubjectInvalid,

    #[error("publish-time invariant violated")]
    #[translate(
        en = "Cannot schedule a subject in the past or publish a draft that is missing required fields",
        ko = "과거 시각으로 예약하거나 필수 항목이 빠진 초안을 발행할 수 없습니다.",
    )]
    PublishInvariantViolation,

    // ── Round play (PR4) ──────────────────────────────────────────
    #[error("round is not in the bet stage")]
    #[translate(
        en = "Bets can only be placed during stage 2",
        ko = "1차 베팅 단계에서만 베팅할 수 있습니다.",
    )]
    BetStageMismatch,

    // ── Flip slot (PR5) ───────────────────────────────────────────
    #[error("flip slot is only open in the last 10s of debate")]
    #[translate(
        en = "Bet flip is only allowed in the last 10 seconds of the debate stage",
        ko = "베팅 변경은 토론 마지막 10초에만 가능합니다.",
    )]
    FlipSlotClosed,

    #[error("flip side must differ from current side")]
    #[translate(
        en = "Flip side must be different from your current bet side",
        ko = "변경할 베팅 사이드는 현재 사이드와 달라야 합니다.",
    )]
    FlipSameSide,

    #[error("flip cite must be another round participant")]
    #[translate(
        en = "Citation must point at another round participant",
        ko = "인용할 참가자는 다른 라운드 참가자여야 합니다.",
    )]
    FlipInvalidCite,

    #[error("flip cite has no rationale to cite")]
    #[translate(
        en = "Cited participant has not submitted a rationale",
        ko = "인용한 참가자가 근거를 제출하지 않았습니다.",
    )]
    FlipCiteNoRationale,

    #[error("flip already used this round")]
    #[translate(
        en = "You have already flipped your bet this round",
        ko = "이번 라운드에서는 이미 베팅을 변경했습니다.",
    )]
    FlipAlreadyUsed,

    #[error("bet must exist before flipping")]
    #[translate(
        en = "You must place a 1st bet before flipping",
        ko = "베팅 변경 전에 1차 베팅을 먼저 해야 합니다.",
    )]
    FlipNoOriginalBet,

    #[error("round is not in the rationale stage")]
    #[translate(
        en = "Rationales can only be submitted during stage 3",
        ko = "근거는 단계 3에서만 제출할 수 있습니다.",
    )]
    RationaleStageMismatch,

    #[error("bet amount out of allowed range")]
    #[translate(
        en = "Bet amount is outside the allowed min..=max range",
        ko = "베팅 금액이 허용 범위를 벗어났습니다.",
    )]
    BetAmountOutOfRange,

    #[error("rationale text invalid")]
    #[translate(
        en = "Rationale text is empty or exceeds the max length",
        ko = "근거 텍스트가 비어있거나 최대 길이를 초과했습니다.",
    )]
    RationaleInvalid,

    #[error("not a round participant")]
    #[translate(
        en = "Only round participants can post bets, rationales, or chat",
        ko = "라운드 참가자만 베팅·근거·채팅을 게시할 수 있습니다.",
    )]
    NotRoundParticipant,

    #[error("not the round insider")]
    #[translate(
        en = "Only the insider can read this private statement",
        ko = "인사이더 본인만 비공개 진술을 조회할 수 있습니다.",
    )]
    NotRoundInsider,

    // ── Lobby + round (PR3) ───────────────────────────────────────
    #[error("no subject available for a new round")]
    #[translate(
        en = "No published subject is available right now — try again later",
        ko = "현재 발행된 대상이 없습니다 — 잠시 후 다시 시도해주세요.",
    )]
    LobbyNoSubjectAvailable,

    #[error("lobby round is full")]
    #[translate(
        en = "The current round is already full",
        ko = "현재 라운드가 이미 만석입니다.",
    )]
    LobbyFull,

    #[error("already joined the current round")]
    #[translate(
        en = "You are already in the current round",
        ko = "이미 라운드에 참여 중입니다.",
    )]
    LobbyAlreadyJoined,

    #[error("not in the current round")]
    #[translate(
        en = "You are not in the current round",
        ko = "현재 라운드에 참여 중이 아닙니다.",
    )]
    LobbyNotJoined,

    #[error("insufficient balance to join")]
    #[translate(
        en = "Insufficient RatelPoint balance to join — need at least {0} RP",
        ko = "참여에 필요한 최소 RP가 부족합니다 — {0} RP 필요.",
    )]
    LobbyInsufficientBalance(i64),

    #[error("round not found")]
    #[translate(
        en = "Round not found",
        ko = "라운드를 찾을 수 없습니다.",
    )]
    RoundNotFound,

    #[error("round has not been settled yet")]
    #[translate(
        en = "Round results are not available yet — settlement has not run.",
        ko = "아직 라운드가 정산되지 않았습니다.",
    )]
    RoundNotSettled,

    // ── Settings ──────────────────────────────────────────────────
    #[error("settings field out of range")]
    #[translate(
        en = "A settings value is outside the allowed range",
        ko = "설정값이 허용 범위를 벗어났습니다.",
    )]
    SettingsOutOfRange,

    // ── Generic admin failures ────────────────────────────────────
    #[error("storage failure")]
    #[translate(
        en = "Storage operation failed",
        ko = "저장 작업에 실패했습니다.",
    )]
    StorageFailure,
}

#[cfg(feature = "server")]
impl FactOrFoldError {
    pub fn status_code(&self) -> crate::axum::http::StatusCode {
        use crate::axum::http::StatusCode;
        match self {
            FactOrFoldError::SubjectNotFound => StatusCode::NOT_FOUND,
            FactOrFoldError::SubjectAlreadyExists => StatusCode::CONFLICT,
            FactOrFoldError::SubjectLocked
            | FactOrFoldError::SubjectInvalid
            | FactOrFoldError::PublishInvariantViolation
            | FactOrFoldError::SettingsOutOfRange
            | FactOrFoldError::LobbyAlreadyJoined
            | FactOrFoldError::LobbyNotJoined
            | FactOrFoldError::LobbyInsufficientBalance(_)
            | FactOrFoldError::BetStageMismatch
            | FactOrFoldError::RationaleStageMismatch
            | FactOrFoldError::BetAmountOutOfRange
            | FactOrFoldError::RationaleInvalid
            | FactOrFoldError::NotRoundParticipant
            | FactOrFoldError::FlipSlotClosed
            | FactOrFoldError::FlipSameSide
            | FactOrFoldError::FlipInvalidCite
            | FactOrFoldError::FlipCiteNoRationale
            | FactOrFoldError::FlipAlreadyUsed
            | FactOrFoldError::FlipNoOriginalBet => StatusCode::BAD_REQUEST,
            FactOrFoldError::NotRoundInsider => StatusCode::FORBIDDEN,
            FactOrFoldError::LobbyFull => StatusCode::CONFLICT,
            FactOrFoldError::LobbyNoSubjectAvailable => StatusCode::SERVICE_UNAVAILABLE,
            FactOrFoldError::RoundNotFound => StatusCode::NOT_FOUND,
            FactOrFoldError::RoundNotSettled => StatusCode::CONFLICT,
            FactOrFoldError::StorageFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for FactOrFoldError {
    fn into_response(self) -> crate::axum::response::Response {
        use crate::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for FactOrFoldError {
    fn as_status_code(&self) -> crate::axum::http::StatusCode {
        self.status_code()
    }
}
