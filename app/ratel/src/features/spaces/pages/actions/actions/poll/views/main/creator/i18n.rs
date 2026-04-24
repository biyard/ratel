use crate::features::spaces::pages::actions::actions::poll::*;

translate! {
    PollCreatorTranslate;

    type_badge_label: {
        en: "Poll",
        ko: "투표",
    },
    footer_status: {
        en: "All changes saved",
        ko: "모든 변경사항 저장됨",
    },
    autosave_saving: { en: "Saving...", ko: "저장 중..." },
    autosave_saved: { en: "Saved", ko: "저장됨" },
    autosave_unsaved: { en: "Unsaved", ko: "저장 안됨" },
    card_content_title: {
        en: "Content",
        ko: "내용",
    },
    card_content_subtitle: {
        en: "Title and questions",
        ko: "제목 및 질문",
    },
    card_config_title: {
        en: "Configuration",
        ko: "설정",
    },
    card_config_subtitle: {
        en: "Schedule, rewards, voting rules, and danger zone",
        ko: "일정, 보상, 투표 규칙, 위험 영역",
    },
    section_content_label: {
        en: "Content",
        ko: "내용",
    },
    section_content_hint: {
        en: "Markdown supported",
        ko: "Markdown 지원",
    },
    section_questions_label: {
        en: "Questions",
        ko: "질문",
    },
    section_questions_hint: {
        en: "Min 1 · Max 20",
        ko: "최소 1 · 최대 20",
    },
    section_schedule_label: {
        en: "Schedule",
        ko: "일정",
    },
    section_voting_rules_label: {
        en: "Voting rules",
        ko: "투표 규칙",
    },
    section_participation_label: {
        en: "Participation & Rewards",
        ko: "참여 및 보상",
    },
    section_dependencies_label: {
        en: "Dependency Actions",
        ko: "선행 액션",
    },
    section_status_label: {
        en: "Status",
        ko: "상태",
    },
    section_danger_label: {
        en: "Danger zone",
        ko: "위험 영역",
    },
    title_label: {
        en: "Title",
        ko: "제목",
    },
    title_placeholder: {
        en: "Enter poll title...",
        ko: "투표 제목을 입력하세요...",
    },
    qtype_single: {
        en: "Single",
        ko: "단일",
    },
    qtype_multi: {
        en: "Multi",
        ko: "다중",
    },
    qtype_subjective: {
        en: "Subjective",
        ko: "주관식",
    },
    qtype_linear: {
        en: "Linear",
        ko: "선형",
    },
    remove_question: {
        en: "Remove question",
        ko: "질문 삭제",
    },
    add_question: {
        en: "Add question",
        ko: "질문 추가",
    },
    add_option: {
        en: "Add option",
        ko: "선지 추가",
    },
    remove_option: {
        en: "Remove option",
        ko: "선지 삭제",
    },
    option_placeholder: {
        en: "Option text",
        ko: "선지 내용",
    },
    allow_other: {
        en: "Allow \"Other\" option",
        ko: "기타 옵션 허용",
    },
    subjective_hint: {
        en: "Response hint · shown to voters",
        ko: "응답 힌트 · 투표자에게 표시",
    },
    subjective_placeholder: {
        en: "Optional prompt shown above the response box...",
        ko: "응답 상자 위에 표시되는 선택적 프롬프트...",
    },
    linear_min_label: {
        en: "Min",
        ko: "최소",
    },
    linear_max_label: {
        en: "Max",
        ko: "최대",
    },
    schedule_starts_at: {
        en: "Starts at",
        ko: "시작",
    },
    schedule_ends_at: {
        en: "Ends at",
        ko: "종료",
    },
    voting_response_editable_label: {
        en: "Allow response editing",
        ko: "응답 수정 허용",
    },
    voting_response_editable_sub: {
        en: "Participants can update their answers while the poll is open",
        ko: "투표 진행 중 참여자가 응답을 수정할 수 있음",
    },
    voting_encrypted_label: {
        en: "Encrypted upload",
        ko: "암호화 업로드",
    },
    voting_encrypted_sub: {
        en: "Encrypt vote results and store on-chain. Responses cannot be edited after submission.",
        ko: "투표 결과를 암호화하여 온체인 저장. 제출 후 응답 수정 불가",
    },
    tile_reward: {
        en: "Reward (CR)",
        ko: "보상 (CR)",
    },
    delete_poll_title: {
        en: "Delete this poll",
        ko: "투표 삭제",
    },
    delete_poll_desc: {
        en: "Removes the poll and all submissions. Rewards already paid are not refunded.",
        ko: "투표와 모든 제출물을 삭제합니다. 이미 지급된 보상은 환불되지 않습니다.",
    },
    delete_poll_btn: {
        en: "Delete poll",
        ko: "투표 삭제",
    },
    card_index_1: {
        en: "01 / 02",
        ko: "01 / 02",
    },
    card_index_2: {
        en: "02 / 02",
        ko: "02 / 02",
    },
}
