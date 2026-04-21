use crate::common::*;

translate! {
    EssenceSourcesTranslate;

    seo_title: { en: "Ratel — Essence Sources", ko: "Ratel — Essence 소스" },

    // Topbar
    back_label: { en: "Back", ko: "뒤로" },
    topbar_eyebrow: { en: "Essence", ko: "Essence" },
    topbar_main: { en: "Sources", ko: "소스" },
    add_source: { en: "Add source", ko: "소스 추가" },

    // Hero
    hero_eyebrow: { en: "Your Essence", ko: "내 Essence" },
    hero_subtitle: {
        en: "Everything feeding your Essence House. Toggle to include/exclude from the public inference layer.",
        ko: "Essence House에 들어가는 모든 소스입니다. 공개 추론 레이어에 포함할지 토글로 설정하세요.",
    },
    hero_sources_word: { en: "sources", ko: "개 소스" },
    hero_chunks_word: { en: "chunks", ko: "청크" },
    hero_tokens_word: { en: "tokens", ko: "토큰" },
    hero_cta_open_house: { en: "Open my House", ko: "내 House 열기" },

    // Breakdown cards
    kind_all: { en: "All", ko: "전체" },
    kind_notion: { en: "Notion", ko: "Notion" },
    kind_ratel_posts: { en: "Ratel posts", ko: "Ratel 포스트" },
    kind_comments: { en: "Comments", ko: "댓글" },
    kind_actions: { en: "Actions", ko: "액션" },

    // Controls
    search_placeholder: {
        en: "Search sources by title, content, or tag…",
        ko: "제목, 내용, 태그로 소스 검색…",
    },
    filter_all: { en: "All", ko: "전체" },
    filter_active: { en: "Active", ko: "활성" },
    filter_paused: { en: "Paused", ko: "일시중지" },
    filter_ai_flagged: { en: "AI flagged", ko: "AI 표시됨" },
    sort_last_synced: { en: "Last synced ↓", ko: "최근 동기화 ↓" },
    sort_last_edited: { en: "Last edited ↓", ko: "최근 편집 ↓" },
    sort_word_count: { en: "Word count ↓", ko: "단어 수 ↓" },
    sort_quality: { en: "Quality ↓", ko: "품질 ↓" },
    sort_title: { en: "Title A–Z", ko: "제목순" },

    // Bulk bar
    bulk_selected_suffix: { en: "selected", ko: "개 선택됨" },
    bulk_pause: { en: "Pause", ko: "일시중지" },
    bulk_reembed: { en: "Re-embed", ko: "재임베딩" },
    bulk_flag_ai: { en: "Flag as AI", ko: "AI로 표시" },
    bulk_remove: { en: "Remove", ko: "제거" },

    // Table
    col_title: { en: "Title", ko: "제목" },
    col_words: { en: "Words", ko: "단어" },
    col_last_sync: { en: "Last sync", ko: "최근 동기화" },
    col_quality: { en: "Quality", ko: "품질" },
    col_in_house: { en: "In House", ko: "House" },
    row_select_label: { en: "Select", ko: "선택" },
    row_more_label: { en: "More", ko: "더보기" },
    row_in_house_label: { en: "In House", ko: "House" },
    row_badge_paused: { en: "Paused", ko: "일시중지됨" },
    row_badge_ai_flagged: { en: "AI flagged", ko: "AI 표시됨" },

    // Pagination
    pagination_prefix: { en: "Showing", ko: "" },
    pagination_of: { en: "of", ko: "/ 총" },
    pagination_previous: { en: "Previous", ko: "이전" },
    pagination_next: { en: "Next", ko: "다음" },

    // Empty state
    empty_title: { en: "No sources match your filters", ko: "조건에 맞는 소스가 없습니다" },
    empty_subtitle: {
        en: "Try clearing the search box or a different filter.",
        ko: "검색어나 필터를 변경해 보세요.",
    },
}
