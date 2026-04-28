use crate::features::spaces::pages::apps::apps::analyzes::*;

translate! {
    SpaceAnalyzesAppTranslate;

    page_title: {
        en: "Poll Analyze",
        ko: "설문 분석",
    },
    poll_section_title: {
        en: "Poll Analyze",
        ko: "설문 분석",
    },
    discussion_section_title: {
        en: "Discussion Analyze",
        ko: "토론 분석",
    },
    sample_survey: {
        en: "Sample Survey",
        ko: "사전 조사",
    },
    final_survey: {
        en: "Final Survey",
        ko: "최종 설문",
    },
    questions: {
        en: "questions",
        ko: "문항",
    },
    view_analyze: {
        en: "View Analyze",
        ko: "분석 보기",
    },
    more: {
        en: "More",
        ko: "더보기",
    },
    no_polls: {
        en: "No polls",
        ko: "설문이 없습니다",
    },
    no_discussions: {
        en: "No discussions",
        ko: "토론이 없습니다",
    },
    to_be_continue: {
        en: "To Be Continue",
        ko: "To Be Continue",
    },
    untitled_discussion: {
        en: "Untitled Discussion",
        ko: "제목 없는 토론",
    },
    back_to_list: {
        en: "Back to Polls",
        ko: "설문 목록으로",
    },
    download_excel: {
        en: "Download Excel",
        ko: "엑셀 다운로드",
    },
    download_started: {
        en: "Download started",
        ko: "다운로드를 시작했습니다",
    },
    responses_count: {
        en: "Responses",
        ko: "참여자",
    },
    remaining_days: {
        en: "Remaining",
        ko: "남은 일시",
    },
    survey_period: {
        en: "Survey Period",
        ko: "설문 기간",
    },
    filter_label: {
        en: "Filter",
        ko: "필터",
    },
    filter_group_label: {
        en: "Filter Category",
        ko: "필터 항목",
    },
    filter_value_label: {
        en: "Filter Value",
        ko: "필터 값",
    },
    filter_all: {
        en: "All",
        ko: "전체",
    },
    filter_gender: {
        en: "Gender",
        ko: "성별",
    },
    filter_age: {
        en: "Age",
        ko: "연령",
    },
    filter_school: {
        en: "School",
        ko: "학교",
    },
    gender_male: {
        en: "Male",
        ko: "남성",
    },
    gender_female: {
        en: "Female",
        ko: "여성",
    },
    gender_unknown: {
        en: "Unknown",
        ko: "알 수 없음",
    },
    total_questions: {
        en: "Questions",
        ko: "문항 수",
    },
    total_response_count_unit: {
        en: "responses",
        ko: "명 응답",
    },
    no_text_responses: {
        en: "No text responses",
        ko: "주관식 응답이 없습니다",
    },
    other_label: {
        en: "Other",
        ko: "기타",
    },
    id: {
        en: "ID",
        ko: "ID",
    },
    attribute: {
        en: "Attribute",
        ko: "속성",
    },
    category: {
        en: "Category",
        ko: "조사구분",
    },
    type_: {
        en: "Type",
        ko: "유형",
    },
    questionnaire: {
        en: "Questionnaire",
        ko: "질문지",
    },
    question: {
        en: "Question",
        ko: "질문",
    },
    answer: {
        en: "Answer",
        ko: "답변",
    },
    university: {
        en: "University",
        ko: "학교",
    },

    // ── Arena LIST view (Phase 1) ──────────────────────────
    arena_breadcrumb_apps: {
        en: "Apps",
        ko: "Apps",
    },
    arena_breadcrumb_current: {
        en: "Analyze",
        ko: "Analyze",
    },
    arena_topbar_title: {
        en: "Analyze Results",
        ko: "분석 결과",
    },
    list_heading: {
        en: "Result Analysis",
        ko: "결과 분석",
    },
    list_count_unit: {
        en: "analyses",
        ko: "개 분석",
    },
    list_hint: {
        en: "Use the arrows to flip through analyses, click a card to open it. The first \"+\" card creates a new analysis.",
        ko: "좌우 화살표로 분석을 넘기고, 카드를 클릭해 열어보세요. 첫 카드의 \"+\"는 새 분석 만들기.",
    },
    new_analysis_title: {
        en: "Create new analysis",
        ko: "새 분석 만들기",
    },
    new_analysis_desc: {
        en: "Pick cross filters to build a new report",
        ko: "교차 필터를 선택해 새 보고서를 만듭니다",
    },
    status_finish: {
        en: "Analysis complete",
        ko: "분석 완료",
    },
    status_in_progress: {
        en: "Running",
        ko: "분석 중",
    },
    status_failed: {
        en: "Failed",
        ko: "실패",
    },
    chips_empty: {
        en: "No filters · all data",
        ko: "필터 없음 · 전체 데이터",
    },
    list_card_pending_toast: {
        en: "Analysis is still running. Please try again once it's complete.",
        ko: "분석이 진행 중입니다. 완료된 뒤 다시 시도해주세요.",
    },
    arrow_prev_label: {
        en: "Previous card",
        ko: "이전 카드",
    },
    arrow_next_label: {
        en: "Next card",
        ko: "다음 카드",
    },
    dot_new_label: {
        en: "Create new analysis",
        ko: "새 분석 만들기",
    },
    dot_report_label_prefix: {
        en: "Report",
        ko: "보고서",
    },

    // ── Arena DETAIL view (Phase 3) ──────────────────────
    detail_sidebar_label: {
        en: "Analyzes",
        ko: "Analyzes",
    },
    detail_active_filters_label: {
        en: "Selected cross filters",
        ko: "선택된 교차 필터",
    },
    detail_active_filters_empty: {
        en: "No filters — all data",
        ko: "필터 없음 — 전체 데이터",
    },
    detail_meta_filter_count: {
        en: "filters",
        ko: "필터",
    },
    detail_meta_filter_count_unit: {
        en: "items",
        ko: "개",
    },
    detail_meta_created_prefix: {
        en: "created",
        ko: "생성",
    },
    detail_panel_chip_poll: {
        en: "Poll",
        ko: "Poll",
    },
    detail_panel_chip_quiz: {
        en: "Quiz",
        ko: "Quiz",
    },
    detail_panel_chip_discussion: {
        en: "Discussion",
        ko: "Discussion",
    },
    detail_panel_chip_follow: {
        en: "Follow",
        ko: "Follow",
    },
    detail_group_poll: {
        en: "Poll",
        ko: "Poll",
    },
    detail_group_quiz: {
        en: "Quiz",
        ko: "Quiz",
    },
    detail_group_discussion: {
        en: "Discussion",
        ko: "Discussion",
    },
    detail_group_follow: {
        en: "Follow",
        ko: "Follow",
    },
    detail_filter_all: {
        en: "All",
        ko: "전체",
    },
    detail_filter_gender: {
        en: "Gender",
        ko: "성별",
    },
    detail_filter_age: {
        en: "Age",
        ko: "나이",
    },
    detail_filter_school: {
        en: "School",
        ko: "학교",
    },
    detail_download_btn: {
        en: "Download responses",
        ko: "응답 다운로드",
    },
    detail_download_poll_testid_label: {
        en: "Download poll responses",
        ko: "설문 응답 다운로드",
    },
    detail_responses_unit: {
        en: "responses",
        ko: "응답",
    },
    detail_attempts_unit: {
        en: "attempts",
        ko: "응시",
    },
    detail_correct_rate_prefix: {
        en: "Correct rate",
        ko: "정답률",
    },
    detail_correct_label: {
        en: "Correct",
        ko: "정답",
    },
    detail_card_hint_poll: {
        en: "Click an option to narrow other panels by that respondent set · multi-select",
        ko: "옵션을 클릭하면 해당 응답자들로 다른 패널이 좁혀집니다 · 복수 선택 가능",
    },
    detail_card_hint_quiz: {
        en: "Click an answer to narrow other panels by those test-takers · multi-select",
        ko: "답을 클릭하면 그 답을 고른 응시자들로 다른 패널이 좁혀집니다 · 복수 선택 가능",
    },
    detail_poll_card1_title: {
        en: "Which area of constitutional reform is the most urgent?",
        ko: "가장 시급하게 추진해야 할 헌법 개정 분야는?",
    },
    detail_poll_card2_title: {
        en: "What value should be most central to constitutional reform?",
        ko: "개헌 시 가장 중요하게 다뤄져야 할 가치는?",
    },
    detail_poll_card3_title: {
        en: "Free-text: things to consider during reform",
        ko: "개헌 추진 과정에서 반드시 고려해야 할 점을 자유롭게 작성해주세요",
    },
    detail_poll_title: {
        en: "Do you think the Korean Constitution needs to be amended?",
        ko: "귀하는 현재 대한민국 헌법을 개정하는 것이 필요하다고 생각하십니까?",
    },
    detail_quiz_title: {
        en: "Constitutional Basics Quiz",
        ko: "헌법 기본 상식 퀴즈",
    },
    detail_quiz_card1_title: {
        en: "How many justices are on the Constitutional Court?",
        ko: "헌법재판소의 재판관 수는?",
    },
    detail_quiz_card2_title: {
        en: "Where does legislative power reside in Korea?",
        ko: "대한민국의 입법권은 어디에 있는가?",
    },
    detail_quiz_card3_title: {
        en: "What does Article 1 of the Constitution say?",
        ko: "헌법 제1조의 내용은?",
    },
    detail_discussion_title: {
        en: "What do you think about the proposed non-consent rape law?",
        ko: "비동의 강간죄 도입에 대해서 어떻게 생각하십니까?",
    },
    detail_discussion_settings_title: {
        en: "Analysis settings",
        ko: "분석 설정",
    },
    detail_discussion_topic_modeling_label: {
        en: "Topic Modeling",
        ko: "Topic Modeling",
    },
    detail_discussion_lda_label: {
        en: "LDA topic count",
        ko: "LDA 토픽 개수",
    },
    detail_discussion_lda_hint: {
        en: "Enter a value between 1 and 20.",
        ko: "1~20 사이로 입력해주세요.",
    },
    detail_discussion_tfidf_label: {
        en: "TF-IDF keyword count",
        ko: "TF-IDF 키워드 개수",
    },
    detail_discussion_network_label: {
        en: "Top network nodes",
        ko: "네트워크 상위 노드 개수",
    },
    detail_discussion_network_hint: {
        en: "Enter a value between 1 and 30.",
        ko: "1~30 사이로 입력해주세요.",
    },
    detail_discussion_excluded_label: {
        en: "Excluded topics",
        ko: "제외된 토픽",
    },
    detail_discussion_excluded_placeholder: {
        en: "e.g. it, that",
        ko: "예: 이거, 그거",
    },
    detail_discussion_excluded_hint: {
        en: "Comma-separated.",
        ko: "쉼표(,)로 구분해 입력해주세요.",
    },
    detail_discussion_btn_reset: {
        en: "Reset",
        ko: "초기화",
    },
    detail_discussion_btn_apply: {
        en: "Apply",
        ko: "확인",
    },
    detail_tfidf_card_title: {
        en: "TF-IDF keywords",
        ko: "TF-IDF 키워드",
    },
    detail_tfidf_card_count: {
        en: "Score",
        ko: "Score",
    },
    detail_lda_card_title: {
        en: "LDA topic modeling",
        ko: "LDA 토픽 모델링",
    },
    detail_lda_edit_label: {
        en: "Edit topic labels",
        ko: "토픽 라벨 편집",
    },
    detail_lda_col_topic: {
        en: "Topic",
        ko: "토픽",
    },
    detail_lda_col_keywords: {
        en: "Keywords",
        ko: "키워드",
    },
    detail_lda_col_filter: {
        en: "Filter",
        ko: "필터",
    },
    detail_lda_filter_aria_prefix: {
        en: "Toggle filter for",
        ko: "필터 토글:",
    },
    detail_network_card_title: {
        en: "Text network",
        ko: "텍스트 네트워크",
    },
    detail_network_card_count: {
        en: "Co-occurrence",
        ko: "Co-occurrence",
    },
    detail_network_aria: {
        en: "Text network",
        ko: "텍스트 네트워크",
    },
    detail_follow_title: {
        en: "Legal Expert Follow Campaign",
        ko: "법률 전문가 팔로우 캠페인",
    },
    detail_follow_card_title: {
        en: "Completion rate by follow target",
        ko: "팔로우 대상별 수행률",
    },
    detail_follow_count_text: {
        en: "42 participants · 12 targets",
        ko: "42명 참여 · 12 타겟",
    },
    detail_follow_legend_done: {
        en: "Completed",
        ko: "수행함",
    },
    detail_follow_legend_miss: {
        en: "Not completed",
        ko: "미수행",
    },
    detail_back_btn_aria: {
        en: "Back",
        ko: "뒤로",
    },
    detail_sb_item_meta_questions: {
        en: "questions",
        ko: "문항",
    },
    detail_sb_item_meta_responses: {
        en: "responses",
        ko: "응답",
    },
    detail_sb_item_meta_attempts: {
        en: "attempts",
        ko: "응시",
    },
    detail_sb_item_meta_comments: {
        en: "comments",
        ko: "댓글",
    },
    detail_sb_item_meta_participants: {
        en: "participants",
        ko: "참여",
    },
    detail_sb_item_meta_targets: {
        en: "targets",
        ko: "타겟",
    },
    detail_sb_item_meta_options: {
        en: "options",
        ko: "선지",
    },
    detail_sb_item_meta_correct: {
        en: "correct",
        ko: "정답",
    },
    detail_sb_item_meta_followers: {
        en: "followers",
        ko: "팔로워",
    },
    detail_panel_empty_title: {
        en: "No data yet",
        ko: "데이터 없음",
    },
    detail_panel_empty_poll: {
        en: "Matched respondents haven't answered any poll questions yet.",
        ko: "응답자들이 답한 설문 문항이 아직 없습니다.",
    },
    detail_panel_empty_quiz: {
        en: "Matched respondents haven't attempted any quizzes yet.",
        ko: "응답자들이 응시한 퀴즈가 아직 없습니다.",
    },
    detail_panel_empty_follow: {
        en: "No follow targets are followed by the matched respondents yet.",
        ko: "응답자들이 팔로우한 타겟이 아직 없습니다.",
    },
    detail_panel_empty_text_answers: {
        en: "No text responses recorded.",
        ko: "수집된 텍스트 응답이 없습니다.",
    },
    detail_discussion_pick_title: {
        en: "Pick a discussion",
        ko: "토론을 선택하세요",
    },
    detail_discussion_pick_hint: {
        en: "Choose a discussion in the sidebar to configure and run the text analysis pipeline.",
        ko: "왼쪽 사이드바에서 토론을 선택하면 분석 설정을 진행할 수 있습니다.",
    },
    detail_discussion_no_run_title: {
        en: "Not analysed yet",
        ko: "아직 분석되지 않음",
    },
    detail_discussion_no_run_hint: {
        en: "Set the parameters above and press 확인 to run the LDA / TF-IDF / network analysis.",
        ko: "위에서 파라미터를 설정한 뒤 확인을 누르면 LDA / TF-IDF / 네트워크 분석이 실행됩니다.",
    },
    detail_discussion_running_title: {
        en: "Analysis in progress",
        ko: "분석 진행 중",
    },
    detail_discussion_running_hint: {
        en: "The analysis pipeline is running. This page will refresh once results are ready.",
        ko: "분석이 진행 중입니다. 완료되면 결과가 표시됩니다.",
    },
    detail_discussion_failed_title: {
        en: "Analysis failed",
        ko: "분석 실패",
    },
    detail_discussion_failed_hint: {
        en: "Something went wrong while running the analysis. Adjust the parameters and try again.",
        ko: "분석 도중 문제가 발생했습니다. 파라미터를 조정한 뒤 다시 시도해주세요.",
    },
    detail_tfidf_card_count_suffix: {
        en: " terms",
        ko: "개 단어",
    },
    detail_lda_card_count_suffix: {
        en: " topics",
        ko: "개 토픽",
    },
    detail_network_card_count_suffix: {
        en: " nodes",
        ko: "개 노드",
    },
    detail_network_card_edge_suffix: {
        en: " edges",
        ko: "개 엣지",
    },

    // ── Arena CREATE wizard (Phase 2) ──────────────────────
    create_topbar_title: {
        en: "New analysis",
        ko: "새 분석 만들기",
    },
    create_cf_title: {
        en: "Cross filter",
        ko: "교차 필터",
    },
    create_cf_hint: {
        en: "Pick the data conditions for this analysis. Leaving it empty analyses the whole dataset.",
        ko: "분석에 사용할 데이터 조건을 선택하세요. 비워두면 전체 데이터로 분석합니다.",
    },
    create_cf_chips_all: {
        en: "All",
        ko: "전체",
    },
    create_cf_add_filter: {
        en: "Add filter",
        ko: "필터 추가",
    },
    create_cf_pick_action_label: {
        en: "Which action's data should we add?",
        ko: "어떤 액션의 데이터를 추가할까요?",
    },
    create_cf_pick_action_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    create_cf_pick_item_label: {
        en: "Pick one item to analyze",
        ko: "분석할 항목을 하나 선택하세요",
    },
    create_cf_pick_item_label_with_type: {
        en: "— pick one item",
        ko: "— 분석할 항목 하나 선택",
    },
    create_cf_back_to_action: {
        en: "← Pick action again",
        ko: "← 액션 다시 선택",
    },
    create_cf_remove_filter_aria: {
        en: "Remove filter",
        ko: "필터 제거",
    },
    create_action_tile_poll: {
        en: "Poll",
        ko: "Poll",
    },
    create_action_tile_quiz: {
        en: "Quiz",
        ko: "Quiz",
    },
    create_action_tile_discussion: {
        en: "Discussion",
        ko: "Discussion",
    },
    create_action_tile_follow: {
        en: "Follow",
        ko: "Follow",
    },
    create_action_count_unit: {
        en: "items",
        ko: "개",
    },
    create_action_questions_unit: {
        en: "questions",
        ko: "문항",
    },
    create_sunji_follow_title: {
        en: "Follow targets",
        ko: "팔로우 타겟",
    },
    create_sunji_follow_empty: {
        en: "No follow targets registered",
        ko: "등록된 팔로우 타겟이 없습니다",
    },
    create_sunji_back: {
        en: "← Pick again",
        ko: "← 다시 선택",
    },
    create_sunji_default_title: {
        en: "Selected item",
        ko: "선택한 항목",
    },
    create_sunji_confirm: {
        en: "Confirm",
        ko: "확인",
    },
    create_sunji_empty: {
        en: "This item has no options to pick.",
        ko: "이 항목에는 선택할 선지가 없습니다.",
    },
    create_sunji_loading: {
        en: "Loading questions…",
        ko: "문항을 불러오는 중…",
    },
    create_sunji_correct_badge: {
        en: "Correct",
        ko: "정답",
    },
    create_keyword_block_title: {
        en: "Keyword input (comma-separated)",
        ko: "키워드 입력 (쉼표로 구분)",
    },
    create_keyword_input_placeholder: {
        en: "e.g. evidence, statement, victim",
        ko: "예: 증거, 진술, 피해자",
    },
    create_keyword_hint: {
        en: "Each keyword becomes its own filter.",
        ko: "각 키워드는 별도 필터로 추가됩니다.",
    },
    create_preview_title: {
        en: "Preview",
        ko: "미리보기",
    },
    create_preview_hint: {
        en: "The report below will be generated with these settings. Enter a name and press \"Generate report\".",
        ko: "아래 내용으로 보고서가 생성됩니다. 분석명을 입력하고 \"보고서 생성\"을 누르세요.",
    },
    create_preview_name_label: {
        en: "Analysis name",
        ko: "분석명",
    },
    create_preview_name_placeholder: {
        en: "e.g. Constitutional reform × false-accusation perception cross-analysis",
        ko: "예: 헌법 개정 의견과 무고죄 인식 교차분석",
    },
    create_preview_name_hint: {
        en: "Used as the result page heading and the list-card title.",
        ko: "결과 페이지와 목록 카드의 제목으로 사용됩니다.",
    },
    create_preview_chips_label: {
        en: "Selected cross filters",
        ko: "선택된 교차 필터",
    },
    create_preview_stat_respondents: {
        en: "Matching respondents",
        ko: "해당 응답자",
    },
    create_preview_stat_records: {
        en: "Matching records",
        ko: "해당되는 데이터 수",
    },
    create_preview_correct_tag: {
        en: "Correct",
        ko: "정답",
    },
    create_pager_prev: {
        en: "← Prev",
        ko: "← 이전",
    },
    create_pager_next: {
        en: "Next →",
        ko: "다음 →",
    },
    create_footer_step_label_create: {
        en: "1 / 2",
        ko: "1 / 2",
    },
    create_footer_step_label_preview: {
        en: "2 / 2",
        ko: "2 / 2",
    },
    create_footer_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    create_footer_next: {
        en: "Next →",
        ko: "다음 →",
    },
    create_footer_back: {
        en: "← Back",
        ko: "← 이전",
    },
    create_footer_confirm: {
        en: "Generate report",
        ko: "보고서 생성",
    },
}
