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

    // Activation hero (always visible above tabs)
    activation_label_on: { en: "Parent-eligible · ON", ko: "하위팀 신청 받기 · ON" },
    activation_label_off: { en: "Parent-eligible · OFF", ko: "하위팀 신청 받기 · OFF" },
    activation_title_on: {
        en: "Accepting sub-team applications",
        ko: "하위팀 가입 신청을 받고 있습니다",
    },
    activation_title_off: {
        en: "Sub-team applications closed",
        ko: "하위팀 가입 신청을 받지 않습니다",
    },
    activation_desc: {
        en: "While ON, the “Sub-team” icon on this team's home routes visitors to the apply page. Toggling OFF stops new applications but keeps existing relationships, broadcasts, and the dashboard fully operational.",
        ko: "ON 상태일 때, 이 팀 홈의 \"하위팀\" 아이콘이 방문자를 가입 신청 페이지로 보냅니다. OFF 로 바꿔도 기존 하위팀 관계, 공지, 대시보드는 정상 작동합니다.",
    },

    // KPI row
    kpi_recognized: { en: "Recognized sub-teams", ko: "인증 하위팀" },
    kpi_pending: { en: "Pending applications", ko: "심사 대기" },
    kpi_last_broadcast: { en: "Last broadcast", ko: "마지막 공지" },
    kpi_pending_review: { en: "Needs review", ko: "검토 필요" },
    kpi_no_broadcast: { en: "—", ko: "—" },

    // Requirements card (mockup line 392-431)
    req_card_title: { en: "Eligibility · Team-level checks", ko: "가입 요건 · TEAM-LEVEL CHECKS" },
    req_card_meta: { en: "Objective team metrics only", ko: "객관적 팀 지표만" },
    req_min_members_title: { en: "Minimum members", ko: "최소 멤버 수" },
    req_min_members_desc: {
        en: "Team must reach this headcount before submitting",
        ko: "팀이 이 인원을 달성해야 제출 가능",
    },
    req_min_days_title: { en: "Minimum team age", ko: "팀 생성 최소 기간" },
    req_min_days_desc: {
        en: "Team must exist for at least this many days before applying",
        ko: "팀이 만들어진 후 최소 일수 후에 신청 가능",
    },
    req_inline_note: {
        en: "For per-team info, mark the field as required in the Application form. Bylaws and similar documents go in the Documents tab — set them as required reading and applicants must read & agree before submitting.",
        ko: "팀별 정보는 신청폼에서 \"필수\" 체크로 받으세요. 규정 같은 문서는 문서 탭에서 만들고 \"필독\"으로 지정하면 가입 신청 때 반드시 읽고 동의해야 제출됩니다.",
    },

    // Form card (mockup line 433-505)
    form_card_title: { en: "Application form", ko: "신청폼 · APPLICATION FORM" },
    form_meta_fields: { en: "fields", ko: "필드" },
    form_meta_required: { en: "required", ko: "필수" },
    form_notice_title: { en: "Linked field", ko: "LINKED FIELD" },
    form_notice_text: {
        en: "Click the 🔗 Link button next to a field to bind its default value to an attribute on the applicant team's profile.",
        ko: "각 필드 옆의 🔗 Link 버튼을 클릭하면 신청자 팀 프로필의 어떤 속성에서 자동으로 값을 가져올지 지정할 수 있습니다.",
    },
    form_new_field: { en: "New field", ko: "새 필드" },

    // Linked-field selector options (per field row)
    form_link_none: { en: "Link", ko: "Link" },
    form_link_team_name: { en: "Team name", ko: "팀 이름" },
    form_link_team_username: { en: "Username", ko: "Username" },
    form_link_team_bio: { en: "Bio / about", ko: "팀 소개 / BIO" },
    form_link_team_profile_url: { en: "Profile image", ko: "프로필 이미지" },

    // Single/multi select options panel
    form_options_title: { en: "Options", ko: "선택지" },
    form_options_placeholder: { en: "Option label…", ko: "선택지 라벨…" },
    form_options_add: { en: "Add option", ko: "선택지 추가" },

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
    docs_card_title: { en: "Documents · DOCUMENTS", ko: "문서 · DOCUMENTS" },
    docs_banner_title: { en: "Documents shown on apply", ko: "가입 신청에서 보이는 문서" },
    docs_banner_text: {
        en: "Manage operating guides / regulations here. Docs marked as Required reading must be read & agreed to before sub-teams submit. Past agreements stay anchored to the version that was published at submit time.",
        ko: "운영 가이드·기타 규정을 여기서 관리하세요. 가입 신청 필독으로 지정한 문서는 하위팀이 신청할 때 반드시 읽고 동의해야 제출됩니다. 문서 내용을 나중에 수정해도 기존 신청의 동의 기록은 당시 문구로 고정됩니다.",
    },
    docs_updated_suffix: { en: "edited", ko: "수정" },
    docs_edit_btn: { en: "Edit", ko: "편집" },
    docs_add_btn: { en: "New document", ko: "새 문서 작성" },
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
    sub_team_options: { en: "Options", ko: "옵션" },
    sub_team_options_drawer_title: { en: "Document options", ko: "문서 옵션" },
    sub_team_options_close: { en: "Close options", ko: "옵션 닫기" },
    broadcast_draft: { en: "Draft", ko: "드래프트" },
    broadcast_delete: { en: "Delete", ko: "삭제" },
    broadcast_published: { en: "Published", ko: "발행됨" },
    broadcast_title_placeholder: { en: "Enter a title…", ko: "제목을 입력하세요…" },
    broadcast_body_placeholder: {
        en: "Tell your story… use the toolbar to add formatting, links and images.",
        ko: "이야기를 들려주세요… 도구 모음을 사용해 서식, 링크, 이미지를 추가할 수 있어요.",
    },
    broadcast_posting_as: { en: "Posting as", ko: "게시자" },
    broadcast_section_tags: { en: "Tags", ko: "태그" },
    broadcast_section_attachments: { en: "Attachments", ko: "첨부 파일" },
    broadcast_attachments_none: { en: "No attachments", ko: "첨부 없음" },
    broadcast_attachments_upload_title: { en: "Upload file", ko: "파일 업로드" },
    broadcast_attachments_upload_hint: { en: "PDF · DOCX · PPTX · XLSX · PNG · JPG", ko: "PDF · DOCX · PPTX · XLSX · PNG · JPG" },
    broadcast_discard_draft: { en: "Discard draft", ko: "초안 버리기" },
    broadcast_status_idle: { en: "Auto-save enabled", ko: "자동 저장 활성화" },
    broadcast_status_dirty: { en: "Unsaved changes", ko: "저장되지 않은 변경" },
    broadcast_status_saving: { en: "Saving…", ko: "저장 중…" },
    broadcast_status_saved: { en: "Auto-saved just now", ko: "자동 저장됨 방금" },
    broadcast_status_error: { en: "Save failed", ko: "저장 실패" },
    broadcast_save_draft_manual: { en: "Save draft", ko: "초안 저장" },
    broadcast_compose_sub: {
        en: "Publish to every recognized sub-team — in-app notifications.",
        ko: "인식된 모든 하위팀에 발행 · 인앱 알림.",
    },
    broadcast_draft_chars: { en: "{n} chars", ko: "{n}자" },
    broadcast_draft_posting_as: { en: "Posting as", ko: "게시자" },
    broadcast_draft_target: { en: "Broadcast", ko: "Broadcast" },
    broadcast_drafts_card_title: { en: "Drafts", ko: "작성 중 · Drafts" },
    broadcast_untitled: { en: "(Untitled draft)", ko: "(제목 없음)" },
    broadcast_published_card_title: {
        en: "Published",
        ko: "발행된 공지 · Published",
    },
    broadcast_published_count: { en: "{n} published", ko: "{n}건" },
    broadcast_drafts_count: { en: "{n} drafts", ko: "{n}건" },
    broadcast_pinned: { en: "Pinned", ko: "Pinned" },
    broadcast_unpinned: { en: "Unpinned", ko: "Unpinned" },
    broadcast_fanout_meta: {
        en: "{n} sub-teams notified",
        ko: "{n} 하위팀 알림",
    },
    broadcast_comments_meta: { en: "{n} comments", ko: "{n} 댓글" },
    broadcast_autosaved_idle: { en: "Saved", ko: "저장됨" },
    broadcast_autosaved_just_now: { en: "Just now", ko: "방금 자동 저장" },
    broadcast_autosaved_minutes: {
        en: "{n} min ago auto-saved",
        ko: "{n}분 전 자동 저장",
    },
    broadcast_autosaved_hours: {
        en: "{n} hours ago auto-saved",
        ko: "{n}시간 전 자동 저장",
    },
    broadcast_autosaved_yesterday: {
        en: "Yesterday auto-saved",
        ko: "어제 자동 저장",
    },
    broadcast_autosaved_days: {
        en: "{n} days ago auto-saved",
        ko: "{n}일 전 자동 저장",
    },
    broadcast_time_today: { en: "Today", ko: "오늘" },
    broadcast_time_yesterday: { en: "Yesterday", ko: "어제" },
    broadcast_time_days_ago: { en: "{n} days ago", ko: "{n}일 전" },
    // Space toggle copy reused from post_edit (Korean translation
    // mirrored verbatim so the broadcast composer looks identical).
    broadcast_enable_space: { en: "Enable Space", ko: "스페이스 활성화" },
    broadcast_space_hint: {
        en: "Turn this announcement into a Space — every recognized sub-team's members can vote, discuss, and earn rewards together.",
        ko: "이 게시물을 스페이스로 전환하세요. 투표하고, 토론하고, 보상을 함께 받을 수 있는 전용 공간을 제공합니다.",
    },
    broadcast_space_active_hint: {
        en: "Publish will take you to the Space designer to configure quests, rewards, and members.",
        ko: "게시하면 퀘스트, 보상, 멤버를 설정하는 스페이스 디자이너로 이동합니다.",
    },
    broadcast_tag_placeholder: { en: "Add a tag…", ko: "태그 추가…" },
    broadcast_remove_tag: { en: "Remove tag", ko: "태그 제거" },

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
    parent_panel_title: { en: "Parent · PARENT", ko: "상위팀 · PARENT" },
    application_status: { en: "Application status", ko: "신청 상태" },
    leave_parent: { en: "Leave", ko: "이탈" },

    // Parent HUD panel — per-status copy mirroring
    // `parent-home-with-button.html` (인증됨 / 심사 중 / 독립 팀).
    parent_card_status_active: { en: "Active", ko: "Active" },
    parent_card_status_pending: { en: "Pending", ko: "Pending" },
    parent_card_meta_recognized_since: {
        en: "Recognized since",
        ko: "인증",
    },
    parent_card_meta_pending: {
        en: "Submitted",
        ko: "제출",
    },
    parent_recognized_info_prefix: {
        en: "This team is a recognized sub-team of ",
        ko: "이 팀은 ",
    },
    parent_recognized_info_suffix: {
        en: ". You'll receive parent broadcasts and your activity will show on the parent dashboard.",
        ko: "의 인증 하위팀입니다. 상위팀의 공지를 받고, 상위팀 활동 대시보드에 공개 활동이 집계됩니다.",
    },
    parent_pending_info: {
        en: "The parent admin is reviewing your application. It usually takes 3–5 business days.",
        ko: "상위팀 담당자가 신청서를 검토 중입니다. 보통 영업일 3–5일 소요됩니다.",
    },
    parent_action_open_home_title: {
        en: "Open parent team home",
        ko: "상위팀 홈으로 이동",
    },
    parent_action_open_home_sub: {
        en: "View parent team's feed",
        ko: "상위팀 팀 피드 열기",
    },
    parent_action_view_bylaws_title: {
        en: "View parent bylaws",
        ko: "상위팀 운영 수칙 보기",
    },
    parent_action_view_bylaws_sub: {
        en: "Read the regulations and documents",
        ko: "규정 및 문서 확인",
    },
    parent_action_leave_title: {
        en: "Leave parent team",
        ko: "상위팀에서 이탈",
    },
    parent_action_leave_sub: {
        en: "Become a standalone team — your content is kept",
        ko: "독립 팀으로 전환 · 콘텐츠는 유지",
    },
    parent_action_view_application_title: {
        en: "Check application status",
        ko: "신청 상태 확인",
    },
    parent_action_view_application_sub: {
        en: "Open timeline · feedback from the parent",
        ko: "타임라인 · 상위팀 피드백 열기",
    },
    parent_action_edit_application_title: {
        en: "Edit application",
        ko: "신청서 수정",
    },
    parent_action_edit_application_sub: {
        en: "Still under review · update the content",
        ko: "아직 심사 대기 중 · 내용 업데이트",
    },
    parent_action_cancel_application_title: {
        en: "Cancel application",
        ko: "신청 취소",
    },
    parent_action_cancel_application_sub: {
        en: "Withdraw the application currently under review",
        ko: "현재 심사 중인 신청을 철회",
    },
    parent_standalone_title: {
        en: "Standalone team",
        ko: "독립 팀입니다",
    },
    parent_standalone_desc: {
        en: "This team isn't part of any parent team yet.",
        ko: "아직 어떤 상위팀에도 속해있지 않습니다.",
    },

    // Bylaws
    bylaws_title: { en: "Bylaws", ko: "운영 수칙" },
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
    growth_score: { en: "Growth score", ko: "성장 점수" },
    growth_score_placeholder: { en: "—", ko: "—" },
    growth_score_meta: { en: "Not available yet", ko: "아직 집계 전" },
    trend_title_weekly: { en: "Activity trend · Weekly", ko: "활동 추이 · 주간" },
    trend_title_monthly: { en: "Activity trend · Monthly", ko: "활동 추이 · 월간" },
    trend_legend_posts: { en: "Posts", ko: "포스트" },
    trend_legend_members: { en: "Active members", ko: "활성 멤버" },
    member_search_placeholder: { en: "Search by handle…", ko: "핸들 검색…" },
    member_sort_active: { en: "Sort: most active", ko: "정렬: 활동 많은 순" },
    per_member_activity: { en: "Per-member activity", ko: "멤버별 활동" },
    direct_announce_title: {
        en: "Direct announcement",
        ko: "이 하위팀에만 공지",
    },
    direct_announce_placeholder: {
        en: "Write a private announcement for this sub-team only…",
        ko: "이 하위팀에만 보낼 공지를 작성하세요…",
    },
    direct_announce_send: { en: "Send", ko: "발송" },
    direct_announce_title_input: {
        en: "Announcement title",
        ko: "공지 제목",
    },
    direct_announce_note: {
        en: "Pinned to this sub-team's feed only · not counted as a broadcast",
        ko: "이 하위팀에만 핀 고정됩니다 · 전체 공지와 별개로 집계",
    },
    direct_announce_history_title: {
        en: "Past direct announcements",
        ko: "이전 공지 이력",
    },
    direct_announce_history_empty: {
        en: "No direct announcements sent yet.",
        ko: "아직 보낸 공지가 없습니다.",
    },
    danger_zone: { en: "Danger zone", ko: "위험 영역" },
    member_handle_header: { en: "Handle", ko: "멤버" },
    member_posts_header: { en: "Posts", ko: "포스트" },
    member_spaces_header: { en: "Spaces", ko: "스페이스" },
    member_last_active_header: { en: "Last active", ko: "최근 활동" },

    // Generic actions / states
    save: { en: "Save", ko: "저장" },
    edit: { en: "Edit", ko: "수정" },
    delete: { en: "Delete", ko: "삭제" },
    loading: { en: "Loading...", ko: "불러오는 중..." },
    empty_list: { en: "Nothing here yet", ko: "아직 없습니다" },

    // Deregister / leave prompts
    deregister_title: { en: "Deregister sub-team", ko: "하위팀 등록 해제" },
    deregister_reason: { en: "Reason", ko: "사유" },
    deregister_confirm: { en: "Deregister", ko: "해제 확정" },
    deregister_status_chip: { en: "Danger action", ko: "위험한 조작" },
    deregister_eyebrow: { en: "Deregister sub-team", ko: "하위팀 등록 해제" },
    deregister_header_title_prefix: { en: "Deregister ", ko: "" },
    deregister_header_title_suffix: {
        en: " from this parent team",
        ko: "의 등록을 해제합니다",
    },
    deregister_header_sub: {
        en: "This only removes the parent-child link. The team keeps running standalone — the parent cannot edit or delete its content.",
        ko: "이 동작은 부모-자식 관계만 해제합니다. 동아리 자체는 독립 팀으로 계속 운영됩니다. 상위 팀은 이 동아리의 콘텐츠를 수정하거나 삭제할 수 없습니다.",
    },
    deregister_consequences_title: {
        en: "What happens after deregister?",
        ko: "해제 후 어떻게 됩니까?",
    },
    deregister_consequence_unlink: {
        en: "The sub-team's parent_team_id is cleared and it becomes a standalone team.",
        ko: "하위팀의 parent_team_id가 제거되어 독립 팀으로 전환됩니다.",
    },
    deregister_consequence_notify: {
        en: "Sub-team admins receive a deregister notification with the reason you write below.",
        ko: "하위팀 admin은 해제 알림과 사유를 전달받습니다.",
    },
    deregister_consequence_demote: {
        en: "Past announcements you sent become normal posts and stay in the team's space.",
        ko: "이전에 발송한 공지들은 일반 게시물로 강등되어 팀 스페이스에 남아 있습니다.",
    },
    deregister_consequence_content: {
        en: "The team's members, posts, and spaces are NOT affected.",
        ko: "동아리의 멤버, 게시물, 스페이스는 전혀 영향을 받지 않습니다.",
    },
    deregister_consequence_reapply: {
        en: "Re-recognition requires a brand-new application.",
        ko: "향후 이 동아리가 다시 인증을 받으려면 정식 신청 절차를 거쳐야 합니다.",
    },
    deregister_reason_placeholder: {
        en: "Explain why you're deregistering — this is delivered verbatim to the sub-team admin.",
        ko: "예: 최근 3개월 간 정기 활동이 확인되지 않았고, 지도교수 변경 사항 공지도 누락되었습니다. 동아리 활성화 후 재신청해주시면 환영합니다.",
    },
    deregister_reason_hint: {
        en: "Sent verbatim to the sub-team admins and permanently logged with the deregister record.",
        ko: "이 메시지는 동아리 admin에게 그대로 전달되며, 해제 기록에 영구 보관됩니다.",
    },
    deregister_notif_preview_label: {
        en: "Notification preview · what the sub-team admin will receive",
        ko: "알림 미리보기 · 하위팀 admin이 받을 알림",
    },
    deregister_notif_preview_title_prefix: { en: "", ko: "" },
    deregister_notif_preview_title_suffix: {
        en: " has deregistered your team",
        ko: "에서 팀 등록을 해제했습니다",
    },
    deregister_notif_preview_empty: {
        en: "Your reason will appear here as you type.",
        ko: "해제 사유가 여기에 표시됩니다 — 왼쪽 텍스트 영역에 입력한 내용이 실시간으로 반영됩니다.",
    },
    deregister_confirm_check: {
        en: "I understand the above and confirm deregistering this sub-team.",
        ko: "위 내용을 이해했고 등록 해제를 확인합니다.",
    },
    deregister_confirm_check_hint: {
        en: "Check this box to enable the \"Deregister\" button.",
        ko: "체크해야 \"해제 확정\" 버튼이 활성화됩니다.",
    },
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
    apply_page_header_sub: {
        en: "Pick the team you admin and its profile (name / bio / lead) auto-fills the form. You can edit any field inline.",
        ko: "내가 관리자인 팀을 선택하면 팀 프로필(이름·소개·지도자 등)이 신청폼에 자동 채워집니다. 필요하면 이 자리에서 바로 수정할 수 있어요.",
    },
    apply_status_drafting: { en: "Drafting", ko: "초안" },
    apply_save_draft: { en: "Save draft", ko: "초안 저장" },
    apply_progress_label: { en: "Requirements met", ko: "요건 충족도" },
    apply_elig_admin_title: { en: "Team admin permission", ko: "팀의 관리자 권한" },
    apply_elig_admin_desc: { en: "Must be an admin of the selected team", ko: "선택한 팀의 admin이어야 신청 가능" },
    apply_elig_min_members_desc: {
        en: "Team must reach this headcount before submitting",
        ko: "팀이 이 인원을 달성해야 제출 가능",
    },
    apply_elig_min_days_title: { en: "Team age requirement", ko: "팀 생성 최소 기간" },
    apply_elig_min_days_desc: { en: "Recently-created teams can't apply yet", ko: "최근 만들어진 팀은 신청 불가" },
    apply_elig_docs_desc: { en: "Read & agree to every required doc", ko: "지정한 필독 문서를 모두 읽고 동의" },
    apply_elig_form_desc: { en: "Fill every required field", ko: "필수 필드 모두 작성" },
    apply_pick_your_team: { en: "Pick your team", ko: "내가 관리자인 팀 선택" },
    apply_picker_placeholder: { en: "Select a team", ko: "팀을 선택하세요" },
    apply_picker_empty: {
        en: "You're not an admin of any team yet",
        ko: "관리자 권한이 있는 팀이 없습니다",
    },
    apply_doc_view_open: { en: "Open to read", ko: "열어 보기" },
    apply_doc_reference_badge: { en: "Reference", ko: "참고" },
    apply_close: { en: "Close", ko: "닫기" },
    apply_submit_progress_prefix: { en: "Requirements", ko: "요건" },
    apply_submit_progress_suffix: { en: "met", ko: "충족" },

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

    // Parent queue row — applicant info line + "review" button.
    queue_row_members_suffix: { en: "members", ko: "명" },
    queue_row_form_snapshot: { en: "Form snapshot", ko: "폼 스냅샷" },
    queue_row_review_btn: { en: "Review application", ko: "신청서 확인하기" },
    queue_row_confirm: { en: "Confirm", ko: "확인" },
    queue_row_approve_placeholder: {
        en: "Optional note for the applicant (e.g. welcome message)",
        ko: "신청자에게 전할 메모 (선택 사항) — 환영 메시지나 안내 사항",
    },
    queue_row_return_placeholder: {
        en: "Please describe what needs to be revised before resubmission",
        ko: "재제출 전 수정해야 할 내용을 설명해주세요",
    },
    queue_row_reject_placeholder: {
        en: "Please explain why the application was rejected",
        ko: "거절 사유를 설명해주세요",
    },

    // Review modal (parent-side, click "Review application")
    review_modal_eyebrow: { en: "Application review", ko: "신청서 검토" },
    review_modal_submitted_at: { en: "Submitted", ko: "제출 시각" },
    review_modal_submitter: { en: "Submitter", ko: "제출자" },
    review_modal_form_section: { en: "Submitted answers", ko: "제출 내역" },
    review_modal_fields_suffix: { en: "fields", ko: "필드" },
    review_modal_close: { en: "Close", ko: "닫기" },

    // Status hero — per-state eyebrow / title / sub copy. Mirrors the
    // `heroContent` table in child-application-status.html.
    status_hero_pending_eyebrow: { en: "Submitted", ko: "제출됨" },
    status_hero_pending_title: {
        en: "Under review",
        ko: "심사 대기 중입니다",
    },
    status_hero_pending_sub: {
        en: "The parent team is reviewing your application. It usually takes 3–5 business days. You'll be notified by Ratel + email.",
        ko: "담당자가 신청서를 검토하고 있습니다. 보통 영업일 기준 3–5일이 소요됩니다. 결과는 Ratel 알림 + 이메일로 안내됩니다.",
    },
    status_hero_returned_eyebrow: { en: "Returned", ko: "수정 반송됨" },
    status_hero_returned_title: {
        en: "Revision requested",
        ko: "수정 요청이 도착했습니다",
    },
    status_hero_returned_sub: {
        en: "The parent team requested edits. Review the comment below, then edit and resubmit. There is no cooldown.",
        ko: "담당자가 신청서 수정을 요청했습니다. 아래 코멘트를 확인하고 수정 후 재제출해주세요. 쿨다운은 없습니다.",
    },
    status_hero_approved_eyebrow: { en: "Approved", ko: "승인됨" },
    status_hero_approved_title: {
        en: "🎉 Recognized as an official sub-team",
        ko: "🎉 정식 하위팀으로 등록되었습니다",
    },
    status_hero_approved_sub: {
        en: "Your team is now a recognized sub-team. You'll receive parent-team announcements and your activity will roll up to the parent dashboard.",
        ko: "이제 인증된 하위팀이 되었습니다. 상위팀의 공지를 받고, 활동이 상위팀 대시보드에 집계됩니다.",
    },
    status_hero_rejected_eyebrow: { en: "Rejected", ko: "거절됨" },
    status_hero_rejected_title: {
        en: "Your application was not approved",
        ko: "안타깝게도 이번 신청은 거절되었습니다",
    },
    status_hero_rejected_sub: {
        en: "Please review the reason below. You can keep operating as a standalone team and reapply to the same or a different parent team without a cooldown.",
        ko: "아래 사유를 확인해주세요. 독립 팀으로 계속 운영할 수 있고, 쿨다운 없이 같은 또는 다른 상위팀에 재신청할 수 있습니다.",
    },

    // Status feedback card — per-state heading + author meta.
    status_feedback_returned_heading: {
        en: "Revision comment",
        ko: "수정 요청 코멘트",
    },
    status_feedback_approved_heading: {
        en: "Welcome message",
        ko: "환영 메시지",
    },
    status_feedback_rejected_heading: {
        en: "Rejection reason",
        ko: "거절 사유",
    },
    status_feedback_author_suffix: {
        en: "Parent team",
        ko: "상위팀 담당자",
    },

    // Submitted-answers snapshot card.
    status_snapshot_heading: {
        en: "Submitted answers",
        ko: "제출 내역",
    },
    status_form_version_meta: {
        en: "Form snapshot",
        ko: "폼 스냅샷",
    },
    status_fields_count_suffix: {
        en: "fields",
        ko: "필드",
    },

    // Footer notice (form-version pinning).
    status_form_pin_title: { en: "Form version pinned", ko: "폼 버전 고정" },
    status_form_pin_text: {
        en: "This application is pinned to the form version at submission time. Future updates to the form do not retroactively apply.",
        ko: "제출 시점의 폼 버전에 이 신청은 고정되어 있습니다. 상위팀이 폼을 업데이트하더라도 이 신청에는 적용되지 않습니다.",
    },

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
    doc_compose_required_desc_short: {
        en: "Must agree on apply",
        ko: "신청 시 동의 필수",
    },
    doc_compose_reference_only: {
        en: "Reference only",
        ko: "참고 자료로만 노출",
    },
    doc_compose_required_desc: {
        en: "Sub-teams must read and agree to this document before submitting.",
        ko: "하위팀이 가입 신청할 때 이 문서를 읽고 동의해야 합니다.",
    },
    doc_compose_word_count: { en: "characters", ko: "글자" },
    doc_compose_autosaved: { en: "Auto-saved", ko: "자동 저장됨" },
    doc_compose_draft_unsaved: { en: "Draft", ko: "초안" },
    doc_compose_preview: { en: "Preview", ko: "미리보기" },
    doc_compose_banner_label: { en: "Document editor · Rich text + attachments", ko: "DOCUMENT EDITOR · RICH TEXT + 첨부" },
    doc_compose_banner_text: {
        en: "regulations / operating guides — composed with the Ratel rich editor. Mark a document as Required and sub-teams must read & agree before submitting their application.",
        ko: "규정·운영 가이드를 Ratel rich editor 로 작성합니다. 필독으로 지정하면 하위팀이 가입 신청 때 반드시 읽고 동의해야 제출됩니다.",
    },
    doc_compose_attachments_title: { en: "Attachments", ko: "첨부 파일 · ATTACHMENTS" },
    doc_compose_upload_title: { en: "Add file · Upload", ko: "파일 추가 · UPLOAD" },
    doc_compose_upload_hint: {
        en: "PDF · DOCX · Image (max 25 MB per file)",
        ko: "PDF · DOCX · 이미지 (최대 25 MB/파일)",
    },
    doc_compose_required_card_title: { en: "Required reading", ko: "가입 신청 필독" },
    doc_compose_info_title: { en: "Document info", ko: "문서 정보" },
    doc_compose_info_version: { en: "Version", ko: "VERSION" },
    doc_compose_info_updated: { en: "Last updated", ko: "LAST UPDATED" },
    doc_compose_info_editor: { en: "Editor", ko: "EDITOR" },
    doc_compose_info_attachments: { en: "Attachments", ko: "ATTACHMENTS" },
    doc_compose_info_required: { en: "Required", ko: "REQUIRED" },
    doc_compose_attach_none: { en: "No attachments", ko: "첨부 없음" },
    doc_compose_attach_files_unit: { en: "files", ko: "파일" },
    doc_compose_discard: { en: "Discard changes", ko: "변경 폐기 · DISCARD" },
    doc_compose_stats_words: { en: "words", ko: "단어" },
    doc_compose_stats_read: { en: "read", ko: "read" },
    doc_compose_stats_chars: { en: "chars", ko: "자" },
    yes: { en: "Yes", ko: "예" },
    no: { en: "No", ko: "아니오" },

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
        ko: "팀 프로필의 소속 표시가 제거됩니다.",
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
    leave_parent_status_chip: {
        en: "Action · Caution",
        ko: "주의 · 실행 액션",
    },
    leave_parent_eyebrow_code: {
        en: "Leave Parent Team",
        ko: "상위팀 이탈",
    },
    leave_parent_header_title_prefix: {
        en: "Leave ",
        ko: "",
    },
    leave_parent_header_title_suffix: {
        en: " and continue standalone",
        ko: "에서 독립합니다",
    },
    leave_parent_header_emphasis: { en: "Leave", ko: "독립" },
    leave_parent_header_sub: {
        en: "Drop the sub-team link without the parent's approval. You'll stop receiving parent broadcasts, your activity won't show on the parent dashboard, and the parent is notified.",
        ko: "부모팀의 승인 없이 sub-team 관계만 해제하고 독립 팀으로 전환합니다. 부모팀의 공지 방송, 활동 집계, 거버넌스에서 벗어납니다. 부모팀은 알림을 받습니다.",
    },
    leave_parent_tie_child_of: { en: "Child of", ko: "Child of" },
    leave_parent_tie_recognized_since: { en: "Recognized since", ko: "인증" },
    leave_parent_panel_sub: {
        en: "Only this team can run this action (team admin role required). The parent's approval isn't needed.",
        ko: "이 동작은 이 팀에서만 실행 가능합니다 (팀 admin 권한 필요). 부모팀의 승인은 필요하지 않습니다.",
    },

    // Bylaws page
    bylaws_page_eyebrow: { en: "Bylaws", ko: "운영 수칙" },
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
        ko: "공개된 운영 수칙이 없습니다",
    },
    bylaws_required_badge: {
        en: "Required",
        ko: "필독",
    },
    bylaws_status_chip: { en: "Public", ko: "공개" },
    bylaws_add_team: { en: "Add bylaw", ko: "운영 수칙 항목 추가" },
    bylaws_add_club: { en: "Add club rule", ko: "회칙 항목 추가" },
    bylaws_page_title: {
        en: "Bylaws — publish your rules so every member follows the same playbook.",
        ko: "운영 수칙을 공개 게시하여 모두가 같은 규칙을 따르게 합니다.",
    },
    bylaws_page_sub: {
        en: "Bylaws behave like regular posts but are grouped in a dedicated section on the team page. Sub-teams can review the parent's bylaws side-by-side.",
        ko: "Bylaws는 일반 게시물과 동일하게 동작하지만, 팀 페이지에서 전용 섹션으로 묶여 보입니다. 자식 팀은 부모 팀의 Bylaws를 바로 확인할 수 있습니다.",
    },
    bylaws_parent_link_eyebrow: {
        en: "Parent team bylaws",
        ko: "상위팀 운영 수칙",
    },
    bylaws_parent_link_meta: {
        en: "View the parent team's bylaws",
        ko: "상위팀이 관리하는 운영 수칙 보기",
    },
    bylaws_view_parent: { en: "Parent view", ko: "PARENT VIEW" },
    bylaws_view_sub: { en: "Sub-team view", ko: "SUB-TEAM VIEW" },
    bylaws_page_eyebrow_fr: { en: "FR-2 · Bylaws", ko: "FR-2 · BYLAWS" },
    // Title is split into a `_strong` lead and a `_rest` tail so the
    // template can render `<strong>{lead}</strong>{rest}` without
    // duplicating the noun (Korean has 운영 수칙 at the front, English keeps
    // it as a single highlighted word).
    bylaws_page_title_strong: { en: "Bylaws", ko: "운영 수칙" },
    bylaws_page_title_rest: {
        en: " — publish your rules so every member follows the same playbook.",
        ko: "을 공개 게시하여 모두가 같은 규칙을 따르게 합니다.",
    },
    bylaws_section_team_regulations: {
        en: "Team regulations · BYLAWS",
        ko: "팀 운영 수칙 · BYLAWS",
    },
    bylaws_section_club_rules: {
        en: "Club rules · CLUB BYLAWS",
        ko: "동아리 회칙 · CLUB BYLAWS",
    },
    bylaws_section_count_items: { en: "{n} items", ko: "{n}개 항목" },
    bylaws_card_bylaw_num: { en: "BYLAW {n}", ko: "BYLAW {n}" },
    bylaws_card_rule_num: { en: "RULE {n}", ko: "RULE {n}" },
    bylaws_required_meta: { en: "Required", ko: "필독" },
    // Documents tab → "View bylaws" entry point
    docs_view_bylaws: { en: "View bylaws", ko: "운영 수칙 자세히 보기" },
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
