use crate::features::spaces::pages::actions::actions::quiz::*;

translate! {
    QuizCreatorTranslate;

    page_title: {
        en: "Quiz",
        ko: "퀴즈",
    },
    btn_edit: {
        en: "Edit",
        ko: "편집",
    },
    btn_save: {
        en: "Save",
        ko: "저장",
    },
    saving: {
        en: "Saving...",
        ko: "저장 중...",
    },
    all_changes_saved: {
        en: "All changes saved",
        ko: "모든 변경사항 저장됨",
    },
    unsaved_changes: {
        en: "Unsaved changes",
        ko: "저장되지 않은 변경사항",
    },
    btn_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    btn_back: {
        en: "Back",
        ko: "뒤로",
    },
    btn_next: {
        en: "Next",
        ko: "다음",
    },
    btn_done: {
        en: "Done",
        ko: "완료",
    },
    no_questions: {
        en: "No questions added yet.",
        ko: "아직 질문이 없습니다.",
    },
    title_label: {
        en: "Title",
        ko: "제목",
    },
    title_placeholder: {
        en: "Enter quiz title...",
        ko: "퀴즈 제목을 입력하세요...",
    },
    description_label: {
        en: "Description",
        ko: "설명",
    },
    description_placeholder: {
        en: "Enter quiz description...",
        ko: "퀴즈 설명을 입력하세요...",
    },
    retry_label: {
        en: "Retry Count",
        ko: "재시도 횟수",
    },
    survey_time_label: {
        en: "Survey Time",
        ko: "설문 시간",
    },
    retry_placeholder: {
        en: "Enter retry count...",
        ko: "재시도 횟수를 입력하세요...",
    },
    pass_score_label: {
        en: "Pass Score",
        ko: "통과 기준 점수",
    },
    pass_score_placeholder: {
        en: "Enter pass score...",
        ko: "통과 기준 점수를 입력하세요...",
    },
    overview_title: {
        en: "Overview",
        ko: "개요",
    },
    overview_description: {
        en: "Configure the quiz title and introduction shown before participants start.",
        ko: "참여 전에 보이는 퀴즈 제목과 소개를 설정합니다.",
    },
    upload_title: {
        en: "Upload",
        ko: "업로드",
    },
    upload_description: {
        en: "Upload quiz reference files for participants.",
        ko: "참여자가 참고할 퀴즈 파일을 업로드합니다.",
    },
    upload_placeholder: {
        en: "Upload section UI will be added here.",
        ko: "업로드 섹션 UI가 여기에 추가됩니다.",
    },
    upload_drop_title: {
        en: "Drag & Drop files here",
        ko: "여기에 파일을 드래그 앤 드롭하세요",
    },
    upload_cta: {
        en: "Upload",
        ko: "업로드",
    },
    upload_supported_types: {
        en: "PDF, DOCX, PPTX, XLSX, PNG, JPG, MP4, MOV 100MB",
        ko: "PDF, DOCX, PPTX, XLSX, PNG, JPG, MP4, MOV 100MB",
    },
    upload_empty: {
        en: "No uploaded files yet.",
        ko: "아직 업로드된 파일이 없습니다.",
    },
    upload_view: {
        en: "View",
        ko: "보기",
    },
    upload_delete: {
        en: "Delete",
        ko: "삭제",
    },
    quiz_section_title: {
        en: "Quiz",
        ko: "퀴즈",
    },
    quiz_section_description: {
        en: "Create and manage quiz questions.",
        ko: "퀴즈 문항을 만들고 관리합니다.",
    },
    setting_section_title: {
        en: "Setting",
        ko: "설정",
    },
    setting_section_description: {
        en: "Manage schedule and scoring rules.",
        ko: "일정과 점수 규칙을 관리합니다.",
    },

    // ── Card UI ─────────────────────────────────────────────
    type_badge_label: {
        en: "Quiz",
        ko: "퀴즈",
    },
    footer_status: {
        en: "All changes saved",
        ko: "모든 변경사항 저장됨",
    },
    autosave_saving: {
        en: "Saving...",
        ko: "저장 중...",
    },
    autosave_saved: {
        en: "Saved",
        ko: "저장됨",
    },
    autosave_unsaved: {
        en: "Unsaved",
        ko: "저장 안됨",
    },
    card_content_title: {
        en: "Content",
        ko: "내용",
    },
    card_content_subtitle: {
        en: "Description, tags, and scoring rules",
        ko: "설명, 태그, 채점 규칙",
    },
    card_questions_title: {
        en: "Questions",
        ko: "질문",
    },
    card_questions_subtitle: {
        en: "Add, reorder, and configure quiz questions",
        ko: "퀴즈 질문 추가, 정렬, 구성",
    },
    card_config_title: {
        en: "Configuration",
        ko: "설정",
    },
    card_config_subtitle: {
        en: "Schedule, rewards, and danger zone",
        ko: "일정, 보상, 위험 영역",
    },
    section_content_label: {
        en: "Content",
        ko: "내용",
    },
    section_content_hint: {
        en: "Markdown supported",
        ko: "Markdown 지원",
    },
    section_scoring_label: {
        en: "Scoring",
        ko: "채점",
    },
    section_attachments_label: {
        en: "Attachments",
        ko: "첨부 파일",
    },
    section_attachments_hint: {
        en: "Reference materials for participants",
        ko: "참가자에게 보여줄 참고 자료",
    },
    dropzone_title: {
        en: "Drop files or click to browse",
        ko: "파일을 드래그하거나 클릭하여 찾기",
    },
    dropzone_sub: {
        en: "PDF, PNG, JPG \u{00B7} max 25MB each",
        ko: "PDF, PNG, JPG \u{00B7} 각 25MB까지",
    },
    remove_file: {
        en: "Remove file",
        ko: "파일 삭제",
    },
    section_questions_label: {
        en: "Questions",
        ko: "질문",
    },
    section_questions_hint: {
        en: "Min 3 · Max 20",
        ko: "최소 3 · 최대 20",
    },
    section_schedule_label: {
        en: "Schedule",
        ko: "일정",
    },
    section_participation_label: {
        en: "Participation & Rewards",
        ko: "참여 및 보상",
    },
    section_danger_label: {
        en: "Danger zone",
        ko: "위험 영역",
    },
    retries_suffix: {
        en: "retries",
        ko: "회 재시도",
    },
    retry_count_label: {
        en: "Retry count",
        ko: "재시도 횟수",
    },
    questions_suffix: {
        en: "questions",
        ko: "문항",
    },
    qtype_single: {
        en: "Single",
        ko: "단일",
    },
    qtype_multi: {
        en: "Multi",
        ko: "다중",
    },
    remove_question: {
        en: "Remove question",
        ko: "질문 삭제",
    },
    add_question: {
        en: "Add question",
        ko: "질문 추가",
    },
    schedule_starts_at: {
        en: "Starts at",
        ko: "시작",
    },
    schedule_ends_at: {
        en: "Ends at",
        ko: "종료",
    },
    tile_reward: {
        en: "Reward (CR)",
        ko: "보상 (CR)",
    },
    tile_prereq: {
        en: "Prerequisite",
        ko: "선행 조건",
    },
    tile_prereq_label: {
        en: "Require Follow action first",
        ko: "Follow 액션 먼저 필요",
    },
    delete_quiz_title: {
        en: "Delete this quiz",
        ko: "퀴즈 삭제",
    },
    delete_quiz_desc: {
        en: "Removes the quiz and all submissions. Rewards already paid are not refunded.",
        ko: "퀴즈와 모든 제출물을 삭제합니다. 이미 지급된 보상은 환불되지 않습니다.",
    },
    delete_quiz_btn: {
        en: "Delete quiz",
        ko: "퀴즈 삭제",
    },
    next_questions: {
        en: "Next: Questions",
        ko: "다음: 질문",
    },
    next_configuration: {
        en: "Next: Configuration",
        ko: "다음: 설정",
    },
    back_questions: {
        en: "Back: Questions",
        ko: "이전: 질문",
    },
    card_index_1: {
        en: "01 / 03",
        ko: "01 / 03",
    },
    card_index_2: {
        en: "02 / 03",
        ko: "02 / 03",
    },
    card_index_3: {
        en: "03 / 03",
        ko: "03 / 03",
    },
}
