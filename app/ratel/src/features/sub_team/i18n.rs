use dioxus_translate::*;

use crate::features::sub_team::models::{
    BroadcastTarget, SubTeamAnnouncementStatus, SubTeamFormFieldType,
};
use crate::features::sub_team::models::SubTeamApplicationStatus;

// ── Main UI translations ────────────────────────────────────────────
translate! {
    SubTeamTranslate;

    // Tab labels (parent management page)
    tab_requirements: { en: "Eligibility", ko: "가입 요건" },
    tab_form: { en: "Application Form", ko: "신청폼" },
    tab_documents: { en: "Documents", ko: "문서" },
    tab_sub_teams: { en: "Sub-teams", ko: "하위팀 목록" },
    tab_queue: { en: "Pending Applications", ko: "신청 대기" },
    tab_broadcast: { en: "Broadcast", ko: "전체 공지 관리" },

    // Settings
    settings_is_parent_eligible: {
        en: "Accept sub-team applications",
        ko: "하위팀 신청 받기",
    },
    settings_min_members: { en: "Minimum members", ko: "최소 인원 수" },
    settings_autosaved: { en: "Auto-saved", ko: "자동 저장됨" },
    settings_save: { en: "Save", ko: "저장" },

    // Form fields UI
    add_field: { en: "Add field", ko: "필드 추가" },
    delete_field: { en: "Delete field", ko: "필드 삭제" },
    reorder_field: { en: "Reorder", ko: "순서 변경" },
    field_label: { en: "Label", ko: "라벨" },
    field_type: { en: "Type", ko: "타입" },
    field_required: { en: "Required", ko: "필수" },
    field_options: { en: "Options", ko: "선택지" },

    // Field type labels (also duplicated on the enum Translate derive)
    type_short_text: { en: "Short text", ko: "짧은 텍스트" },
    type_long_text: { en: "Long text", ko: "긴 텍스트" },
    type_number: { en: "Number", ko: "숫자" },
    type_date: { en: "Date", ko: "날짜" },
    type_single_select: { en: "Single select", ko: "단일 선택" },
    type_multi_select: { en: "Multi select", ko: "다중 선택" },
    type_url: { en: "URL", ko: "URL" },

    // Docs UI
    add_document: { en: "Add document", ko: "문서 추가" },
    edit_document: { en: "Edit document", ko: "문서 수정" },
    required_reading: { en: "Must read", ko: "필독" },
    document_body: { en: "Body", ko: "본문" },
    delete_document: { en: "Delete document", ko: "문서 삭제" },
    reorder_document: { en: "Reorder", ko: "순서 변경" },
    document_title: { en: "Title", ko: "제목" },

    // Queue UI
    review_application: { en: "Review application", ko: "신청 심사" },
    approve: { en: "Approve", ko: "승인" },
    reject: { en: "Reject", ko: "반려" },
    reject_reason: { en: "Reason", ko: "사유" },
    r#return: { en: "Return for revision", ko: "반송" },
    return_comment: { en: "Revision comment", ko: "수정 요청 코멘트" },

    // Broadcast UI
    broadcast_title: { en: "Title", ko: "제목" },
    broadcast_body: { en: "Body", ko: "본문" },
    broadcast_compose: { en: "Write announcement", ko: "공지 작성" },
    broadcast_publish: { en: "Publish", ko: "발행" },
    broadcast_draft: { en: "Draft", ko: "드래프트" },
    broadcast_delete: { en: "Delete", ko: "삭제" },
    broadcast_published: { en: "Published", ko: "발행됨" },

    // Apply UI
    select_parent_team: { en: "Select a parent team", ko: "팀 선택" },
    submit_requirement_unmet: {
        en: "Submission requirements not met",
        ko: "제출 조건 미충족",
    },
    missing_required_field: {
        en: "A required field is missing",
        ko: "필수 필드 누락",
    },
    agree_required_docs: {
        en: "Agree to the must-read documents",
        ko: "필독 문서 동의하기",
    },
    agreement_required: { en: "Agreement required", ko: "동의 필요" },
    submit: { en: "Submit", ko: "제출하기" },
    cancel: { en: "Cancel", ko: "취소" },

    // Parent HUD
    parent_icon: { en: "Parent team", ko: "상위팀 아이콘" },
    application_status: { en: "Application status", ko: "신청 상태" },
    leave_parent: { en: "Leave", ko: "이탈" },

    // Bylaws
    bylaws_title: { en: "Bylaws", ko: "학칙" },
    bylaws_regulations: { en: "Regulations", ko: "규정" },

    // Activity dashboard
    window_weekly: { en: "Weekly", ko: "주간" },
    window_monthly: { en: "Monthly", ko: "월간" },
    post_count: { en: "Posts", ko: "포스트 수" },
    space_count: { en: "Spaces", ko: "스페이스 수" },
    active_member_count: { en: "Active members", ko: "활성 멤버 수" },
    privacy_notice_short: {
        en: "Private posts not included",
        ko: "Private 미포함 공지",
    },

    // Generic actions / states
    save: { en: "Save", ko: "저장" },
    edit: { en: "Edit", ko: "수정" },
    delete: { en: "Delete", ko: "삭제" },
    loading: { en: "Loading...", ko: "불러오는 중..." },
    empty_list: { en: "Nothing here yet", ko: "아직 없습니다" },

    // Deregister / leave prompts
    deregister_title: { en: "Deregister sub-team", ko: "하위팀 등록 해제" },
    deregister_reason: { en: "Reason", ko: "사유" },
    deregister_confirm: { en: "Deregister", ko: "해제" },
    leave_parent_title: { en: "Leave parent team", ko: "상위팀 이탈" },
    leave_parent_reason: { en: "Reason (optional)", ko: "사유 (선택)" },
    leave_parent_confirm: { en: "Leave", ko: "이탈" },

    // Page under construction placeholder
    page_under_construction: {
        en: "Page under construction",
        ko: "페이지 준비중",
    },
}

