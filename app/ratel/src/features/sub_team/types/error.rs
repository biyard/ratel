use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SubTeamError {
    // ── Parent-eligibility / relationship invariants ───────────────────
    #[error("parent team is not accepting sub-team applications")]
    #[translate(
        en = "This team is not accepting sub-team applications",
        ko = "이 팀은 현재 하위팀 신청을 받지 않습니다."
    )]
    ParentNotEligible,

    #[error("team is already a recognized sub-team")]
    #[translate(
        en = "This team is already a recognized sub-team of another parent",
        ko = "이미 다른 상위팀의 하위팀으로 등록되어 있습니다."
    )]
    AlreadyRecognizedSubTeam,

    #[error("team has an in-flight application")]
    #[translate(
        en = "This team already has a pending or returned application",
        ko = "이 팀에는 이미 처리 중인 신청이 있습니다."
    )]
    ApplicationInFlight,

    #[error("cannot apply to self or descendant")]
    #[translate(
        en = "A team cannot apply to itself or to one of its own sub-teams",
        ko = "자기 자신이나 자신의 하위팀에게는 신청할 수 없습니다."
    )]
    CycleDetected,

    // ── Submit-path validation ────────────────────────────────────────
    #[error("member count below minimum")]
    #[translate(
        en = "The applying team does not meet the parent's minimum member count",
        ko = "신청 팀의 인원 수가 상위팀 최소 기준에 미달합니다."
    )]
    MemberCountBelowMinimum,

    #[error("missing required form field")]
    #[translate(
        en = "One or more required application fields are missing",
        ko = "필수 신청 필드가 누락되었습니다."
    )]
    MissingRequiredFormField,

    #[error("missing required doc agreement")]
    #[translate(
        en = "You must agree to every required document before submitting",
        ko = "제출 전에 모든 필독 문서에 동의해야 합니다."
    )]
    MissingRequiredDocAgreement,

    #[error("doc agreement body hash stale")]
    #[translate(
        en = "A required document has been updated since you agreed — please re-read and re-agree",
        ko = "동의하신 이후 문서가 변경되었습니다. 다시 확인하고 동의해주세요."
    )]
    DocAgreementStale,

    // ── Application decision lifecycle ────────────────────────────────
    #[error("application not found")]
    #[translate(en = "Application not found", ko = "신청을 찾을 수 없습니다.")]
    ApplicationNotFound,

    #[error("application not in expected state")]
    #[translate(
        en = "Application is not in the expected state for this action",
        ko = "이 작업을 수행하기에 신청 상태가 맞지 않습니다."
    )]
    ApplicationStateMismatch,

    // ── Doc / form-field CRUD ─────────────────────────────────────────
    #[error("sub-team document not found")]
    #[translate(en = "Document not found", ko = "문서를 찾을 수 없습니다.")]
    DocumentNotFound,

    #[error("sub-team document body too large")]
    #[translate(
        en = "Document body exceeds the 64 KB limit",
        ko = "문서 본문이 64 KB 제한을 초과했습니다."
    )]
    DocumentBodyTooLarge,

    #[error("sub-team form field not found")]
    #[translate(en = "Form field not found", ko = "신청 필드를 찾을 수 없습니다.")]
    FormFieldNotFound,

    // ── Announcement / broadcast ──────────────────────────────────────
    #[error("announcement not found")]
    #[translate(en = "Announcement not found", ko = "공지를 찾을 수 없습니다.")]
    AnnouncementNotFound,

    #[error("announcement not in draft")]
    #[translate(
        en = "Announcement is no longer in draft and cannot be edited",
        ko = "초안 상태가 아닌 공지는 수정할 수 없습니다."
    )]
    AnnouncementNotInDraft,

    #[error("announcement publish failed")]
    #[translate(
        en = "Announcement could not be published",
        ko = "공지를 게시하지 못했습니다."
    )]
    AnnouncementPublishFailed,

    #[error("broadcast would exceed sub-team cap")]
    #[translate(
        en = "Broadcasting to more than 50 sub-teams is not supported in this release",
        ko = "50개를 초과하는 하위팀에게 공지하는 기능은 아직 지원되지 않습니다."
    )]
    BroadcastTooManySubTeams,

    // ── Leave / deregister ────────────────────────────────────────────
    #[error("team is not a recognized sub-team")]
    #[translate(
        en = "This team is not currently a recognized sub-team of any parent",
        ko = "이 팀은 현재 어느 상위팀에도 속해있지 않습니다."
    )]
    NotASubTeam,

    #[error("sub-team link not found")]
    #[translate(
        en = "The sub-team relationship could not be found",
        ko = "하위팀 관계를 찾을 수 없습니다."
    )]
    SubTeamLinkNotFound,
}

#[cfg(feature = "server")]
impl SubTeamError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        use SubTeamError::*;
        match self {
            ApplicationNotFound
            | DocumentNotFound
            | FormFieldNotFound
            | AnnouncementNotFound
            | SubTeamLinkNotFound => StatusCode::NOT_FOUND,
            AlreadyRecognizedSubTeam | ApplicationInFlight | AnnouncementNotInDraft => {
                StatusCode::CONFLICT
            }
            BroadcastTooManySubTeams => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SubTeamError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SubTeamError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
