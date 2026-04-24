use dioxus_translate::*;

use crate::features::sub_team::models::SubTeamApplicationStatus;
use crate::features::sub_team::models::{
    BroadcastTarget, SubTeamAnnouncementStatus, SubTeamFormFieldType,
};
use crate::features::sub_team::types::ParentRelationshipStatus;

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

    // Apply page
    apply_page_title: { en: "Apply as Sub-team", ko: "하위팀 가입 신청" },
    apply_page_eyebrow: { en: "Apply as Sub-team", ko: "하위팀 가입 신청" },
    apply_target_label: { en: "Target parent team", ko: "신청 대상" },
    apply_select_parent: { en: "Select a parent team", ko: "상위팀 선택" },
    apply_required_docs: { en: "Required reading", ko: "필독 문서" },
    apply_docs_open_review: { en: "Open to review", ko: "열어서 검토" },
    apply_docs_agreed: { en: "Agreed", ko: "동의 완료" },
    apply_form_fields: { en: "Application form", ko: "신청폼" },
    apply_eligibility_title: { en: "Eligibility check", ko: "가입 요건" },
    apply_elig_min_members: { en: "Minimum members met", ko: "최소 멤버 수 충족" },
    apply_elig_form_filled: { en: "Required fields filled", ko: "필수 필드 작성 완료" },
    apply_elig_docs_agreed: { en: "Required docs agreed", ko: "필독 문서 동의 완료" },
    apply_submit: { en: "Submit application", ko: "신청 제출" },
    apply_submit_sub: {
        en: "All requirements must be met before submitting",
        ko: "모든 요건이 충족되면 제출할 수 있습니다",
    },
    apply_parent_eligible_off: {
        en: "This team is not accepting sub-team applications",
        ko: "이 팀은 현재 하위팀 신청을 받지 않습니다",
    },

    // Doc agreement modal
    doc_modal_eyebrow: { en: "Required reading", ko: "필독 문서" },
    doc_modal_agree: { en: "Agree", ko: "동의하기" },
    doc_modal_agreed: { en: "Agreed", ko: "동의 완료" },
    doc_modal_cancel: { en: "Cancel", ko: "취소" },
    doc_modal_notice: {
        en: "Clicking Agree records this document version and your consent timestamp permanently. Cancelling does not save your agreement.",
        ko: "동의하기를 누르면 제출 기록에 이 문서의 현재 버전과 동의 시점이 영구 보관됩니다. 취소하면 동의 상태가 저장되지 않습니다.",
    },

    // Application status
    status_page_title: { en: "Application Status", ko: "신청 상태" },
    status_page_eyebrow: { en: "Application Tracker", ko: "신청 상태" },
    status_relationship_standalone: { en: "Standalone team", ko: "독립 팀" },
    status_relationship_pending: { en: "Application pending", ko: "심사 대기 중" },
    status_relationship_recognized: { en: "Recognized sub-team", ko: "인증된 하위팀" },
    status_edit_and_resubmit: { en: "Edit and resubmit", ko: "수정 후 재제출" },
    status_cancel_application: { en: "Cancel application", ko: "신청 취소" },
    status_history: { en: "Application history", ko: "신청 기록" },
    status_no_applications: {
        en: "No applications yet",
        ko: "신청 내역이 없습니다",
    },
    status_decision_reason: { en: "Decision reason", ko: "심사 결과 사유" },
    status_latest_application: { en: "Latest application", ko: "최근 신청" },

    // Doc compose
    doc_compose_title_new: { en: "New document", ko: "새 문서" },
    doc_compose_title_edit: { en: "Edit document", ko: "문서 편집" },
    doc_compose_eyebrow: { en: "Sub-team · Document", ko: "하위팀 · 문서" },
    doc_compose_title_placeholder: {
        en: "Document title",
        ko: "문서 제목",
    },
    doc_compose_body_placeholder: {
        en: "Write document content…",
        ko: "문서 내용을 작성하세요…",
    },
    doc_compose_required_on: { en: "Marked as required", ko: "필독으로 지정됨" },
    doc_compose_required_off: { en: "Not required", ko: "필독 아님" },
    doc_compose_required_desc: {
        en: "Sub-teams must read and agree to this document before submitting.",
        ko: "하위팀이 가입 신청할 때 이 문서를 읽고 동의해야 합니다.",
    },
    doc_compose_word_count: { en: "characters", ko: "글자" },

    // Leave parent
    leave_parent_page_eyebrow: { en: "Leave Parent Team", ko: "상위팀 이탈" },
    leave_parent_current_tie: { en: "Current relationship", ko: "현재 소속" },
    leave_parent_keep_title: { en: "What stays", ko: "유지되는 것" },
    leave_parent_keep_team: {
        en: "Your team (members, posts, spaces) — continues as a standalone team.",
        ko: "팀 자체 (멤버, 게시물, 스페이스) — 독립 팀으로 계속 운영.",
    },
    leave_parent_keep_announcements: {
        en: "Past announcements from the parent remain as normal posts (unpinned).",
        ko: "부모팀이 과거에 보낸 공지들은 일반 게시물로 남습니다 (핀 해제됨).",
    },
    leave_parent_keep_admins: {
        en: "Team admins and member composition.",
        ko: "동아리 admin 권한과 멤버 구성.",
    },
    leave_parent_lose_title: { en: "What you lose", ko: "상실하는 것" },
    leave_parent_lose_broadcasts: {
        en: "No more broadcasts from the parent team.",
        ko: "앞으로 부모팀 공지를 받지 않습니다.",
    },
    leave_parent_lose_dashboard: {
        en: "Your activity will no longer count in the parent's dashboard.",
        ko: "부모팀 활동 대시보드에 집계되지 않습니다.",
    },
    leave_parent_lose_badge: {
        en: "The parent affiliation badge is removed from your team profile.",
        ko: "팀 프로필의 학과 소속 표시가 제거됩니다.",
    },
    leave_parent_lose_reapply: {
        en: "Rejoining requires a brand-new application.",
        ko: "재가입하려면 정식 신청 절차를 다시 거쳐야 합니다.",
    },
    leave_parent_reason_placeholder: {
        en: "Optional message to parent team admins.",
        ko: "부모팀 admin에게 남길 메시지 (선택)",
    },
    leave_parent_reason_hint: {
        en: "This message is included in the parent team's leave notification. Leaving still works if you skip it.",
        ko: "이 메시지는 부모팀 admin에게 전달되는 이탈 알림에 포함됩니다. 생략해도 이탈은 실행됩니다.",
    },
    leave_parent_confirm_checkbox: {
        en: "I confirm leaving the parent team and becoming a standalone team.",
        ko: "상위팀과의 sub-team 관계를 해제하고 독립 팀이 되는 것에 동의합니다.",
    },
    leave_parent_not_recognized: {
        en: "This team is not currently a recognized sub-team.",
        ko: "이 팀은 현재 인증된 하위팀이 아닙니다.",
    },

    // Bylaws page
    bylaws_page_eyebrow: { en: "Bylaws", ko: "학칙" },
    bylaws_team_regulations: {
        en: "Team regulations",
        ko: "팀 규정",
    },
    bylaws_parent_regulations: {
        en: "Parent team regulations",
        ko: "상위팀 규정",
    },
    bylaws_empty: {
        en: "No bylaws published yet",
        ko: "공개된 학칙이 없습니다",
    },
    bylaws_required_badge: {
        en: "Required",
        ko: "필독",
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

#[derive(Debug, Clone, Copy, Translate)]
pub enum ParentRelationshipStatusLabel {
    #[translate(en = "Standalone team", ko = "독립 팀")]
    Standalone,
    #[translate(en = "Application pending", ko = "심사 대기 중")]
    PendingSubTeam,
    #[translate(en = "Recognized sub-team", ko = "인증된 하위팀")]
    RecognizedSubTeam,
}

impl From<ParentRelationshipStatus> for ParentRelationshipStatusLabel {
    fn from(s: ParentRelationshipStatus) -> Self {
        match s {
            ParentRelationshipStatus::Standalone => Self::Standalone,
            ParentRelationshipStatus::PendingSubTeam => Self::PendingSubTeam,
            ParentRelationshipStatus::RecognizedSubTeam => Self::RecognizedSubTeam,
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