// ── Enum Translate derives ──────────────────────────────────────────
//
// The backend enums live in `features::sub_team::models`; they derive
// Serialize/Deserialize but not Translate. UI layers need a
// locale-aware rendering, so we mirror them here as newtypes that
// implement `Translate`. Callers convert with `.into()`.
//
// This keeps the backend enum definitions free of UI concerns while
// still giving us a single place to maintain EN/KO strings per variant.

#[derive(Debug, Clone, Copy, Translate)]
pub enum SubTeamFormFieldTypeLabel {
    #[translate(en = "Short text", ko = "짧은 텍스트")]
    ShortText,
    #[translate(en = "Long text", ko = "긴 텍스트")]
    LongText,
    #[translate(en = "Number", ko = "숫자")]
    Number,
    #[translate(en = "Date", ko = "날짜")]
    Date,
    #[translate(en = "Single select", ko = "단일 선택")]
    SingleSelect,
    #[translate(en = "Multi select", ko = "다중 선택")]
    MultiSelect,
    #[translate(en = "URL", ko = "URL")]
    Url,
}

impl From<SubTeamFormFieldType> for SubTeamFormFieldTypeLabel {
    fn from(t: SubTeamFormFieldType) -> Self {
        match t {
            SubTeamFormFieldType::ShortText => Self::ShortText,
            SubTeamFormFieldType::LongText => Self::LongText,
            SubTeamFormFieldType::Number => Self::Number,
            SubTeamFormFieldType::Date => Self::Date,
            SubTeamFormFieldType::SingleSelect => Self::SingleSelect,
            SubTeamFormFieldType::MultiSelect => Self::MultiSelect,
            SubTeamFormFieldType::Url => Self::Url,
        }
    }
}

#[derive(Debug, Clone, Copy, Translate)]
pub enum SubTeamApplicationStatusLabel {
    #[translate(en = "Draft", ko = "작성 중")]
    Draft,
    #[translate(en = "Pending", ko = "대기 중")]
    Pending,
    #[translate(en = "Approved", ko = "승인됨")]
    Approved,
    #[translate(en = "Rejected", ko = "반려됨")]
    Rejected,
    #[translate(en = "Returned", ko = "반송됨")]
    Returned,
    #[translate(en = "Cancelled", ko = "취소됨")]
    Cancelled,
}

impl From<SubTeamApplicationStatus> for SubTeamApplicationStatusLabel {
    fn from(s: SubTeamApplicationStatus) -> Self {
        match s {
            SubTeamApplicationStatus::Draft => Self::Draft,
            SubTeamApplicationStatus::Pending => Self::Pending,
            SubTeamApplicationStatus::Approved => Self::Approved,
            SubTeamApplicationStatus::Rejected => Self::Rejected,
            SubTeamApplicationStatus::Returned => Self::Returned,
            SubTeamApplicationStatus::Cancelled => Self::Cancelled,
        }
    }
}

#[derive(Debug, Clone, Copy, Translate)]
pub enum SubTeamAnnouncementStatusLabel {
    #[translate(en = "Draft", ko = "드래프트")]
    Draft,
    #[translate(en = "Published", ko = "발행됨")]
    Published,
    #[translate(en = "Deleted", ko = "삭제됨")]
    Deleted,
}

impl From<SubTeamAnnouncementStatus> for SubTeamAnnouncementStatusLabel {
    fn from(s: SubTeamAnnouncementStatus) -> Self {
        match s {
            SubTeamAnnouncementStatus::Draft => Self::Draft,
            SubTeamAnnouncementStatus::Published => Self::Published,
            SubTeamAnnouncementStatus::Deleted => Self::Deleted,
        }
    }
}

#[derive(Debug, Clone, Translate)]
pub enum BroadcastTargetLabel {
    #[translate(en = "All recognized sub-teams", ko = "모든 등록 하위팀")]
    AllRecognizedSubTeams,
}

impl From<BroadcastTarget> for BroadcastTargetLabel {
    fn from(t: BroadcastTarget) -> Self {
        match t {
            BroadcastTarget::AllRecognizedSubTeams => Self::AllRecognizedSubTeams,
        }
    }
}
